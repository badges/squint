mod server;
use server::{start_server, GenericServerError};

const DEFAULT_PORT: u16 = 3001;

#[tokio::main]
async fn main() -> Result<(), GenericServerError> {
    let port =
        std::env::var("PORT").map_or(DEFAULT_PORT, |p| p.parse::<u16>().unwrap_or(DEFAULT_PORT));
    let socket_address: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    start_server(socket_address).await?;
    Ok(())
}
