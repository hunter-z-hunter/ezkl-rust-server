use rpc_server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    run_server().await
}
