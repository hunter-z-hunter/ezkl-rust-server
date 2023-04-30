use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use ezkl_lib::circuit::CheckMode;
use ezkl_lib::{
    commands::{Cli, Commands, RunArgs, StrategyType, TranscriptType},
    execute::{run, ExecutionError},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::{
    env,
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
};

use base64::{self, decode};
use env_logger::Builder;
use log::{info, LevelFilter};
use std::io::Write;
use std::path::Path;

pub struct EzklServer {}

const SERVER_ARGS: RunArgs = RunArgs {
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
struct JsonRpcRequest<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: u64,
}

#[derive(Debug, Deserialize, Serialize)] // <-- add Serialize here
struct JsonRpcParams {
    input_data: Vec<Vec<f32>>,
    input_shapes: Vec<Vec<usize>>,
    output_data: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EchoData {
    input_data: Vec<Vec<f32>>,
    input_shapes: Vec<Vec<usize>>,
    output_data: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OnnxFileData {
    onnx_file_content: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct CreateEvmContractData {
    project_name: String,
    echo_data: EchoData,
    onnx_file_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProofData {
    num_instance: Vec<usize>,
    instances: Vec<Vec<Vec<u64>>>,
    proof: Vec<u8>,
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

fn save_onnx_file(onnx_file_data: &str, base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let decoded_data = base64::decode(onnx_file_data)?;
    let onnx_file_path = format!("{}/network.onnx", base_path);
    println!("WITHING SAVE ONNX Onnx file path: {}", onnx_file_path);
    let mut file = File::create(onnx_file_path)?;
    file.write_all(&decoded_data)?;
    Ok(())
}

fn retrieve_proof_data<P: AsRef<Path>>(path: P) -> Result<ProofData, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let proof_data: ProofData = serde_json::from_str(&contents)?;
    Ok(proof_data)
}

#[post("/forward")]
async fn forward(input_data: web::Json<EchoData>) -> impl Responder {
    let cli = Cli {
        command: Commands::Forward {
            data: "./data/baby_gaia_2d/input.json".to_string(),
            model: "./data/baby_gaia_2d/network.onnx".to_string(),
            output: "./data/baby_gaia_2d/output.json".to_string(),
        },
        args: SERVER_ARGS,
    };

    info!("Cli: {:?}", cli);

    env::set_var("EZKLCONF", "./data/forward.json");
    let input_data_str = serde_json::to_string(&input_data.into_inner()).unwrap();

    info!("input_data_str: {:?}", input_data_str);

    store_json_data(&input_data_str, "./data/baby_gaia_2d/input.json").unwrap();

    run(cli).await.unwrap();
    let output_str = retrieve_json_data("./data/baby_gaia_2d/output.json").unwrap();
    let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    info!("Output: {:?}", output);

    HttpResponse::Ok().json(output)
}

#[post("/submit_proof")]
async fn submit_proof(input_data: web::Json<EchoData>) -> impl Responder {
    let cli = Cli {
        command: Commands::Prove {
            data: "./data/baby_gaia_2d/input.json".to_string(),
            model: PathBuf::from("./data/baby_gaia_2d/network.onnx"),
            vk_path: PathBuf::from("./data/baby_gaia_2d/baby_gaia_2d.vk"),
            proof_path: PathBuf::from("./data/baby_gaia_2d/baby_gaia_2d.pf"),
            params_path: PathBuf::from("kzg.params"),
            transcript: TranscriptType::EVM,
            strategy: StrategyType::Single,
        },
        args: RunArgs {
            pack_base: 1_u32,
            bits: 16_usize,
            check_mode: CheckMode::UNSAFE,
            logrows: 19_u32,
            public_inputs: false,
            public_outputs: true,
            public_params: false,
            scale: 7_u32,
            tolerance: 0_usize,
            allocated_constraints: None,
        },
    };
    info!("1.Cli: {:?}", cli);

    env::set_var("EZKLCONF", "./data/baby_gaia_2d/prove_baby_gaia.json");
    let input_data_str = serde_json::to_string(&input_data).unwrap();
    store_json_data(&input_data_str, "./data/baby_gaia_2d/input.json").unwrap();

    info!("2.Input_data_str: {:?}", input_data_str);
    let res = run(cli).await;

    info!("3.Res: {:?}", res);
    let output_str = retrieve_json_data("./data/baby_gaia_2d/output.json").unwrap();
    let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    info!("4.Output: {:?}", output);

    HttpResponse::Ok().json(output)
}

#[post("/mock")]
async fn mock(input_data: web::Json<EchoData>) -> impl Responder {
    let cli = Cli {
        command: Commands::Mock {
            data: "./data/baby_gaia_2d/input.json".to_string(),
            model: "./data/baby_gaia_2d/network.onnx".to_string(),
        },
        args: RunArgs {
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
        },
    };
    info!("1.Cli: {:?}", cli);

    env::set_var("EZKLCONF", "./data/baby_gaia_2d/mock_baby_gaia.json");
    let input_data_str = serde_json::to_string(&input_data.into_inner()).unwrap();
    store_json_data(&input_data_str, "./data/baby_gaia_2d/input.json").unwrap();

    info!("2.Input_data_str: {:?}", input_data_str);
    let res = run(cli).await;

    info!("3.Res: {:?}", res);
    let output_str = retrieve_json_data("./data/baby_gaia_2d/output.json").unwrap();
    let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    info!("4.Output: {:?}", output);

    HttpResponse::Ok().json(output)
}

#[post("/generate_evm_contract")]
async fn generate_evm_contract(data: web::Json<CreateEvmContractData>) -> impl Responder {
    let project_name = &data.project_name;
    let echo_data = &data.echo_data;
    let onnx_file_data = &data.onnx_file_data;
    // let onnx_file_path = "./network.onnx";

    println!("Project name: {}", project_name);
    println!("Echo data: {:?}", echo_data);
    println!("Onnx file data: {}", onnx_file_data);
    // Save the ONNX file locally

    let base_path = format!("./data/{}", project_name);

    // Check if the directory exists, and create it if it doesn't
    if !Path::new(&base_path).exists() {
        fs::create_dir_all(&base_path).unwrap();
        println!("Created directory: {}", base_path);
    } else {
        println!("Directory already exists: {}", base_path);
    }

    save_onnx_file(onnx_file_data, &base_path).unwrap();

    let input_json_path = format!("{}/input.json", base_path);
    let network_onnx_path = format!("{}/network.onnx", base_path);
    let vk_path = format!("{}/{}.vk", base_path, project_name);
    let proof_path = format!("{}/{}.pf", base_path, project_name);
    let evm_config_path = format!("{}/prove_{}.json", base_path, project_name);
    let deployment_code_path = format!("{}/{}.code", base_path, project_name);
    let sol_code_path = format!("{}.sol", project_name);

    println!("1. Input json path: {}", input_json_path);
    println!("1. Network onnx path: {}", network_onnx_path);
    println!("1. Vk path: {}", vk_path);
    println!("1. Proof path: {}", proof_path);
    println!("1. Evm config path: {}", evm_config_path);
    println!("1. Deployment code path: {}", deployment_code_path);
    println!("1. Sol code path: {}", sol_code_path);

    let cli_prove = Cli {
        command: Commands::Prove {
            data: input_json_path.clone(),
            model: PathBuf::from(network_onnx_path.clone()),
            vk_path: PathBuf::from(vk_path.clone()),
            proof_path: PathBuf::from(proof_path.clone()),
            params_path: PathBuf::from("kzg.params"),
            transcript: TranscriptType::EVM,
            strategy: StrategyType::Single,
        },
        args: RunArgs {
            pack_base: 1_u32,
            bits: 16_usize,
            check_mode: CheckMode::UNSAFE,
            logrows: 17_u32,
            public_inputs: false,
            public_outputs: true,
            public_params: false,
            scale: 7_u32,
            tolerance: 0_usize,
            allocated_constraints: None,
        },
    };

    println!("2. Input json path: {}", input_json_path);
    println!("2. Network onnx path: {}", network_onnx_path);
    println!("2. Vk path: {}", vk_path);
    println!("2. Proof path: {}", proof_path);
    println!("2. Evm config path: {}", evm_config_path);
    println!("2. Deployment code path: {}", deployment_code_path);
    println!("2. Sol code path: {}", sol_code_path);

    let cli = Cli {
        command: Commands::CreateEVMVerifier {
            data: input_json_path.clone(),
            model: PathBuf::from(network_onnx_path.clone()),
            vk_path: PathBuf::from(vk_path.clone()),
            deployment_code_path: Some(PathBuf::from(deployment_code_path.clone())),
            params_path: PathBuf::from("kzg.params"),
            sol_code_path: Some(PathBuf::from(sol_code_path.clone())),
        },
        args: RunArgs {
            pack_base: 1_u32,
            bits: 16_usize,
            check_mode: CheckMode::UNSAFE,
            logrows: 17_u32,
            public_inputs: false,
            public_outputs: true,
            public_params: false,
            scale: 7_u32,
            tolerance: 0_usize,
            allocated_constraints: None,
        },
    };

    info!("1.1 Cli: {:?}", cli_prove);
    info!("1.2 Cli: {:?}", cli);

    // env::set_var("EZKLCONF", evm_config_path);
    env::set_var("EZKLCONF", "./data/test_project/prove_test_project.json");

    let input_data_str = serde_json::to_string(echo_data).unwrap();
    info!("2.1 Input_data_str: {:?}", input_data_str);
    store_json_data(&input_data_str, &input_json_path).unwrap();

    info!("2.1 Input_data_str: {:?}", input_data_str);
    let res = run(cli_prove).await;
    info!("3.1 Res: {:?}", res);

    env::set_var("EZKLCONF", "./data/test_project/genevm_test_project.json");

    let res_2 = run(cli).await;
    info!("3.2 Res: {:?}", res_2);

    // Read the generated .sol file content
    let sol_code = std::fs::read_to_string(&sol_code_path)
        .unwrap_or_else(|_| panic!("Unable to read the generated .sol file: {}", sol_code_path));

    info!("4.Sol_code: {:?}", sol_code);

    HttpResponse::Ok().body(sol_code)
}

#[post("/prove")]
async fn prove(data: web::Json<CreateEvmContractData>) -> impl Responder {
    let project_name = &data.project_name;
    let echo_data = &data.echo_data;
    let onnx_file_data = &data.onnx_file_data;
    // let onnx_file_path = "./network.onnx";

    println!("Project name: {}", project_name);
    println!("Echo data: {:?}", echo_data);
    println!("Onnx file data: {}", onnx_file_data);
    // Save the ONNX file locally

    let base_path = format!("./data/{}", project_name);

    // Check if the directory exists, and create it if it doesn't
    if !Path::new(&base_path).exists() {
        fs::create_dir_all(&base_path).unwrap();
        println!("Created directory: {}", base_path);
    } else {
        println!("Directory already exists: {}", base_path);
    }

    save_onnx_file(onnx_file_data, &base_path).unwrap();

    let input_json_path = format!("{}/input.json", base_path);
    let network_onnx_path = format!("{}/network.onnx", base_path);
    let vk_path = format!("{}/{}.vk", base_path, project_name);
    let proof_path = format!("{}/{}.pf", base_path, project_name);
    let evm_config_path = format!("{}/prove_{}.json", base_path, project_name);
    let deployment_code_path = format!("{}/{}.code", base_path, project_name);
    let sol_code_path = format!("{}.sol", project_name);

    println!("1. Input json path: {}", input_json_path);
    println!("1. Network onnx path: {}", network_onnx_path);
    println!("1. Vk path: {}", vk_path);
    println!("1. Proof path: {}", proof_path);
    println!("1. Evm config path: {}", evm_config_path);
    println!("1. Deployment code path: {}", deployment_code_path);
    println!("1. Sol code path: {}", sol_code_path);

    let cli_prove = Cli {
        command: Commands::Prove {
            data: input_json_path.clone(),
            model: PathBuf::from(network_onnx_path.clone()),
            vk_path: PathBuf::from(vk_path.clone()),
            proof_path: PathBuf::from(proof_path.clone()),
            params_path: PathBuf::from("kzg.params"),
            transcript: TranscriptType::EVM,
            strategy: StrategyType::Single,
        },
        args: RunArgs {
            pack_base: 1_u32,
            bits: 16_usize,
            check_mode: CheckMode::UNSAFE,
            logrows: 17_u32,
            public_inputs: false,
            public_outputs: true,
            public_params: false,
            scale: 7_u32,
            tolerance: 0_usize,
            allocated_constraints: None,
        },
    };

    info!("1.1 Cli: {:?}", cli_prove);

    // env::set_var("EZKLCONF", evm_config_path);
    env::set_var("EZKLCONF", "./data/test_project/prove_test_project.json");

    let input_data_str = serde_json::to_string(echo_data).unwrap();
    info!("2.1 Input_data_str: {:?}", input_data_str);
    store_json_data(&input_data_str, &input_json_path).unwrap();

    info!("2.1 Input_data_str: {:?}", input_data_str);
    let res = run(cli_prove).await;
    // info!("3.1 Res: {:?}", res);
    // let input_json_path = format!("{}/output.json", base_path);
    // let output_str = retrieve_json_data(&input_json_path).unwrap();
    // let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    // info!("4.Output: {:?}", output);
    let output = format!("{:?}", res);
    let proof_data = retrieve_proof_data(&proof_path).unwrap();
    info!("5.Proof Data: {:?}", proof_data);

    HttpResponse::Ok().json((output, proof_data))
}

pub async fn run_server() -> std::io::Result<()> {
    let addr = format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or_else(|_| String::from("8080"))
    );
    let rpc = web::Data::new(EzklServer {});

    // Initialize the logger
    Builder::new().filter(None, LevelFilter::Info).init();

    // Your server initialization code here

    info!("Server started on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(rpc.clone())
            .service(forward)
            .service(submit_proof)
            .service(mock)
            .service(generate_evm_contract)
            .service(prove)
    })
    .bind(addr)?
    .run()
    .await
}
