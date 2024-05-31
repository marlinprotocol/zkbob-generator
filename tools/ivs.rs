use actix_web::http::StatusCode;
use actix_web::{get, post, web, Responder};
use actix_web::{App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::time::Duration;

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

// The payload for the POST request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputPayload {
    pub public: String,
    pub private: String,
}

#[post("/checkInput")]
async fn check_input_handler(payload: web::Json<InputPayload>) -> impl Responder {
    let public_inputs = hex::decode(payload.clone().public).unwrap().into();
    let private_inputs = hex::decode(payload.clone().private).unwrap();

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
