use bindings::shared_types::Ask;
use ethers::signers::{LocalWallet, Signer};
use libzeropool_zkbob::fawkes_crypto::engines::bn256::Fr;
use libzeropool_zkbob::{
    circuit::tx::{c_transfer, CTransferPub, CTransferSec},
    fawkes_crypto::{
        backend::bellman_groth16::Parameters,
        backend::bellman_groth16::{engines::Bn256, prover, verifier},
        circuit::cs::CS,
    },
    native::tx::{TransferPub, TransferSec},
    POOL_PARAMS,
};
use serde::Deserialize;
use tokio::fs;

use std::str;
use std::time::Instant;

use ethers::abi::{decode, encode, ParamType, Token};
use ethers::types::Bytes;
use ethers::types::U256;

use serde_json::{self, json};

use crate::verification::verify_zkbob_secret;

#[derive(Debug, Deserialize)]
struct ProofRaw {
    a: [String; 2],
    b: [[String; 2]; 2],
    c: [String; 2],
}

#[derive(Debug)]
struct Proof {
    a: [U256; 2],
    b: [[U256; 2]; 2],
    c: [U256; 2],
}

pub struct GenerateZkbobProof {
    pub proof: Option<ethers::types::Bytes>,
    pub verification_status: bool,
    pub signature: Option<String>,
}

pub struct BenchmarkProofGenerationResponse {
    pub proof_generation_time: u128,
}

impl From<ProofRaw> for Proof {
    fn from(raw: ProofRaw) -> Self {
        Self {
            a: [
                U256::from_dec_str(&raw.a[0]).expect("Failed to parse a[0]"),
                U256::from_dec_str(&raw.a[1]).expect("Failed to parse a[1]"),
            ],
            b: [
                [
                    U256::from_dec_str(&raw.b[0][0]).expect("Failed to parse b[0][0]"),
                    U256::from_dec_str(&raw.b[0][1]).expect("Failed to parse b[0][1]"),
                ],
                [
                    U256::from_dec_str(&raw.b[1][0]).expect("Failed to parse b[1][0]"),
                    U256::from_dec_str(&raw.b[1][1]).expect("Failed to parse b[1][1]"),
                ],
            ],
            c: [
                U256::from_dec_str(&raw.c[0]).expect("Failed to parse c[0]"),
                U256::from_dec_str(&raw.c[1]).expect("Failed to parse c[1]"),
            ],
        }
    }
}

fn param_gen(transfer_params_path: &str) -> Parameters<Bn256> {
    let param_file = std::fs::read(transfer_params_path).unwrap();
    let mut param_format: &[u8] = &param_file;
    let params: Parameters<Bn256> = Parameters::read(&mut param_format, true, true).unwrap();
    params
}

fn decode_input(
    encoded_input: Bytes,
) -> Result<[ethers::types::U256; 5], Box<dyn std::error::Error>> {
    let param_types = vec![ParamType::FixedArray(Box::new(ParamType::Uint(256)), 5)];
    let tokens = decode(&param_types, &encoded_input.0)?;
    if let Some(ethers::abi::Token::FixedArray(arr)) = tokens.first() {
        if arr.len() == 5 {
            let mut output = [U256::zero(); 5];
            for (i, token) in arr.iter().enumerate() {
                if let ethers::abi::Token::Uint(u) = token {
                    output[i] = *u;
                } else {
                    return Err("Expected a U256 inside the FixedArray".into());
                }
            }
            Ok(output)
        } else {
            Err("Unexpected number of decoded tokens inside the FixedArray".into())
        }
    } else {
        Err("Unexpected decoded token type".into())
    }
}

pub async fn invalid_input_response(ask_id: u64, public_inputs: Bytes) -> GenerateZkbobProof {
    let read_secp_private_key = fs::read("/app/secp.sec").await.unwrap();
    let secp_private_key = secp256k1::SecretKey::from_slice(&read_secp_private_key)
        .unwrap()
        .display_secret()
        .to_string();
    let signer_wallet = secp_private_key.parse::<LocalWallet>().unwrap();
    let value = vec![
        ethers::abi::Token::Uint(ask_id.into()),
        ethers::abi::Token::Bytes(public_inputs.to_vec()),
    ];
    let encoded = ethers::abi::encode(&value);
    let digest = ethers::utils::keccak256(encoded);

    let signature = signer_wallet
        .sign_message(ethers::types::H256(digest))
        .await
        .unwrap();

    let output = GenerateZkbobProof {
        proof: None,
        verification_status: false,
        signature: Some("0x".to_owned() + &signature.to_string()),
    };

    return output;
}

pub async fn zkbob_generator(
    ask: Ask,
    private_input: Vec<u8>,
    ask_id: u64,
) -> Result<GenerateZkbobProof, Box<dyn std::error::Error>> {
    let public_inputs = ask.prover_data;

    let are_inputs_valid = verify_zkbob_secret(&public_inputs, &private_input).await;
    if are_inputs_valid.is_err() {
        return Ok(invalid_input_response(ask_id.clone(), public_inputs.clone()).await);
    }

    let are_inputs_valid = are_inputs_valid.unwrap();

    if !are_inputs_valid {
        return Ok(invalid_input_response(ask_id.clone(), public_inputs.clone()).await);
    }

    let generate_proof_response = generate_zkbob_proof(
        private_input,
        public_inputs,
        "./params/transfer_params_prod.bin",
    )
    .unwrap();
    log::info!("proof: {:?}", generate_proof_response.proof);

    Ok(generate_proof_response)
}

fn circuit<C: CS<Fr = Fr>>(public: CTransferPub<C>, secret: CTransferSec<C>) {
    c_transfer(&public, &secret, &*POOL_PARAMS);
}

//Generate Proof
fn generate_zkbob_proof(
    secret_bytes: Vec<u8>,
    public_inputs_bytes: ethers::types::Bytes,
    transfer_params_path: &str,
) -> Result<GenerateZkbobProof, Box<dyn std::error::Error>> {
    let params = param_gen(transfer_params_path);
    log::info!("Proof generation started...");
    let ts_prove = Instant::now();

    let secret = String::from_utf8(secret_bytes).unwrap();
    let secret: TransferSec<Fr> = serde_json::from_str(&secret).unwrap();
    let public = decode_input(public_inputs_bytes).unwrap();

    let data = json!({
        "root": public[0].to_string(),
        "nullifier": public[1].to_string(),
        "out_commit": public[2].to_string(),
        "delta": public[3].to_string(),
        "memo": public[4].to_string()
    });

    let public: TransferPub<Fr> = serde_json::from_value(data).unwrap();
    let (inputs, snark_proof) = prover::prove(&params, &public, &secret, circuit);

    let duration = ts_prove.elapsed();
    log::info!("Proof generation time: {:?}", duration);
    let proof_string = serde_json::to_string(&snark_proof).unwrap();
    let res = verifier::verify(&params.get_vk(), &snark_proof, &inputs);
    log::info!("Proof verification status : {:?}", res);

    let raw_proof: ProofRaw = serde_json::from_str(&proof_string).expect("Failed to parse JSON");

    let proof: Proof = raw_proof.into();

    let tokens = Token::FixedArray(vec![
        Token::Uint(proof.a[0]),
        Token::Uint(proof.a[1]),
        Token::Uint(proof.b[0][0]),
        Token::Uint(proof.b[0][1]),
        Token::Uint(proof.b[1][0]),
        Token::Uint(proof.b[1][1]),
        Token::Uint(proof.c[0]),
        Token::Uint(proof.c[1]),
    ]);

    let encoded_data = encode(&[tokens]);

    let output = GenerateZkbobProof {
        proof: Some(encoded_data.into()),
        verification_status: res,
        signature: None,
    };

    Ok(output)
}

pub fn benchmark_proof_generation(
    secret_bytes: Vec<u8>,
    public_inputs_bytes: ethers::types::Bytes,
) -> Result<BenchmarkProofGenerationResponse, Box<dyn std::error::Error>> {
    let params = param_gen("./params/transfer_params_prod.bin");
    log::info!("Proof generation started...");
    let ts_prove = Instant::now();

    let secret = String::from_utf8(secret_bytes).unwrap();
    let secret: TransferSec<Fr> = serde_json::from_str(&secret).unwrap();
    let public = decode_input(public_inputs_bytes).unwrap();

    let data = json!({
        "root": public[0].to_string(),
        "nullifier": public[1].to_string(),
        "out_commit": public[2].to_string(),
        "delta": public[3].to_string(),
        "memo": public[4].to_string()
    });

    let public: TransferPub<Fr> = serde_json::from_value(data).unwrap();
    let (inputs, snark_proof) = prover::prove(&params, &public, &secret, circuit);

    let duration = ts_prove.elapsed();
    log::info!("Proof generation time: {:?}ms", duration.as_millis());
    let res = verifier::verify(&params.get_vk(), &snark_proof, &inputs);
    log::info!("Proof verification status : {:?}", res);

    let output = BenchmarkProofGenerationResponse {
        proof_generation_time: duration.as_millis(),
    };

    Ok(output)
}
