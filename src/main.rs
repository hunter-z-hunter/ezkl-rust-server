use dotenv::dotenv;
use eyre::Result;
use log::{error, info};
use rpc_server::run_server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    colog::init();
    let (addr, server_handle) = run_server().await.unwrap();
    let url = format!("http://{}", addr);
    println!("Server started, listening on {url}");

    server_handle.stopped().await;

    Ok(())
}
