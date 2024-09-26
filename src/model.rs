// use bindings::shared_types::Ask;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum InputError {
    #[display(fmt = "input decode failed due to invalid structure")]
    DecodeFailed,

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
            InputError::DecodeFailed => StatusCode::NOT_ACCEPTABLE,
            InputError::InvalidInputPayload => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TestInput {
    pub private_input: Vec<u8>,
    pub public_input: ethers::types::Bytes,
}
