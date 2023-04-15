use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use ezkl_lib::{
    commands::{Cli, Commands, RunArgs, StrategyType, TranscriptType},
    execute::{run, ExecutionError},
};
use ezkl_lib::circuit::{CheckMode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fs::File, io::prelude::*, path::PathBuf};

use log::{info, LevelFilter};
use env_logger::Builder;

pub struct HunterZHunterRpc {}

const SERVER_ARGS: RunArgs = RunArgs {
    tolerance: 0,
    scale: 7,
    bits: 16,
    logrows: 17,
    public_inputs: false,
    public_outputs: true,
    public_params: false,
    check_mode: CheckMode::SAFE,
    pack_base: 1,
};

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

impl HunterZHunterRpc {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn forward(&self, input_data: Value) -> Result<Value, ExecutionError> {
        info!("Received forward request");

        let cli = Cli {
            command: Commands::Forward {
                data: "./data/eth_tokyo/input.json".to_string(),
                model: "./data/eth_tokyo/network.onnx".to_string(),
                output: "output.json".to_string(),
            },
            args: SERVER_ARGS,
        };

        env::set_var("EZKLCONF", "./data/forward.json");
        // let input_data_str = serde_json::to_string(&input_data).unwrap();
        // store_json_data(&input_data_str, "./data/eth_tokyo/input.json").unwrap();
        // run(cli).await.unwrap();
        let output = retrieve_json_data("output.json").unwrap();
        Ok(output)
    }

    pub async fn submit_proof(&self, input_data: Value) -> Result<bool, ExecutionError> {
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
        let input_data_str = serde_json::to_string(&input_data).unwrap();
        store_json_data(&input_data_str, "./data/eth_tokyo/input.json").unwrap();
        let output_data = input_data["output_data"].clone();

        let res = run(cli).await;
        print!("res: {:?}", res);

        Ok(true)
    }
}

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



#[get("/test")]
async fn test() -> impl Responder {
    // Modify the Cli struct with input variables
    let cli = Cli {
        command: Commands::Forward {
            data: "./data/eth_tokyo/input.json".to_string(),
            model: "./data/eth_tokyo/network.onnx".to_string(),
            output: "output.json".to_string(),
        },
        args: SERVER_ARGS,
    };

    info!("Cli: {:?}", cli);

    env::set_var("EZKLCONF", "./data/forward.json");
    // let input_data_str = serde_json::to_string(&input_data)?;
    // store_json_data(&input_data_str, "./data/eth_tokyo/input.json").unwrap();
    run(cli).await.unwrap();
    let output_str = retrieve_json_data("output.json").unwrap();
    let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    info!("Output: {:?}", output);

    HttpResponse::Ok().json(output) // Return the output as JSON
}




#[derive(Debug, Deserialize, Serialize)]
struct EchoData {
    input_data: Vec<Vec<f32>>,
    input_shapes: Vec<Vec<usize>>,
    output_data: Vec<Vec<f32>>,
}

#[post("/forward")]
async fn forward(input_data: web::Json<EchoData>) -> impl Responder {

    let cli = Cli {
        command: Commands::Forward {
            data: "./data/eth_tokyo/input.json".to_string(),
            model: "./data/eth_tokyo/network.onnx".to_string(),
            output: "./data/eth_tokyo/output.json".to_string(),
        },
        args: SERVER_ARGS,
    };

    info!("Cli: {:?}", cli);

    env::set_var("EZKLCONF", "./data/forward.json");
    let input_data_str = serde_json::to_string(&input_data.into_inner()).unwrap();

    info!("input_data_str: {:?}", input_data_str);

    store_json_data(&input_data_str, "./data/eth_tokyo/input.json").unwrap();

    run(cli).await.unwrap();
    let output_str = retrieve_json_data("./data/eth_tokyo/output.json").unwrap();
    let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    info!("Output: {:?}", output);

    HttpResponse::Ok().json(output)
}



#[post("/submit_proof")]
async fn submit_proof(input_data: web::Json<EchoData>) -> impl Responder {
    let cli = Cli {
        command: Commands::Prove {
            data: "./data/eth_tokyo/input.json".to_string(),
            model: PathBuf::from("./data/eth_tokyo/network.onnx"),
            vk_path: PathBuf::from("./data/eth_tokyo/eth_tokyo.vk"),
            proof_path: PathBuf::from("./data/eth_tokyo/eth_tokyo.pf"),
            params_path: PathBuf::from("kzg.params"),
            transcript: TranscriptType::Blake,
            strategy: StrategyType::Single,
        },
        args: SERVER_ARGS,
    };
    info!("1.Cli: {:?}", cli);

    env::set_var("EZKLCONF", "./data/submit_proof.json");
    let input_data_str = serde_json::to_string(&input_data.input_data).unwrap();
    store_json_data(&input_data_str, "./data/eth_tokyo/input.json").unwrap();

    info!("2.Input_data_str: {:?}", input_data_str);
    let res = run(cli).await;

    info!("3.Res: {:?}", res);
    let output_str = retrieve_json_data("./data/eth_tokyo/output.json").unwrap();
    let output: JsonRpcParams = serde_json::from_str(&output_str.to_string()).unwrap();
    info!("4.Output: {:?}", output);

    HttpResponse::Ok().json(output)
}


pub async fn run_server() -> std::io::Result<()> {
    let addr = "0.0.0.0:3030";
    let rpc = web::Data::new(HunterZHunterRpc::new());

        // Initialize the logger
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();

    // Your server initialization code here

    info!("Server started on http://0.0.0.0:3030");

    HttpServer::new(move || {
        App::new()
            .app_data(rpc.clone())
            .service(forward)
            .service(submit_proof)
            .service(test)
    })
    .bind(addr)?
    .run()
    .await
}
