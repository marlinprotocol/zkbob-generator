use crate::model;
use crate::zkbob_generator;
use actix_web::error::Error;
use ethers::abi::{decode, ParamType};
use ethers::types::Bytes;

use libzeropool_zkbob::{
    fawkes_crypto::{
        backend::bellman_groth16::{engines::Bn256, prover::Proof, verifier, Parameters},
        core::sizedvec::SizedVec,
        engines::bn256::Fr,
        ff_uint::Num,
        native::poseidon::poseidon_merkle_proof_root,
    },
    native::{
        key,
        params::PoolParams,
        tx::{self, TransferPub, TransferSec},
    },
    POOL_PARAMS,
};
use serde_json::json;

fn into_zkbob_secret(decoded_secret: String) -> Result<TransferSec<Fr>, model::InputError> {
    let decoded_secret_bytes = hex::decode(decoded_secret).unwrap();
    let secret_string = String::from_utf8(decoded_secret_bytes).unwrap();
    let secret_value = serde_json::from_str(&secret_string);
    let secret;
    if secret_value.is_err() {
        return Err(model::InputError::InvalidInputPayload);
    } else {
        secret = secret_value.unwrap();
    }
    let zkbob_secret: TransferSec<Fr> = serde_json::from_value(secret).unwrap();

    Ok(zkbob_secret)
}

fn into_zkbob_pub_input(decoded_pub_input: String) -> Result<TransferPub<Fr>, Error> {
    use ethers::prelude::*;

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

    let decoded_pub_input_bytes = hex::decode(decoded_pub_input).unwrap();
    let public = decode_input(decoded_pub_input_bytes.into()).unwrap();
    let public_value = json!({
        "root": public[0].to_string(),
        "nullifier": public[1].to_string(),
        "out_commit": public[2].to_string(),
        "delta": public[3].to_string(),
        "memo": public[4].to_string()
    });

    let zkbob_pub_input: TransferPub<Fr> = serde_json::from_value(public_value).unwrap();

    Ok(zkbob_pub_input)
}

pub async fn verify_zkbob_secret(
    public_input: &Bytes,
    private_inputs: &Vec<u8>,
) -> Result<bool, Error> {
    let mut result = false;
    let pub_input = hex::encode(public_input);
    let sec_input = hex::encode(private_inputs);
    let zkbob_public = into_zkbob_pub_input(pub_input.to_string()).unwrap();
    let zkbob_secret = match into_zkbob_secret(sec_input.to_string()) {
        Ok(data) => data,
        Err(e) => return Err(e.into()),
    };

    // calculating output hashes
    let out_account_hash = zkbob_secret.tx.output.0.hash(&POOL_PARAMS.clone());
    let out_note_hash = zkbob_secret
        .tx
        .output
        .1
        .iter()
        .map(|e| e.hash(&POOL_PARAMS.clone()))
        .collect::<Vec<_>>();
    let out_hash = [[out_account_hash].as_ref(), out_note_hash.as_slice()].concat();

    // calculating input hashes
    let in_account_hash = zkbob_secret.tx.input.0.hash(&POOL_PARAMS.clone());
    let in_note_hash = zkbob_secret
        .tx
        .input
        .1
        .iter()
        .map(|n| n.hash(&POOL_PARAMS.clone()))
        .collect::<Vec<_>>();
    let in_hash = [[in_account_hash].as_ref(), in_note_hash.as_slice()].concat();
    let inproof = zkbob_secret.in_proof.0;
    let eta = key::derive_key_eta(zkbob_secret.eddsa_a, &POOL_PARAMS.clone());

    let out_commit = tx::out_commitment_hash(&out_hash, &POOL_PARAMS.clone());
    let root = poseidon_merkle_proof_root(in_account_hash, &inproof, POOL_PARAMS.compress());
    let path_num = from_bool_to_num(inproof.path).unwrap();
    let nullifier = tx::nullifier(in_account_hash, eta, path_num, &POOL_PARAMS.clone());
    let _tx_hash = tx::tx_hash(&in_hash, zkbob_public.out_commit, &POOL_PARAMS.clone());

    if out_commit == zkbob_public.out_commit
        && root == zkbob_public.root
        && nullifier == zkbob_public.nullifier
    {
        result = true;
    }
    Ok(result)
}

pub fn from_bool_to_num(path: SizedVec<bool, 48>) -> Result<Num<Fr>, Error> {
    let mut acc: Num<Fr> = path[0].into();
    let mut k = Num::ONE;
    for n in 1..48 {
        k = k.double();
        let num: Num<Fr> = path[n].into();
        acc += k * num;
    }
    Ok(acc)
}

pub async fn verify_zkbob_inputs_and_proof(
    payload: kalypso_ivs_models::models::VerifyInputsAndProof,
) -> Result<bool, Error> {
    let params = param_gen("./params/transfer_params_prod.bin");
    let mut result = false;

    let public_input = payload.clone().public_input.unwrap();
    let public_input_bytes = ethers::types::Bytes::from(public_input);
    let private_input = payload.clone().private_input.unwrap();

    // verify the inputs
    let verify_inputs = verify_zkbob_secret(&public_input_bytes, &private_input)
        .await
        .unwrap();

    // verify the proof with corresponding public inputs
    let public = zkbob_generator::decode_input(public_input_bytes.clone()).unwrap();

    let data = json!({
        "root": public[0].to_string(),
        "nullifier": public[1].to_string(),
        "out_commit": public[2].to_string(),
        "delta": public[3].to_string(),
        "memo": public[4].to_string()
    });

    let public: TransferPub<Fr> = serde_json::from_value(data).unwrap();
    let inputs = vec![
        public.clone().root,
        public.clone().nullifier,
        public.clone().out_commit,
        public.clone().delta,
        public.clone().memo,
    ];

    let proof_received = payload.clone().proof;
    let proof_structure = decode_proof(proof_received).unwrap();

    let verify_proof = verifier::verify(&params.get_vk(), &proof_structure, &inputs);

    if verify_inputs && verify_proof {
        result = true;
    }

    return Ok(result);
}

fn param_gen(transfer_params_path: &str) -> Parameters<Bn256> {
    let param_file = std::fs::read(transfer_params_path).unwrap();
    let mut param_format: &[u8] = &param_file;
    let params: Parameters<Bn256> = Parameters::read(&mut param_format, true, true).unwrap();
    params
}

pub fn decode_proof(encoded_input: Vec<u8>) -> Result<Proof<Bn256>, model::InputError> {
    let param_types = vec![ParamType::Bytes, ParamType::Bytes, ParamType::Bytes];
    let tokens = match decode(&param_types, &encoded_input.as_slice()) {
        Ok(data) => data,
        Err(_) => {
            return Err(model::InputError::DecodeFailed);
        }
    };

    let proof_param_types = vec![
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::Uint(256),
    ];
    let proof = tokens[1].clone().into_bytes().unwrap();
    let proof_parts = match decode(&proof_param_types, &proof.as_slice()) {
        Ok(data) => data,
        Err(_) => {
            return Err(model::InputError::DecodeFailed);
        }
    };
    let proof_reconstructed = json!({
        "a": [proof_parts[0].clone().into_uint().unwrap().to_string(), proof_parts[1].clone().into_uint().unwrap().to_string()],
        "b": [[proof_parts[2].clone().into_uint().unwrap().to_string(), proof_parts[3].clone().into_uint().unwrap().to_string()], [proof_parts[4].clone().into_uint().unwrap().to_string(), proof_parts[5].clone().into_uint().unwrap().to_string()]],
        "c": [proof_parts[6].clone().into_uint().unwrap().to_string(), proof_parts[7].clone().into_uint().unwrap().to_string()]
    });
    let proof_structure: Proof<Bn256> = serde_json::from_value(proof_reconstructed).unwrap();
    return Ok(proof_structure);
}
