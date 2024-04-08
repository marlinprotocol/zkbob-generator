use crate::model::{GenerateProofInputs, TestInput};
use crate::response::response;
use crate::zkbob_generator;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, Responder};
use serde_json::Value;
use std::fs;

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    response("The zkbob generator is running!!", StatusCode::OK, None)
}

// Get generator status from the supervisord
#[get("/benchmark")]
async fn benchmark() -> impl Responder {
    let test_inputs_path = "./app/test_inputs.json".to_string();
    let alt_test_inputs_path = "../app/test_inputs.json".to_string();
    let file_content =
        fs::read_to_string(test_inputs_path).or_else(|_| fs::read_to_string(alt_test_inputs_path));
    if file_content.is_err() {
        log::error!("{:#?}", file_content.err());
        return response(
            "There was an issue while reading the test_inputs.json, please make sure you provide a valid input file.",
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }
    let test_inputs: TestInput = match serde_json::from_str(&file_content.unwrap()) {
        Ok(input) => input,
        Err(err) => {
            log::error!("{}", err);
            return response(
                "Invalid test_inputs.json format",
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
            );
        }
    };

    let benchmark_proof_generation = zkbob_generator::benchmark_proof_generation(
        test_inputs.private_input,
        test_inputs.public_input,
    );

    if benchmark_proof_generation.is_err() {
        return response(
            "There was an issue benchmarking the proof generation time.",
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    let time_taken_to_generate_proof_in_ms = benchmark_proof_generation
        .unwrap()
        .proof_generation_time
        .to_string();

    response(
        "Proof generated, the proof generation time returned is in milliseconds",
        StatusCode::OK,
        Some(Value::String(time_taken_to_generate_proof_in_ms)),
    )
}

#[post("/generateProof")]
async fn generate_proof(jsonbody: web::Json<GenerateProofInputs>) -> impl Responder {
    let zkbob_inputs = jsonbody.0;
    log::info!(
        "Request received by the zkb generator for ask ID : {}",
        zkbob_inputs.ask_id
    );
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
    } else if !zkbob_proof_response.verification_status && zkbob_proof_response.signature.is_some()
    {
        return response(
            "Inputs were invalid, signature genereated for invalid input.",
            StatusCode::BAD_REQUEST,
            Some(Value::String(zkbob_proof_response.signature.unwrap())),
        );
    }

    log::info!(
        "Proof generated by the zkb generator for ask ID : {}",
        zkbob_inputs.ask_id
    );
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
    let scope = web::scope("/api")
        .service(test)
        .service(benchmark)
        .service(generate_proof);
    conf.service(scope);
}
