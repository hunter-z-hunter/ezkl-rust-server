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
                // find euclidian distance
                euclidean_distance(input_data, ) // where do we find the target data?
                Ok(true)
            }
            Err(e) => {
                info!("mock failed");
                Ok(false)
            }
        }
    }

    async fn submit_proof(&self, cli: Cli, input_data: Value, target_data: Value) -> Result<()> {
        env::set_var("EZKLCONF", "./data/submit_proof.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json").unwrap();
        run(cli).await.unwrap();
        Ok(()) => {
            info!("mock success");
            // find euclidian distance
            euclidean_distance(input_data, target_data); 
            Ok(true)
        }
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

// Finding the Euclidian distance between the two output tensors of our machine learning model
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    // check to make sure that a and b are the same length since the tensors should be the same
    assert_eq!(a.len(), b.len(), "The lengths of a and b are {} and {}. They should be the same length.", a.len(), b.len());

    a.iter()
        .zip(b)
        .map(|(&x, &y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = [10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        assert_eq!(euclidean_distance(&a, &b), 18.16590212458495);
    }

    #[test]
    #[should_panic(expected = "The lengths of a and b are 10 and 9. They should be the same length.")]
    fn test_euclidean_distance_different_lengths() {
        let a = [1.0, 2.0, 3.8, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 110.8];
        let b = [10.0, 9.0, 84.0, 7.0, 6.4, 51.0, 4.0, 3.8, 2.0];
        euclidean_distance(&a, &b);
    }
}

