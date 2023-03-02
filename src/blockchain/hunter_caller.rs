use ethers::{
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    solc::Solc,
};
use eyre::Result;
use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration};


pub struct VerifyandAwardParams {
    huntId: &String,
    winner: &String,
    proof: &String
}

impl VerifyandAwardParams {
    pub fn new(huntId: &String, winner: &String, proof: &String) -> Self {
        Self {
            huntId,
            winner,
            proof
        }
    }
}

// Generate the type-safe contract bindings by providing the ABI
// definition
abigen!(
    HunterZHunter,
    "./hunter.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main(params: VerifyandAwardParams) -> Result<()> {
    use ethers::prelude::*;
    use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration};

    const ALCHEMY_URL: &str = "https://polygon-mumbai.g.alchemy.com/v2/E14Alon0FRdGqDecTOYC9O0qZZ6yI3N3";
    // compile contract and launch anvil
    let anvil = Anvil::new().spawn();

    // set path to the contract
    let source = Path::new(&env!("CARGO_MANIFEST_DIR")).join("./hunterzhunter.sol");
    let compiled = Solc::default().compile_source(source).expect("Could not compile contracts");
    let (abi, bytecode, _runtime_bytecode) =
        compiled.find("HunterZHunter").expect("could not find contract").into_parts_or_default();

    // get our key
    let key = ethers::core::utils::polygon::get_private_key_from_env().unwrap();

    // create a wallet from the key
    let wallet: LocalWallet = LocalWallet::from(key).with_chain_id(Chain::Mumbai);

    // connect to the provider
    let provider = Provider::<Http>::try_from(ALCHEMY_URL)?
        .interval(Duration::from_secs(1));

    // instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // get the contract address
    let addr = "0x4dFcFC0AC16ca7Cb7402639E5a2098FF7d5322ec".parse().unwrap();

    // create an instance of the contract
    let contract = HunterZHunter::new(addr, client.clone());

    // call the verifyAndAward function with the struct params
    let _award = contract.verifyAndAwardPrize(params.huntId, params.winner, params.proof).await?;

    // getting events
    let logs = contract.query_filter().from_block(32605822).query().await?;

    // print logs to be sure hunter was rewarded
    println!("Logs: {}", serde_json::to_string(&logs)?);

    Ok(())
}
