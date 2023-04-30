// utils.rs

use serde_json::Value;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use crate::types::ProofData;


pub fn generate_evm_config_json(
    input_json_path: &str,
    network_onnx_path: &str,
    vk_path: &str,
    proof_path: &str,
) -> String {
    format!(
        r#"{{
    "command": {{
      "Prove": {{
        "data": "{}",
        "model": "{}",
        "vk_path": "{}",
        "proof_path": "{}",
        "params_path": "kzg.params",
        "transcript": "EVM",
        "strategy": "Single"
      }}
    }},
    "args": {{
        "bits": 16,
        "check_mode": "UNSAFE",
        "logrows": 17,
        "pack_base": 1,
        "public_inputs": false,
        "public_outputs": true,
        "public_params": false,
        "scale": 7,
        "tolerance": 0
      }}
  }}"#,
        input_json_path, network_onnx_path, vk_path, proof_path
    )
}


pub fn generate_genevm_config_json(
    input_json_path: &str,
    network_onnx_path: &str,
    vk_path: &str,
    deployment_code_path: &str,
    sol_code_path: &str,
) -> String {
    format!(
        r#"{{
    "command": {{
      "CreateEVMVerifier": {{
        "data": "{}",
        "model": "{}",
        "params_path": "kzg.params",
        "vk_path": "{}",
        "deployment_code_path": "{}",
        "sol_code_path": "{}"
      }}
    }},
    "args": {{
        "bits": 16,
        "check_mode": "UNSAFE",
        "logrows": 17,
        "pack_base": 1,
        "public_inputs": false,
        "public_outputs": true,
        "public_params": false,
        "scale": 7,
        "tolerance": 0
      }}
  }}"#,
        input_json_path, network_onnx_path, vk_path, deployment_code_path, sol_code_path
    )
}



pub fn store_json_data(json_str: &str, path: &str) -> std::io::Result<()> {
    // Open the file for writing
    let mut file = File::create(path)?;

    // Write the Json data to the file
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

pub fn retrieve_json_data(path: &str) -> std::io::Result<Value> {
    // Open the file for reading
    let mut file = File::open(path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a JSON object
    let json_data: Value = serde_json::from_str(&contents)?;

    Ok(json_data)
}

pub fn save_onnx_file(onnx_file_data: &str, base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let decoded_data = base64::decode(onnx_file_data)?;
    let onnx_file_path = format!("{}/network.onnx", base_path);
    println!("WITHING SAVE ONNX Onnx file path: {}", onnx_file_path);
    let mut file = File::create(onnx_file_path)?;
    file.write_all(&decoded_data)?;
    Ok(())
}

pub fn retrieve_proof_data<P: AsRef<Path>>(path: P) -> Result<ProofData, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let proof_data: ProofData = serde_json::from_str(&contents)?;
    Ok(proof_data)
}