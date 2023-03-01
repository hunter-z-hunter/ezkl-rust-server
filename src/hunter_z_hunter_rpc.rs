/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result, __reexports::serde_json},
    proc_macros::rpc,
    tracing::info,
};

use ezkl::commands::{Cli};
use ezkl::execute::run;
use std::env;

pub struct HunterZHunterRpc {}

#[rpc(server, client)]
trait HunterZHunterApi {
    #[method(name = "call_run")]
    async fn call_run(&self, cli: Cli) -> Result<()>;
    #[method(name = "mock")]
    async fn mock(&self, cli: Cli) -> Result<bool>;
    #[method(name = "submit_proof")]
    async fn submit_proof(&self, cli: Cli) -> Result<()>;
}

#[async_trait]
impl HunterZHunterApiServer for HunterZHunterRpc {

    async fn call_run(&self, cli: Cli) -> Result<()>{
        env::set_var("EZKLCONF", "data");
        run(cli).await.unwrap();
        Ok(())
    }

    async fn mock(&self, config: Cli) -> Result<bool> {
        env::set_var("EZKLCONF", "./data/mock.json");
        run(config).await.unwrap();
        Ok(true)
    }

    async fn submit_proof(&self, config: Cli) -> Result<()> {
        env::set_var("EZKLCONF", "./data/submit_proof.json");
        run(config).await.unwrap();
        Ok(())
    }    
}

impl HunterZHunterRpc {
    pub fn new() -> Self {
        Self {}
    }
}


