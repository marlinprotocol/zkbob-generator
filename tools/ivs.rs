use actix_web::http::StatusCode;
use actix_web::{get, post, web, Responder};
use actix_web::{App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use ethers::core::k256::ecdsa::{SigningKey, VerifyingKey};
use ethers::core::rand::thread_rng;
use ethers::core::utils::hex::FromHex;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Signature, SignatureError, H160};
use ethers::utils::keccak256;
use hex::decode;
use hex::encode;
use reqwest::Client;
use std::error::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Ivs pending");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port: u16 = 3030;

    let server = HttpServer::new(move || App::new().configure(routes))
        .client_request_timeout(Duration::new(0, 0))
        .bind(("0.0.0.0", port))
        .unwrap_or_else(|_| panic!("Can not bind to {}", &port))
        .run();

    log::info!("zkbob-generator start on port {}", port);
    server.await
    // implement POST /checkInput { input:bytes, secret: bytes } -> returns http_response 200/400
    // implement POST /checkEncryptedInput { input: bytes, encryptedSecret: bytes } -> returns http_response 200/400
}

// Routes
pub fn routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(test)
        .service(check_input_handler);
    conf.service(scope);
}

// Get generator status from the supervisord
#[get("/test")]
async fn test() -> impl Responder {
    zkbob_generator::response::response("The zkbob ivs is running!!", StatusCode::OK, None)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputPayload {
    pub public: String,
    pub secrets: Option<String>,
}

#[post("/checkInputs")]
async fn check_input_handler(payload: web::Json<InputPayload>) -> impl Responder {
    let public_inputs = hex::decode(payload.clone().public).unwrap().into();
    let private_inputs = hex::decode(payload.clone().secrets.unwrap()).unwrap();

    let result =
        zkbob_generator::verification::verify_zkbob_secret(&public_inputs, &private_inputs).await;
    if result.is_err() {
        return zkbob_generator::response::response(
            "invalid inputs!!",
            StatusCode::BAD_REQUEST,
            None,
        );
    }

    let result = result.unwrap();
    if result {
        return zkbob_generator::response::response("valid inputs.", StatusCode::OK, None);
    } else {
        return zkbob_generator::response::response(
            "invalid inputs!!",
            StatusCode::BAD_REQUEST,
            None,
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedInputPayload {
    pub public: String,
    pub encrypted_secrets: String,
}
#[post("/checkEncryptedInputs")]
async fn check_encrypted_input_handler(
    payload: web::Json<EncryptedInputPayload>,
) -> actix_web::Result<HttpResponse> {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DecryptRequest {
        market_id: String,
        private_input: String,
        acl: String,
        signature: String,
        ivs_pubkey: String,
    }

    let market_id = "1";
    let signature = sign_message_hash(market_id.to_string());
    let ivs_pubkey = "04dea308a0d2b8f80f2304818df0eaea9da11d082e58c5018e04d37662b7db2b051d9475505986ed42d099cecbe6f5e5d824ee272d979fae9803190340420dd399";

    let decrypt_request = DecryptRequest {
        market_id: market_id.to_string(),
        private_input: payload.clone().encrypted_secrets,
        acl: payload.clone().public,
        signature: signature,
        ivs_pubkey: ivs_pubkey.to_string(),
    };

    // Create a reqwest Client
    let client = Client::new();

    // Define the URL for the request
    let url = "http://localhost:3000/decryptRequest";
    // send request to matching engine `/decryptRequest from here`

    let response = client.post(url).json(&decrypt_request).send().await;

    if response.is_err() {
        return Ok(zkbob_generator::response::response(
            "Bad Request",
            StatusCode::BAD_REQUEST,
            None,
        ));
    }

    return Ok(zkbob_generator::response::response(
        "Valid Inputs",
        StatusCode::OK,
        None,
    ));
}

pub fn generate_key() {
    // Generate a random signing key (private key)
    let signing_key = SigningKey::random(&mut thread_rng());

    // Derive the verifying key (public key) from the signing key (private key)
    let verifying_key = VerifyingKey::from(&signing_key);

    // Print the private key in hexadecimal format
    let private_key_bytes = signing_key.to_bytes();
    println!("Private Key: 0x{}", encode(private_key_bytes));

    // Print the public key in uncompressed format
    let binding = verifying_key.to_encoded_point(false);
    let public_key_bytes = binding.as_bytes();
    println!("Public Key: 0x{}", encode(public_key_bytes));
}

pub fn sign_message_hash(market_id: String) -> String {
    // Generate a random wallet (private key)
    let private_key_hex = "890d9d749c8cec9babae73e6056ce54e9e96d0f3fc88afa93546a41813ba5ce0";

    // Initialize the wallet from the private key
    let wallet: LocalWallet = private_key_hex.parse().expect("Invalid private key");

    let ivs_pubkey = "04dea308a0d2b8f80f2304818df0eaea9da11d082e58c5018e04d37662b7db2b051d9475505986ed42d099cecbe6f5e5d824ee272d979fae9803190340420dd399";
    // Example message
    let message = market_id;

    // Create a Keccak-256 hash of the message
    let message_hash = keccak256(&message);

    let signature: Signature = wallet
        .sign_hash(ethers::types::TxHash(message_hash))
        .unwrap();

    signature_to_str(signature)
    // Concatenate the hexadecimal strings
    // let signature_hex = format!("{}{}{}", r_hex, s_hex, v_hex);
}

pub fn signature_to_str(signature: Signature) -> String {
    let r_hex = <[u8; 32]>::try_from(signature.r).unwrap();
    let s_hex = <[u8; 32]>::try_from(signature.s).unwrap();
    let v_hex = signature.v.to_string();

    let signature_hex = format!("{}{}{}", hex::encode(r_hex), hex::encode(s_hex), v_hex);
    signature_hex
}
