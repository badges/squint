use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::net::SocketAddr;

pub type GenericServerError = Box<dyn std::error::Error + Send + Sync>;

const SVG_BASE_URL: &str = "https://img.shields.io";
const HEALTH_CHECK_BODY: &str = concat!(
    r#"{"status":"pass","version":""#,
    env!("CARGO_PKG_VERSION"),
    r#""}"#
);

async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::builder()
            .status(301)
            .header(hyper::header::LOCATION, SVG_BASE_URL)
            .body(Body::empty()),
        // TODO: This should probably incorporate a check of connectivity to the upstream SVG provider
        // https://tools.ietf.org/html/draft-inadarei-api-health-check-03#section-3
        (&Method::GET, "/health") => Response::builder()
            .status(200)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Body::from(HEALTH_CHECK_BODY)),
        // (&Method::GET, _) => rasterize(req, http_client.to_owned()).await,
        // GET is the only supported HTTP Verb at this time, and a GET request with an invalid badge route
        // will be handled by the above arm with a 404 response code. This arm just handles unsupported verbs.
        (_, _) => Response::builder().status(405).body(Body::empty()),
    }
}

pub(crate) async fn start_server(socket_addr: SocketAddr) -> Result<(), GenericServerError> {
    Server::bind(&socket_addr)
        .serve(make_service_fn(move |_| async {
            Ok::<_, GenericServerError>(service_fn(route))
        }))
        .await?;
    Ok(())
}
