use halo2curves::bn256::Fr;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result, __reexports::serde_json},
    proc_macros::rpc,
    tracing::info,
};

use core::panic;
use ezkl::{
    commands::{Cli, Commands},
    execute::ExecutionError,
    pfsys::{prepare_data, prepare_model_circuit_and_public_input},
};
use ezkl::{
    commands::{RunArgs, StrategyType, TranscriptType},
    execute::run,
};
use halo2_proofs::{dev::MockProver, poly::commitment::ParamsProver};
use serde_json::Value;
use std::{env, error::Error, fs::File};
use std::{io::prelude::*, path::PathBuf};
use hunter_z_hunter_rpc::request::PostData;

pub struct HunterZHunterRpc {}

#[rpc(server, client)]
trait HunterZHunterApi {
    #[method(name = "forward")]
    async fn forward(&self, input_data: Value) -> Result<Value>;
    #[method(name = "mock")]
    async fn mock(&self, input_data: Value, target_output_data: Value) -> Result<bool>;
    #[method(name = "submit_proof")]

    async fn submit_proof(&self, input_data: Value, target_output_data: Value) -> Result<bool>;
    #[method(name = "verify_aggr_proof")]
    async fn verify_aggr_proof(&self, input_data: Value, target_output_data: Value)
        -> Result<bool>;
}

const SERVER_ARGS: RunArgs = RunArgs {
    tolerance: 0_usize,
    scale: 4_i32,
    bits: 10_usize,
    logrows: 12_u32,
    public_inputs: false,
    public_outputs: true,
    public_params: false,
    max_rotations: 512_usize,
};

#[async_trait]
impl HunterZHunterApiServer for HunterZHunterRpc {
    async fn forward(&self, input_data: Value) -> Result<Value> {
        let cli = Cli {
            command: Commands::Forward {
                data: "./data/4l_relu_conv_fc/input.json".to_string(),
                model: "./data/4l_relu_conv_fc/network.onnx".to_string(),
                output: "output.json".to_string(),
            },
            args: SERVER_ARGS,
        };
        env::set_var("EZKLCONF", "./data/forward.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json").unwrap();
        run(cli).await.unwrap();
        let output = retrieve_json_data("output.json").unwrap();
        Ok(output)
    }

    async fn mock(&self, input_data: Value, target_output_data: Value) -> Result<bool> {
        env::set_var("EZKLCONF", "./data/mock.json");

        let cli = Cli {
            command: Commands::Mock {
                data: "./data/4l_relu_conv_fc/input.json".to_string(),
                model: "./data/4l_relu_conv_fc/network.onnx".to_string(),
            },
            args: SERVER_ARGS,
        };
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json")?;
        let output_data = input_data["output_data"].clone();
        let target_output_data = target_output_data["target_output_data"].clone();
        let output_data_vec: Vec<Vec<f64>> = serde_json::from_value(output_data)?;
        let target_output_data_vec: Vec<Vec<f64>> = serde_json::from_value(target_output_data)?;
        let distance = euclidean_distance(&output_data_vec[0], &target_output_data_vec[0]);
        let res = run(cli).await;
        print!("res: {:?}", res);
        match res {
            Ok(_) => {
                info!("mock success");
                if distance < 0.1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Ok(false),
        }
    }

    async fn submit_proof(&self, input_data: Value, target_output_data: Value) -> Result<bool> {
        let cli = Cli {
            command: Commands::Prove {
                data: "./data/4l_relu_conv_fc/input.json".to_string(),
                model: PathBuf::from("./data/4l_relu_conv_fc/network.onnx"),
                vk_path: PathBuf::from("4l_relu_conv_fc.vk"),
                proof_path: PathBuf::from("4l_relu_conv_fc.vk"),
                params_path: PathBuf::from("kzg.params"),
                transcript: TranscriptType::EVM,
                strategy: StrategyType::Single,
            },
            args: SERVER_ARGS,
        };
        env::set_var("EZKLCONF", "./data/submit_proof.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json")?;
        let output_data = input_data["output_data"].clone();
        let target_output_data = target_output_data["target_output_data"].clone();
        let output_data_vec: Vec<Vec<f64>> = serde_json::from_value(output_data)?;
        let target_output_data_vec: Vec<Vec<f64>> = serde_json::from_value(target_output_data)?;
        let distance = euclidean_distance(&output_data_vec[0], &target_output_data_vec[0]);

        // trigger payment

        let res = run(cli).await;
        print!("res: {:?}", res);
        match res {
            Ok(_) => {
                info!("mock success");
                if distance < 0.1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Ok(false),
        }
    }

    async fn verify_aggr_proof(
        &self,
        input_data: Value,
        target_output_data: Value,
    ) -> Result<bool> {
        env::set_var("EZKLCONF", "./data/submit_proof.json");
        let cli = Cli {
            command: Commands::VerifyAggr {
                proof_path: PathBuf::from("aggr_2l.pf"),
                vk_path: PathBuf::from("aggr_2l.vk"),
                params_path: PathBuf::from("kzg.params"),
                transcript: TranscriptType::EVM,
            },
            args: SERVER_ARGS,
        };
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/4l_relu_conv_fc/input.json").unwrap();
        let output_data = input_data["output_data"].clone();
        let target_output_data = target_output_data["target_output_data"].clone();
        let output_data_vec: Vec<Vec<f64>> = serde_json::from_value(output_data).unwrap();
        let target_output_data_vec: Vec<Vec<f64>> =
            serde_json::from_value(target_output_data).unwrap();
        let distance = euclidean_distance(&output_data_vec[0], &target_output_data_vec[0]);
        let res = run(cli).await;
        match res {
            Ok(_) => {
                info!("Verify success");
                if distance < 0.1 {
                    Ok(true)
                    // call the payment function

                } else {
                    Ok(false)
                }
            }
            Err(e) => {
                info!("Verify failed");
                Ok(false)
            }
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
fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    // check to make sure that a and b are the same length since the tensors should be the same
    assert_eq!(
        a.len(),
        b.len(),
        "The lengths of a and b are {} and {}. They should be the same length.",
        a.len(),
        b.len()
    );

    a.iter()
        .zip(b)
        .map(|(&x, &y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn passOutputData(output: &Value) -> Value {
    // call request and create a new hunt struct instance.
    // when we add hunt_id to the call, we'll pass it here as well
}

fn triggerPayment() -> bool {
        // define params for Ethers rust call
        // We pass in the huntID, the address of the winner, and the proof here.
        // let params = VerifyAwardParams::new();
        // let result = hunter_caller::main(params).unwrap();
        // println!("contract logs: {:?}", result);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a: &Vec<f64> = &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b: &Vec<f64> = &vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        assert_eq!(euclidean_distance(&a, &b), 18.16590212458495);
    }

    #[test]
    #[should_panic(
        expected = "The lengths of a and b are 10 and 9. They should be the same length."
    )]
    fn test_euclidean_distance_different_lengths() {
        let a: &Vec<f64> = &vec![1.0, 2.0, 3.8, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 110.8];
        let b: &Vec<f64> = &vec![10.0, 9.0, 84.0, 7.0, 6.4, 51.0, 4.0, 3.8, 2.0];
        euclidean_distance(&a, &b);
    }
}
