use reqwest;
mod hunter_z_hunter_rpc;

pub mod request{
    pub struct HuntData {
        huntId: String,
        proof: Result<String, Error>,
    }
    
    pub static HUNT_DATA_VEC: &[HuntData];
    
    impl PostData for HuntData {
        fn new(huntId: String, proof: Value) -> Self {
            let data = Self { huntId, proof };
            HUNT_DATA_VEC.push(data.clone());
    
            let client = reqwest::Client::new();
            let res = client
                .post(String::from(endpoint_url))
                .body(format!("huntId: {}, proof: {}", data.huntId, data.proof.unwrap()))
                .send()
                .await
                .unwrap();
    
            data
        }
    }
    
}
