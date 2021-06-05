use hyper::header::{HeaderMap, HeaderName, CONTENT_TYPE};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use reqwest::Client;
use std::net::SocketAddr;
use url::form_urlencoded;

use crate::badge::BadgeStyle;
use crate::graphics::{convert_svg_to_png, INVALID_SVG_BADGE};

pub type GenericServerError = Box<dyn std::error::Error + Send + Sync>;

const HEALTH_CHECK_BODY: &str = concat!(
    r#"{"status":"pass","version":""#,
    env!("CARGO_PKG_VERSION"),
    "\"}",
);
const FORWARDING_REQUEST_HEADERS: &[&str] =
    &["if-modified-since", "if-unmodified-since", "if-none-match"];
const FORWARDING_RESPONSE_HEADERS: &[&str] = &["date", "cache-control", "expires", "last-modified"];

fn get_badge_style(req: &Request<Body>) -> BadgeStyle {
    req.uri().query().map_or(BadgeStyle::Unspecified, |q| {
        form_urlencoded::parse(q.as_bytes())
            .find_map(|(p, v)| {
                if p != "style" {
                    return None;
                }
                match &*v.to_lowercase() {
                    "flat" => Some(BadgeStyle::Flat),
                    "flat-square" => Some(BadgeStyle::FlatSquare),
                    "for-the-badge" => Some(BadgeStyle::ForTheBadge),
                    "plastic" => Some(BadgeStyle::Plastic),
                    "social" => Some(BadgeStyle::Social),
                    _ => None,
                }
            })
            .unwrap_or(BadgeStyle::Unspecified)
    })
}

async fn get_svg(
    req: Request<Body>,
    http_client: Client,
    svg_base_url: &'static str,
) -> Result<(HeaderMap, u16, Vec<u8>, BadgeStyle), ()> {
    let suffix = if let Some(path_and_query) = req.uri().path_and_query() {
        path_and_query.as_str().replace(".png", ".svg")
    } else {
        "".to_owned()
    };
    let badge_style = get_badge_style(&req);

    let svg_url = format!("{}{}", svg_base_url, suffix);
    let mut headers = HeaderMap::new();
    for header in FORWARDING_REQUEST_HEADERS.iter() {
        if req.headers().contains_key(HeaderName::from_static(header)) {
            headers.insert(
                HeaderName::from_static(header),
                req.headers()[*header].to_owned(),
            );
        }
    }

    // NB: We're currently creating an owned byte vector of the response body,
    // which does involve copying those bytes into a new vector.
    //
    // This is done because we ultimately end up needing ownership of the
    // bytes in order to use the canonical API surface of the libs used
    // in the SVG --> PNG conversion.
    //
    // There __could__ potentially be some options to avoid this, at least in
    // most cases, but it doesn't make sense to do so at this time
    // given the effort/complexity vs. reward/benefit tradeoffs.
    match http_client.get(&svg_url).headers(headers).send().await {
        Ok(res) => {
            let headers = res.headers().to_owned();
            let status = res.status().as_u16();
            let bytes = match res.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to get SVG response body bytes. Details: {:?}", e);
                    return Err(());
                }
            };
            Ok((headers, status, bytes.to_vec(), badge_style))
        }
        Err(e) => {
            eprintln!("Failed to fetch SVG data. Details: {:?}", e);
            Err(())
        }
    }
}

async fn rasterize(
    req: Request<Body>,
    http_client: Client,
    svg_base_url: &'static str,
) -> Result<Response<Body>, hyper::http::Error> {
    let (svg_res_headers, svg_status, svg_bytes, badge_style) =
        match get_svg(req, http_client, svg_base_url).await {
            Ok((headers, status, data, style)) => (headers, status, data, style),
            Err(_) => {
                return Response::builder()
                    .status(502)
                    .body(Body::from(INVALID_SVG_BADGE.to_owned()));
            }
        };

    let mut res = Response::builder().header(CONTENT_TYPE, "image/png");
    // Unwrapping should be fine here as there's nothing in the preceding response
    // builder that could introduce errors.
    let res_headers = res.headers_mut().unwrap();
    for header in FORWARDING_RESPONSE_HEADERS.iter() {
        if svg_res_headers.contains_key(HeaderName::from_static(header)) {
            res_headers.append(
                HeaderName::from_static(header),
                svg_res_headers[*header].to_owned(),
            );
        }
    }

    if svg_status == 304 {
        return res.status(304).body(Body::empty());
    }

    let (png_stream, res_status) = match convert_svg_to_png(Some(svg_bytes), badge_style) {
        Ok(png_stream) => (png_stream, 200),
        Err(e) => {
            eprintln!("Failed to convert SVG to PNG. Details: {:?}", e);
            (INVALID_SVG_BADGE.to_owned(), 502)
        }
    };

    res.status(res_status).body(Body::from(png_stream))
}

async fn route(
    req: Request<Body>,
    http_client: Client,
    svg_base_url: &'static str,
) -> Result<Response<Body>, hyper::http::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::builder()
            .status(301)
            .header(hyper::header::LOCATION, svg_base_url)
            .body(Body::empty()),
        // TODO: This should probably incorporate a check of connectivity to the upstream SVG provider
        // https://tools.ietf.org/html/draft-inadarei-api-health-check-03#section-3
        (&Method::GET, "/health") => Response::builder()
            .status(200)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Body::from(HEALTH_CHECK_BODY)),
        (&Method::GET, _) => rasterize(req, http_client.to_owned(), svg_base_url).await,
        // GET is the only supported HTTP Verb at this time, and a GET request with an invalid badge route
        // will be handled by the above arm with a 404 response code. This arm just handles unsupported verbs.
        (_, _) => Response::builder().status(405).body(Body::empty()),
    }
}

pub(crate) async fn start_server(
    socket_addr: SocketAddr,
    svg_base_url: &'static str,
) -> Result<(), GenericServerError> {
    let client = Client::new();
    Server::bind(&socket_addr)
        .serve(make_service_fn(move |_| {
            let client = client.clone();
            async move {
                Ok::<_, GenericServerError>(service_fn(move |req| {
                    route(req, client.to_owned(), svg_base_url)
                }))
            }
        }))
        .await?;
    Ok(())
}
