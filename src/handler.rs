use crate::model::GenerateProofInputs;
use crate::response::response;
use crate::zkbob_generator;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Responder};
use serde_json::Value;

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    response("Hi there!!", StatusCode::OK, None)
}

#[post("/generateProof")]
async fn generate_proof(jsonbody: web::Json<GenerateProofInputs>) -> impl Responder {
    let zkbob_inputs = jsonbody.0;
    let generate_zkbob_proof = zkbob_generator::zkbob_generator(
        zkbob_inputs.ask,
        zkbob_inputs.private_input,
        zkbob_inputs.ask_id,
    )
    .await;

    if generate_zkbob_proof.is_err() {
        return response(
            "There was an issue while generating the proof",
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    let zkbob_proof_response = generate_zkbob_proof.unwrap();

    if !zkbob_proof_response.verification_status && zkbob_proof_response.signature.is_none() {
        return response(
            "There was an issue while generating the proof",
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    else if !zkbob_proof_response.verification_status && zkbob_proof_response.signature.is_some() {
        return response(
            "Inputs were invalid, signature genereated for invalid input.",
            StatusCode::BAD_REQUEST,
            Some(Value::String(zkbob_proof_response.signature.unwrap())),
        );
    }

    response(
        "Proof generated",
        StatusCode::OK,
        Some(Value::String(
            zkbob_proof_response.proof.unwrap().to_string(),
        )),
    )
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(test).service(generate_proof);
    conf.service(scope);
}
