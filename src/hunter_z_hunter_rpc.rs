use halo2curves::bn256::Fr;
/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result, __reexports::serde_json},
    proc_macros::rpc,
    tracing::info,
};

use core::panic;
use ezkl_lib::{
    commands::{Cli, Commands},
    execute::ExecutionError,
    pfsys::{prepare_data},
};
use ezkl_lib::{
    commands::{RunArgs, StrategyType, TranscriptType},
    execute::run,
    circuit::{CheckMode},
};
use halo2_proofs::{dev::MockProver, poly::commitment::ParamsProver};
use serde_json::Value;
use std::{env, error::Error, fs::File, sync::{Arc, Mutex}};
use std::{io::prelude::*, path::PathBuf};
use coins_ledger::transports::Ledger;
pub struct HunterZHunterRpc {}

trait HunterZHunterApi {
    #[method(name = "forward")]
    async fn forward(&self, input_data: Value) -> Result<Value>;
    // #[method(name = "mock")]
    // async fn mock(&self, input_data: Value, target_output_data: Value) -> Result<bool>;
    #[method(name = "submit_proof")]
    async fn submit_proof(&self, input_data: Value) -> Result<bool>;
    // #[method(name = "verify_aggr_proof")]
    // async fn verify_aggr_proof(&self, input_data: Value, target_output_data: Value)
    //     -> Result<bool>;
}


const SERVER_ARGS: RunArgs = RunArgs {
    tolerance: 0_usize,
    scale: 7_u32,
    bits: 16_usize,
    logrows: 17_u32,
    public_inputs: false,
    public_outputs: true,
    public_params: false,
    check_mode: CheckMode::UNSAFE,
    pack_base: 1_u32,
};

#[async_trait]
impl HunterZHunterRpc {
    async fn forward(&self, input_data: Value) -> Result<Value> {
        let cli = Cli {
            command: Commands::Forward {
                data: "./data/eth_tokyo/input.json".to_string(),
                model: "./data/eth_tokyo/network.onnx".to_string(),
                output: "output.json".to_string(),
            },
            args: SERVER_ARGS,
        };

        // Wrap Ledger in Arc<Mutex<T>>
        // let ledger = Arc::new(Mutex::new(Ledger::init().await?));
        env::set_var("EZKLCONF", "./data/forward.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/eth_tokyo/input.json").unwrap();
        run(cli).await.unwrap();
        let output = retrieve_json_data("output.json").unwrap();
        Ok(output)
    }


    async fn submit_proof(&self, input_data: Value) -> Result<bool> {
        let cli = Cli {
            command: Commands::Prove {
                data: "./data/eth_tokyo/input.json".to_string(),
                model: PathBuf::from("./data/eth_tokyo/network.onnx"),
                vk_path: PathBuf::from("/data/eth_tokyo/eth_tokyo.vk"),
                proof_path: PathBuf::from("/data/eth_tokyo/eth_tokyo.pf"),
                params_path: PathBuf::from("kzg.params"),
                transcript: TranscriptType::Blake,
                strategy: StrategyType::Single,
            },
            args: SERVER_ARGS,
        };
        
        env::set_var("EZKLCONF", "./data/submit_proof.json");
        let input_data_str = serde_json::to_string(&input_data)?;
        store_json_data(&input_data_str, "./data/eth_tokyo/input.json")?;
        let output_data = input_data["output_data"].clone();


        let res = run(cli).await;
        print!("res: {:?}", res);

        Ok(true)
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
