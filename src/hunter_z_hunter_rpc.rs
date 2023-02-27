/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result, __reexports::serde_json},
    proc_macros::rpc, tracing::info,
};
use std::{fs::File, error::Error};
use std::io::{Read};
use std::path::PathBuf;

use ezkl::commands::{Cli, Commands, ProofSystem, RunArgs, StrategyType, TranscriptType};
use ezkl::execute::run;
use std::env;

pub struct HunterZHunterRpc {}

const EZKLCONF: &str = "EZKLCONF";

#[rpc(server, client)]
trait HunterZHunterApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;
    #[method(name = "generate_proof")]
    async fn generate_proof(&self) -> Result<String>;
    #[method(name = "create_kzg_params")]
    async fn create_kzg_params(&self) -> Result<()>;
    #[method(name = "call_run")]
    async fn call_run(&self, cli: Cli) -> Result<()>;
}

#[async_trait]
impl HunterZHunterApiServer for HunterZHunterRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn generate_proof(&self) -> Result<String> {
        println!("Generating proof...");

        // ezkl --bits=16 -K=17 prove -D ./examples/onnx/examples/1l_conv/input.json -M ./examples/onnx/examples/1l_conv/network.onnx --proof-path 1l_conv.pf --vk-path 1l_conv.vk --params-path=kzg.params --transcript=evm
        // {"command":{"Prove":{"data":"./examples/onnx/examples/1l_conv/input.json","model":"./examples/onnx/examples/1l_conv/network.onnx","vk_path":"1l_conv.vk","proof_path":"1l_conv.pf","params_path":"kzg.params","pfsys":"KZG","transcript":"EVM","strategy":"Single"}},
        // "args":{"tolerance":0,"scale":7,"bits":16,"logrows":17,"public_inputs":false,"public_outputs":true,"public_params":false,"max_rotations":512}}
        let input_data_path =PathBuf::from("./data/1l_conv/input.json");
        let mut file = File::open(input_data_path).map_err(Box::<dyn Error>::from).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(Box::<dyn Error>::from).unwrap();
        println!("data: {}", data);

        let cli_commands = Commands::Prove {
            /// The path to the .json data file, which should include both the network input (possibly private) and the network output (public input to the proof)
            // data: PathBuf::from("./data/1l_conv/input.json"),
            data: "./data/1l_conv/input.json".to_string(),
            /// The path to the .onnx model file
            model: PathBuf::from("./data/1l_conv/network.onnx"),
            /// The path to output to the desired verfication key file
            vk_path: PathBuf::from("1l_conv.vk"),
            /// The path to the desired output file
            proof_path: PathBuf::from("1l_conv.pf"),
            /// The path to load the desired params file            
            params_path: PathBuf::from("kzg.params"),
            /// The path to load the desired params file
            pfsys: ProofSystem::KZG,
            transcript: TranscriptType::EVM,
            strategy: StrategyType::Single,
        };

        let cli_args = RunArgs {
            /// The tolerance for error on model outputs
            tolerance: 0_usize,
            /// The denominator in the fixed point representation used when quantizing
            scale: 7_i32,
            /// The number of bits used in lookup tables
            bits: 16_usize,
            /// The log_2 number of rows
            logrows: 17_u32,
            /// Flags whether inputs are public
            public_inputs: false,
            /// Flags whether outputs are public
            public_outputs: true,
            /// Flags whether params are public
            public_params: false,
            /// Flags to set maximum rotations
            max_rotations: 512_usize,
        };

        let cli = Cli {
            command: cli_commands,
            args: cli_args,
        };
        println!("cli: {:?}", cli);
        println!("Before run");
        env::set_var("EZKLCONF", cli.as_json().unwrap());
        run(cli).await.unwrap();

        //TODO: Add return proof based on locally saved proof file

        let input_data_path =PathBuf::from("./data/1l_conv/input.json");
        let mut file = File::open(input_data_path).map_err(Box::<dyn Error>::from).unwrap();
        let mut final_data = String::new();
        file.read_to_string(&mut final_data)
            .map_err(Box::<dyn Error>::from).unwrap();

        // let return_data = PathBuf::from("return_data");
        // let mut data = Vec::new();
        // File::open(return_data)?.read_to_end(&mut data)?;
        Ok(final_data)

    }


    async fn create_kzg_params(&self) -> Result<()> {
        println!("Creating KZG params...");
        // ezkl --bits=16 -K=17 create-kzg-params --params-path=kzg.params
        // {"command":{"CreateKZGParams":{"params_path":"kzg.params"}},"args":{"bits":16,"logrows":17}}
        let cli_commands = Commands::GenSrs {
            /// The path to the desired output file
            params_path: PathBuf::from("./data/kzg.params"),
            pfsys: ProofSystem::KZG,
        };


        let cli_args = RunArgs {
            /// The tolerance for error on model outputs
            tolerance: 0_usize,
            /// The denominator in the fixed point representation used when quantizing
            scale: 7_i32,
            /// The number of bits used in lookup tables
            bits: 16_usize,
            /// The log_2 number of rows
            logrows: 17_u32,
            /// Flags whether inputs are public
            public_inputs: false,
            /// Flags whether outputs are public
            public_outputs: true,
            /// Flags whether params are public
            public_params: false,
            /// Flags to set maximum rotations
            max_rotations: 512_usize,
        };

        let cli = Cli {
            command: cli_commands,
            args: cli_args,
        };
        println!("cli: {:?}", cli);
        println!("Before run");
        env::set_var("EZKLCONF", cli.as_json().unwrap());
        run(cli).await.unwrap();
        Ok(())
    }

    async fn call_run(&self, config: Cli) -> Result<()> {
        println!("Before run");
        println!("config: {:?}", config);
        info!("config: {:?}", config);
        env::set_var(EZKLCONF, serde_json::to_string(&config)?);
        let value = run(config).await;
        println!("value: {:?}", value);
        println!("After run");
        Ok(())
    }

}

impl HunterZHunterRpc {
    pub fn new() -> Self {
        Self {}
    }
}