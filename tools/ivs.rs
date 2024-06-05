use actix_web::http::StatusCode;
use actix_web::{get, post, web, Responder};
use actix_web::{App, HttpResponse, HttpServer};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use secret_inputs_helpers::try_decrypt;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use ethers::core::k256::ecdsa::{SigningKey, VerifyingKey};
use ethers::core::rand::thread_rng;
use ethers::core::utils::hex::FromHex;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Bytes, Signature, SignatureError, H160, U256};
use ethers::utils::keccak256;
use hex::decode;
use hex::encode;
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

mod secret_inputs_helpers;

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
    let ivs_pubkey = read_private_key_and_generate_public();

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

    let response: Result<reqwest::Response, reqwest::Error> =
        client.post(url).json(&decrypt_request).send().await;
    let mut body: String;

    match response {
        Ok(res) => {
            body = res.text().await.unwrap();
            println!("Response body: {}", body);
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            return Ok(zkbob_generator::response::response(
                "Request failed",
                StatusCode::BAD_REQUEST,
                None,
            ));
        }
    }

    let mut file = File::open("./app.secp").expect("file not found");
    let mut private_key_hex = String::new();
    file.read_to_string(&mut private_key_hex);

    // Step 2: Convert the hexadecimal private key to bytes
    let private_key_bytes = hex::decode(private_key_hex.trim()).expect("Invalid hex string");
    let market_id_U256 = U256::from_str(market_id).unwrap();
    let market_id_bytes = hex::decode(market_id.trim()).expect("Invalid hex string");
    let encrypted_data = body;
    let encrypted_data_bytes = hex::decode(encrypted_data.trim()).expect("Invalid hex string");

    let decrypted = try_decrypt(&encrypted_data_bytes, &private_key_bytes, market_id_U256).unwrap();
    let bytes: Bytes = Bytes::from(market_id_bytes);

    let result = zkbob_generator::verification::verify_zkbob_secret(&bytes, &decrypted).await;
    if result.is_err() {
        return Ok(zkbob_generator::response::response(
            "invalid inputs!!",
            StatusCode::BAD_REQUEST,
            None,
        ));
    }

    let result = result.unwrap();
    if result {
        return Ok(zkbob_generator::response::response(
            "valid inputs.",
            StatusCode::OK,
            None,
        ));
    } else {
        return Ok(zkbob_generator::response::response(
            "invalid inputs!!",
            StatusCode::BAD_REQUEST,
            None,
        ));
    }
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

pub fn read_private_key_and_generate_public() -> String {
    // Step 1: Read the private key from the file
    let mut file = File::open("./app.secp").expect("file not found");
    let mut private_key_hex = String::new();
    file.read_to_string(&mut private_key_hex);

    // Step 2: Convert the hexadecimal private key to bytes
    let private_key_bytes = hex::decode(private_key_hex.trim()).expect("Invalid hex string");

    // Step 3: Generate the public key using secp256k1
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&private_key_bytes).expect("32 bytes, within curve order");
    let pk = PublicKey::from_secret_key(&secp, &sk);

    // Convert the public key to uncompressed and compressed formats
    let uncompressed_public_key = hex::encode(pk.serialize_uncompressed());
    uncompressed_public_key
}

pub fn sign_message_hash(market_id: String) -> String {
    // Generate a random wallet (private key)
    let mut file = File::open("./app.secp").expect("file not found");
    let mut private_key_hex = String::new();
    file.read_to_string(&mut private_key_hex);

    // Initialize the wallet from the private key
    let wallet: LocalWallet = private_key_hex.parse().expect("Invalid private key");

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
