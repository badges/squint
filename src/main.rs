mod badge;
mod graphics;
mod server;
use graphics::INVALID_SVG_BADGE;
use server::{start_server, GenericServerError};
#[macro_use]
extern crate lazy_static;

const DEFAULT_PORT: u16 = 3001;
const DEFAULT_SVG_BASE_URL: &str = "https://img.shields.io";
lazy_static! {
    static ref SVG_BASE_URL: String =
        std::env::var("SVG_BASE_URL").unwrap_or_else(|_| DEFAULT_SVG_BASE_URL.to_owned());
}

#[tokio::main]
async fn main() -> Result<(), GenericServerError> {
    lazy_static::initialize(&INVALID_SVG_BADGE);
    lazy_static::initialize(&SVG_BASE_URL);
    let port =
        std::env::var("PORT").map_or(DEFAULT_PORT, |p| p.parse::<u16>().unwrap_or(DEFAULT_PORT));
    let socket_address: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    start_server(socket_address, &SVG_BASE_URL, INVALID_SVG_BADGE.as_slice()).await?;
    Ok(())
}
