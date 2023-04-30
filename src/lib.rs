use actix_web::{ post, web, App, HttpResponse, HttpServer, Responder};
use ezkl_lib::circuit::CheckMode;
use ezkl_lib::{
    commands::{Cli, Commands, RunArgs, StrategyType, TranscriptType},
    execute::{run},
};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self},

    path::PathBuf,
};

use base64::{self, decode};
use env_logger::Builder;
use log::{info, LevelFilter};

use std::path::Path;

mod utils;
mod types;

use utils::{generate_evm_config_json, generate_genevm_config_json, save_onnx_file, store_json_data, retrieve_proof_data};

use types::{
    EzklServer, SERVER_ARGS, JsonRpcRequest, JsonRpcParams, EchoData, OnnxFileData,
    CreateEvmContractData, ProofData,
};

use crate::utils::retrieve_json_data;



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

    // Create project directory
    let base_path = format!("./data/{}", project_name);
    if !Path::new(&base_path).exists() {
        fs::create_dir_all(&base_path).unwrap();
        println!("Created directory: {}", base_path);
    } else {
        println!("Directory already exists: {}", base_path);
    }

    // Save ONNX file and input JSON
    save_onnx_file(onnx_file_data, &base_path).unwrap();
    let input_json_path = format!("{}/input.json", base_path);
    let input_data_str = serde_json::to_string(echo_data).unwrap();
    store_json_data(&input_data_str, &input_json_path).unwrap();

    // Generate and save prove and genevm config JSON
    let network_onnx_path = format!("{}/network.onnx", base_path);
    let vk_path = format!("{}/{}.vk", base_path, project_name);
    let proof_path = format!("{}/{}.pf", base_path, project_name);
    let prove_config_path = format!("{}/prove_{}.json", base_path, project_name);
    let genevm_config_path = format!("{}/genevm_{}.json", base_path, project_name);
    let prove_config_json = generate_evm_config_json(&input_json_path, &network_onnx_path, &vk_path, &proof_path);
    let deployment_code_path = format!("{}/{}.code", base_path, project_name);
    let sol_code_path = format!("{}.sol", project_name);
    let genevm_config_json = generate_genevm_config_json(&input_json_path, &network_onnx_path, &vk_path, &deployment_code_path, &sol_code_path);
    store_json_data(&prove_config_json, &prove_config_path).expect("Unable to write prove config file");
    store_json_data(&genevm_config_json, &genevm_config_path).expect("Unable to write genevm config file");

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

    env::set_var("EZKLCONF", &prove_config_path);
    run(cli_prove).await;

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

    env::set_var("EZKLCONF", &genevm_config_path);
    run(cli).await;
    // Read and return the generated .sol file content
    let sol_code = std::fs::read_to_string(&sol_code_path)
        .unwrap_or_else(|_| panic!("Unable to read the generated .sol file: {}", sol_code_path));

    HttpResponse::Ok().body(sol_code)
}

#[post("/prove")]
async fn prove(data: web::Json<CreateEvmContractData>) -> impl Responder {
    let project_name = &data.project_name;
    let echo_data = &data.echo_data;
    let onnx_file_data = &data.onnx_file_data;

    // Create project directory
    let base_path = format!("./data/{}", project_name);
    if !Path::new(&base_path).exists() {
        fs::create_dir_all(&base_path).unwrap();
        println!("Created directory: {}", base_path);
    } else {
        println!("Directory already exists: {}", base_path);
    }

    // Save ONNX file and input JSON
    save_onnx_file(onnx_file_data, &base_path).unwrap();
    let input_json_path = format!("{}/input.json", base_path);
    let input_data_str = serde_json::to_string(echo_data).unwrap();
    store_json_data(&input_data_str, &input_json_path).unwrap();

    // Generate and save EVM config JSON
    let network_onnx_path = format!("{}/network.onnx", base_path);
    let vk_path = format!("{}/{}.vk", base_path, project_name);
    let proof_path = format!("{}/{}.pf", base_path, project_name);
    let evm_config_path = format!("{}/prove_{}.json", base_path, project_name);
    let evm_config_json = generate_evm_config_json(&input_json_path, &network_onnx_path, &vk_path, &proof_path);
    store_json_data(&evm_config_json, &evm_config_path).expect("Unable to write EVM config file");

    // Run the CLI command
    env::set_var("EZKLCONF", &evm_config_path);
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
    let res = run(cli_prove).await;

    // Retrieve proof data and output
    let proof_data = retrieve_proof_data(&proof_path).unwrap();
    let output = format!("{:?}", res);

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
