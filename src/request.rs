use reqwest;
use std::io::prelude::*;
use thiserror::Error;
// mod hunter_z_hunter_rpc;

pub mod request {
    // here we'd pass in the proof
    use std::fs::File;
    use std::io::Read;

    pub async fn postData() -> bool {
        const ENDPOINT_URL: &str = "http://localhost:3000/api/proof";
        let mut file = File::open("./data/sol_calldata.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut mockProof = File::open("./data/sol_calldata.json").unwrap();
        let client = reqwest::Client::new();
        let res = client
            .post(String::from(ENDPOINT_URL))
            .body(contents)
            .send()
            .await
            .unwrap();
        res.status().is_success()
    }
}
