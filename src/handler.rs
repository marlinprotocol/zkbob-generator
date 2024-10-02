use crate::model;
use crate::response::response;
use crate::{verification, zkbob_generator};
use actix_web::web::Data;
use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use ethers::{
    core::k256::ecdsa::SigningKey,
    signers::{LocalWallet, Signer, Wallet},
};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::{fs, str::FromStr};

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    response(
        "The zkbob generator is running!!",
        StatusCode::OK,
        Some("Zkbob Prover is running!".into()),
    )
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
    let test_inputs: model::TestInput = match serde_json::from_str(&file_content.unwrap()) {
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
        return HttpResponse::ExpectationFailed().json(
            kalypso_generator_models::models::BenchmarkResponse {
                data: "Failed".to_string(),
                time_in_ms: 0,
            },
        );
    }

    let time_taken_to_generate_proof_in_ms =
        benchmark_proof_generation.unwrap().proof_generation_time;

    return HttpResponse::Ok().json(kalypso_generator_models::models::BenchmarkResponse {
        data: "Success".to_string(),
        time_in_ms: time_taken_to_generate_proof_in_ms,
    });
}

#[post("/generateProof")]
async fn generate_proof(
    payload: web::Json<kalypso_generator_models::models::InputPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let zkbob_inputs = payload.0;
    let enclave_key = { ecies_priv_key.lock().unwrap().clone() };
    log::info!("Request received by the zkb generator");
    let generate_zkbob_proof = zkbob_generator::zkbob_generator(zkbob_inputs, enclave_key).await;

    match generate_zkbob_proof {
        Ok(prove) => {
            if prove.proof.is_some() && prove.signature.is_some() {
                let public_inputs = prove.inputs.unwrap();
                let proof_bytes = prove.proof.unwrap();
                let signature = prove.signature.unwrap();
                let sig_bytes = ethers::types::Bytes::from_str(&signature).unwrap();
                let value = vec![
                    ethers::abi::Token::Bytes(public_inputs.to_vec()),
                    ethers::abi::Token::Bytes(proof_bytes.to_vec()),
                    ethers::abi::Token::Bytes(sig_bytes.to_vec()),
                ];
                let encoded = ethers::abi::encode(&value);
                return Ok(HttpResponse::Ok().json(
                    kalypso_generator_models::models::GenerateProofResponse {
                        proof: encoded.to_vec(),
                    },
                ));
            } else if prove.proof.is_none() && prove.signature.is_some() {
                let signature = prove.signature.unwrap();
                return Ok(response(
                    "Invalid inputs received, signature generated",
                    StatusCode::BAD_REQUEST,
                    Some(Value::String(signature)),
                ));
            } else {
                return Ok(response(
                    "There was an issue while generating the proof.",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                ));
            }
        }
        Err(e) => Err(e),
    }
}

#[post("/checkInput")]
async fn check_input_handler(
    payload: web::Json<kalypso_generator_models::models::InputPayload>,
) -> impl Responder {
    let default_response = kalypso_ivs_models::models::CheckInputResponse { valid: false };
    let private_input = match payload.clone().get_plain_secrets() {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::NotImplemented().json(default_response);
        }
    };

    let public_input = payload.clone().get_public();
    let public_input_bytes = ethers::types::Bytes::from(public_input);

    let verification = verification::verify_zkbob_secret(&public_input_bytes, &private_input).await;

    match verification {
        Ok(verify) => {
            if verify {
                return HttpResponse::Ok()
                    .json(kalypso_ivs_models::models::CheckInputResponse { valid: true });
            } else {
                return HttpResponse::Ok().json(default_response);
            }
        }
        Err(_) => {
            return HttpResponse::Ok().json(default_response);
        }
    }
}

#[post("/getAttestationForInvalidInputs")]
async fn get_attestation_for_invalid_inputs(
    payload: web::Json<kalypso_ivs_models::models::InvalidInputPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
    let signer_wallet = get_signer(ecies_priv_key);
    let default_response = kalypso_ivs_models::models::CheckInputResponse { valid: false };

    let private_input = match payload.clone().get_plain_secrets() {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::NotImplemented().json(default_response);
        }
    };

    let public_input = payload.clone().get_public();
    let public_input_bytes = ethers::types::Bytes::from(public_input);

    let verification = verification::verify_zkbob_secret(&public_input_bytes, &private_input).await;

    match verification {
        Ok(verify) => {
            if verify {
                return HttpResponse::Ok()
                    .json(kalypso_ivs_models::models::CheckInputResponse { valid: true });
            } else {
                return HttpResponse::Ok()
                    .json(generate_invalid_input_attestation(payload.0, signer_wallet).await);
            }
        }
        Err(_) => {
            return HttpResponse::Ok()
                .json(generate_invalid_input_attestation(payload.0, signer_wallet).await);
        }
    }
}

#[post("/checkEncryptedInputs")]
async fn check_encrypted_input(
    payload: web::Json<kalypso_ivs_models::models::EncryptedInputPayload>,
    ecies_priv_key: Data<Arc<Mutex<Vec<u8>>>>,
) -> impl Responder {
    let payload = payload.0;
    // log::info!("{:?}", &payload);
    let default_response = kalypso_ivs_models::models::CheckInputResponse { valid: false };

    let (signature, ivs_pub_key) = {
        let message = &payload.market_id;
        let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
        let signer_wallet = get_signer(ecies_priv_key);
        let digest = ethers::utils::keccak256(message.as_bytes());

        let read_secp_pub_key = match fs::read("./app/secp.pub") {
            Ok(data) => data,
            Err(_) => {
                return response(
                    "There was an issue while reading the secp public key.",
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                );
            }
        };
        let mut modified_secp_pub_key = vec![0x04];
        modified_secp_pub_key.extend_from_slice(&read_secp_pub_key);
        let signature = signer_wallet
            .sign_hash(ethers::types::H256(digest))
            .expect("Failed signing market id for check encrypted inputs");
        (signature.to_string(), modified_secp_pub_key)
    };
    let decrypt_request_payload = kalypso_matching_engine_models::models::DecryptRequest {
        market_id: payload.clone().market_id.to_string(),
        private_input: hex::encode(payload.clone().encrypted_secrets),
        acl: hex::encode(payload.clone().acl),
        signature,
        ivs_pubkey: hex::encode(ivs_pub_key),
    };

    let client = reqwest::Client::new();
    let api_response = client
        .post(&payload.me_decryption_url)
        .json(&decrypt_request_payload)
        .send()
        .await
        .unwrap();
    println!("Handler, me decrypt response: {:?}", api_response);

    if api_response.status().is_success() {
        let response_payload: kalypso_matching_engine_models::models::GetRequestResponse =
            match api_response.json().await {
                Ok(data) => data,
                Err(err) => {
                    dbg!(err);
                    return response(
                        "Unable to get response from matching engine",
                        StatusCode::EXPECTATION_FAILED,
                        None,
                    );
                }
            };

        let encrypted_data = hex::decode(response_payload.encrypted_data).unwrap();
        let ecies_priv_key = { ecies_priv_key.lock().unwrap().clone() };
        let decrypted_data =
            kalypso_helper::secret_inputs_helpers::decrypt_ecies(&ecies_priv_key, &encrypted_data)
                .unwrap();

        let decrypted_secret = {
            let decrypted_secret = String::from_utf8(decrypted_data.clone());
            if decrypted_secret.is_ok() {
                decrypted_secret.unwrap()
            } else {
                let decompreseed_decrypted_data =
                    kalypso_helper::secret_inputs_helpers::flatten(&decrypted_data).unwrap();
                String::from_utf8(decompreseed_decrypted_data).unwrap()
            }
        };

        let public_input = payload.clone().public_inputs.unwrap();
        let public_input_bytes = ethers::types::Bytes::from(public_input);

        let verification = verification::verify_zkbob_secret(
            &public_input_bytes,
            &decrypted_secret.as_bytes().to_vec(),
        )
        .await;

        match verification {
            Ok(verify) => {
                if verify {
                    return HttpResponse::Ok()
                        .json(kalypso_ivs_models::models::CheckInputResponse { valid: true });
                } else {
                    return HttpResponse::Ok().json(default_response);
                }
            }
            Err(_) => {
                return HttpResponse::Ok().json(default_response);
            }
        }
    } else {
        response(
            "Could not fetch info from matching engine",
            StatusCode::FAILED_DEPENDENCY,
            None,
        )
    }
}

#[post("/verifyInputsAndProof")]
async fn verify_inputs_and_proof(
    payload: web::Json<kalypso_ivs_models::models::VerifyInputsAndProof>,
) -> impl Responder {
    let default_response = kalypso_ivs_models::models::VerifyInputAndProofResponse {
        is_input_and_proof_valid: false,
    };

    let verification = verification::verify_zkbob_inputs_and_proof(payload.0).await;

    match verification {
        Ok(verify) => {
            if verify {
                return HttpResponse::Ok()
                    .json(kalypso_ivs_models::models::CheckInputResponse { valid: true });
            } else {
                return HttpResponse::Ok().json(default_response);
            }
        }
        Err(_) => {
            return HttpResponse::Ok().json(default_response);
        }
    }
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(benchmark)
        .service(generate_proof)
        .service(check_input_handler)
        .service(get_attestation_for_invalid_inputs)
        .service(check_encrypted_input)
        .service(verify_inputs_and_proof);
    conf.service(scope);
}

async fn generate_invalid_input_attestation(
    payload: kalypso_ivs_models::models::InvalidInputPayload,
    signer_wallet: Wallet<SigningKey>,
) -> kalypso_generator_models::models::GenerateProofResponse {
    let ask_id = payload.only_ask_id();
    let value = vec![
        ethers::abi::Token::Uint(ask_id.into()),
        ethers::abi::Token::Bytes(payload.get_public()),
    ];
    let encoded = ethers::abi::encode(&value);
    let digest = ethers::utils::keccak256(encoded);

    let signature = signer_wallet
        .sign_message(ethers::types::H256(digest))
        .await
        .unwrap();

    let response = kalypso_generator_models::models::GenerateProofResponse {
        proof: signature.to_vec(),
    };
    return response;
}

fn get_signer(ecies_priv_key: Vec<u8>) -> Wallet<SigningKey> {
    let secp_private_key = secp256k1::SecretKey::from_slice(&ecies_priv_key)
        .unwrap()
        .display_secret()
        .to_string();
    secp_private_key.parse::<LocalWallet>().unwrap()
}
