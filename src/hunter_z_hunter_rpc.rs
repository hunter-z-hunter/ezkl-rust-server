/// The RPC module for the Ethereum protocol required by Kakarot.
use jsonrpsee::{
    core::{async_trait, RpcResult as Result},
    proc_macros::rpc,
};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use ezkl::commands::{Cli, Commands, ProofSystem, RunArgs, StrategyType, TranscriptType};
use ezkl::execute::run;
pub struct HunterZHunterRpc {}

#[rpc(server, client)]
trait HunterZHunterApi {
    #[method(name = "hello_world")]
    async fn hello_world(&self) -> Result<String>;
    #[method(name = "generate_proof")]
    async fn generate_proof(&self, input_data: String) -> Result<Vec<u8>>;
}

#[async_trait]
impl HunterZHunterApiServer for HunterZHunterRpc {
    async fn hello_world(&self) -> Result<String> {
        Ok("Hello World!".to_string())
    }

    async fn generate_proof(&self, input_data: String) -> Result<Vec<u8>> {
        let cli_commands = Commands::Prove {
            /// The path to the .json data file, which should include both the network input (possibly private) and the network output (public input to the proof)
            data: input_data,
            /// The path to the .onnx model file
            model: PathBuf::from("model"),
            /// The path to output to the desired verfication key file
            vk_path: PathBuf::from("vk"),
            /// The path to the desired output file
            proof_path: PathBuf::from("proof"),
            /// The path to load the desired params file            
            params_path: PathBuf::from("params"),
            /// The path to load the desired params file
            pfsys: ProofSystem::KZG,
            transcript: TranscriptType::EVM,
            strategy: StrategyType::Single,
        };

        let cli_args = RunArgs {
            /// The tolerance for error on model outputs
            tolerance: 10_usize,
            /// The denominator in the fixed point representation used when quantizing
            scale: 100_i32,
            /// The number of bits used in lookup tables
            bits: 8_usize,
            /// The log_2 number of rows
            logrows: 10_u32,
            /// Flags whether inputs are public
            public_inputs: true,
            /// Flags whether outputs are public
            public_outputs: true,
            /// Flags whether params are public
            public_params: true,
            /// Flags to set maximum rotations
            max_rotations: 0_usize,
        };

        let cli = Cli {
            command: cli_commands,
            args: cli_args,
        };

        run(cli).await.unwrap();

        //TODO: Add return proof based on locally saved proof file
        let return_data = PathBuf::from("return_data");
        let mut data = Vec::new();
        File::open(return_data)?.read_to_end(&mut data)?;
        Ok(data)

    }
}

impl HunterZHunterRpc {
    pub fn new() -> Self {
        Self {}
    }
}
