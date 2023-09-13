use dotenv::dotenv;
use log::info;
use std::env;
use sync::api::StorageService;
use tonic::transport::Server;

use sync::logger;
use sync::storage_proto::storage_server::StorageServer;
use sync::utils::get_port;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logger::init();
    let server_port = get_port(env::args().collect());
    let addr = format!("0.0.0.0:{}", server_port);
    info!("Listening on port {}", addr);

    Server::builder()
        .add_service(StorageServer::new(StorageService))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
