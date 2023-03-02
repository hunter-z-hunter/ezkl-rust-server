use halo2curves::bn256::Fr;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result, __reexports::serde_json},
    proc_macros::rpc,
    tracing::info,
};

use ezkl::execute::run;
use ezkl::{
    commands::{Cli, Commands},
    execute::ExecutionError,
    pfsys::{prepare_data, prepare_model_circuit_and_public_input},
};
use halo2_proofs::{dev::MockProver, poly::commitment::ParamsProver};
use serde_json::Value;
use std::io::prelude::*;
use std::{env, error::Error, fs::File};

pub struct HunterZHunterRpc {}

#[rpc(server, client)]
trait HunterZHunterApi {
    #[method(name = "call_run")]
    async fn call_run(&self, cli: Cli) -> Result<()>;
    #[method(name = "mock")]
    async fn mock(&self, cli: Cli, input_data: Value) -> Result<bool>;
    #[method(name = "submit_proof")]
    async fn submit_proof(&self, cli: Cli, input_data: Value, target_data: Value) -> Result<()>;
}

#[async_trait]
impl HunterZHunterApiServer for HunterZHunterRpc {
    async fn call_run(&self, cli: Cli) -> Result<()> {
        env::set_var("EZKLCONF", "data");
        run(cli).await.unwrap();
        Ok(())
    }

    async fn mock(&self, cli: Cli, input_data: Value) -> Result<bool> {
        env::set_var("EZKLCONF", "./data/mock.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json").unwrap();
        let res = run(cli).await;
        match res {
            Ok(_) => {
                info!("mock success");
                Ok(true)
            }
            Err(e) => {
                info!("mock failed");
                Ok(false)
            }
        }
    }

    async fn submit_proof(&self, cli: Cli, input_data: Value, target_output_data: Value) -> Result<()> {
        env::set_var("EZKLCONF", "./data/submit_proof.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json").unwrap();
        run(cli).await.unwrap();
        Ok(())
    }
}

impl HunterZHunterRpc {
    pub fn new() -> Self {
        Self {}
    }
}

fn store_json_data(json_str: &str, path: &str) -> std::io::Result<()> {
    // Open the file for writing
    let mut file = File::create(path)?;

    // Write the Json data to the file
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

fn retrieve_json_data(path: &str) -> std::io::Result<Value> {
    // Open the file for reading
    let mut file = File::open(path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a JSON object
    let json_data: Value = serde_json::from_str(&contents)?;

    Ok(json_data)
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += (a[i] - b[i]).powi(2);
    }
    sum.sqrt()
}
