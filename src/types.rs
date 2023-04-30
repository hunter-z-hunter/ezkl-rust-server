// types.rs

use ezkl_lib::circuit::CheckMode;
use ezkl_lib::commands::{RunArgs, StrategyType, TranscriptType};
use serde::{Deserialize, Serialize};


pub struct EzklServer {}

pub const SERVER_ARGS: RunArgs = RunArgs {
    bits: 16_usize,
    check_mode: CheckMode::UNSAFE,
    logrows: 19_u32,
    pack_base: 1_u32,
    public_inputs: false,
    public_outputs: true,
    public_params: false,
    scale: 7_u32,
    tolerance: 0_usize,
    allocated_constraints: None,
};

#[derive(Debug, Deserialize, Serialize)] // <-- add Serialize here
pub struct JsonRpcRequest<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: u64,
}

#[derive(Debug, Deserialize, Serialize)] // <-- add Serialize here
pub struct JsonRpcParams {
    pub input_data: Vec<Vec<f32>>,
    pub input_shapes: Vec<Vec<usize>>,
    pub output_data: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoData {
    pub input_data: Vec<Vec<f32>>,
    pub input_shapes: Vec<Vec<usize>>,
    pub output_data: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnnxFileData {
    pub onnx_file_content: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEvmContractData {
    pub project_name: String,
    pub echo_data: EchoData,
    pub onnx_file_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofData {
    pub num_instance: Vec<usize>,
    pub instances: Vec<Vec<Vec<u64>>>,
    pub proof: Vec<u8>,
}