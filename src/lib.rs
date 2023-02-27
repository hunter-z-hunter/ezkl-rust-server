use eyre::Result;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use std::net::{AddrParseError, SocketAddr};
use thiserror::Error;
mod hunter_z_hunter_rpc;
use hunter_z_hunter_rpc::{HunterZHunterApiServer, HunterZHunterRpc};

#[derive(Error, Debug)]
pub enum RpcError {
    #[error(transparent)]
    JsonRpcServerError(#[from] jsonrpsee::core::Error),
}

pub async fn run_server() -> Result<(SocketAddr, ServerHandle), RpcError> {
    let socket_addr =
        std::env::var("PORT").unwrap_or("0.0.0.0:3030".to_owned());

    let server = ServerBuilder::default()
        .build(socket_addr.parse::<SocketAddr>().unwrap())
        .await?;
    let addr = server.local_addr()?;
    let rpc_calls = HunterZHunterRpc::new();
    let handle = server.start(rpc_calls.into_rpc()).unwrap();

    Ok((addr, handle))
}
