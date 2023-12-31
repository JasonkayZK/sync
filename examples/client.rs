use std::env;

use sync::storage_proto::storage_client::StorageClient;
use sync::storage_proto::{AddRequest, ListRequest, RegisterRequest};
use sync::utils::get_port;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_port = get_port(env::args().collect());
    let addr = format!("http://0.0.0.0:{}", server_port);

    let mut cli = StorageClient::connect(addr).await.unwrap();

    cli.register(RegisterRequest {
        connect_addr: "localhost:8887".to_string(),
    })
    .await
    .unwrap();

    cli.add(AddRequest {
        key: boost_rs::rand::string::get_random_alphanumeric_string(3),
    })
    .await
    .unwrap();

    println!(
        "list: {:#?}",
        cli.list(ListRequest {}).await.unwrap().into_inner().data
    );

    Ok(())
}
