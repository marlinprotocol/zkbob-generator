use bindings::shared_types::Ask;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct GenerateProofInputs {
    pub ask: Ask,
    pub private_input: Vec<u8>,
    pub ask_id: u64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TestInput {
    pub private_input: Vec<u8>,
    pub public_input: ethers::types::Bytes,
}
