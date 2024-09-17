// use bindings::shared_types::Ask;
use serde::{Deserialize, Serialize};
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum InputError {
    #[display(fmt = "file not found")]
    FileNotFound,

    #[display(fmt = "incorrect config")]
    BadConfigData,

    #[display(fmt = "decryption fails due to incorrect keys")]
    DecryptionFailed,

    #[display(fmt = "invalid market")]
    InvalidMarket,

    #[display(fmt = "payload not valid")]
    InvalidInputPayload,
}

impl error::ResponseError for InputError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            InputError::FileNotFound => StatusCode::NOT_FOUND,
            InputError::BadConfigData => StatusCode::NOT_ACCEPTABLE,
            InputError::DecryptionFailed => StatusCode::MISDIRECTED_REQUEST,
            InputError::InvalidMarket => StatusCode::NOT_IMPLEMENTED,
            InputError::InvalidInputPayload => StatusCode::BAD_REQUEST,
        }
    }
}

// #[derive(Serialize, Debug, Deserialize)]
// pub struct GenerateProofInputs {
//     pub ask: Ask,
//     pub private_input: Vec<u8>,
//     pub ask_id: u64,
// }

#[derive(Serialize, Debug, Deserialize)]
pub struct TestInput {
    pub private_input: Vec<u8>,
    pub public_input: ethers::types::Bytes,
}
