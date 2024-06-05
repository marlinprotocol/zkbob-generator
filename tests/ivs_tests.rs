#[cfg(test)]
mod tests {
    use reqwest::Client;
    use std::process::{Child, Command};
    use std::thread;
    use std::time::Duration;

    fn spawn_server() -> Child {
        let server = Command::new("cargo")
            .args(&["run", "--bin", "ivs"]) // Replace with your actual binary name
            .spawn()
            .expect("Server failed to start");

        // Give the server a few seconds to start
        thread::sleep(Duration::from_secs(2));
        server
    }

    #[tokio::test]
    async fn test_basic_endpoint() {
        let mut server = spawn_server();

        let client = Client::new();
        let response = client
            .get("http://127.0.0.1:3030/api/test")
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);
        let body = response.text().await.expect("Failed to read response body");
        assert_eq!(
            body,
            "{\"message\":\"The zkbob ivs is running!!\",\"data\":null}"
        );

        // Terminate the server process
        server.kill().expect("Failed to kill server process");
    }

    #[tokio::test]
    async fn test_check_inputs() {
        use serde::{Deserialize, Serialize};

        let mut server = spawn_server();

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct InputPayload {
            pub public: String,
            pub secrets: Option<String>,
        }

        let client = Client::new();
        let payload = InputPayload {
            public: "some_public_data".into(),
            secrets: Some("some_secret_data".into()),
        };

        let response = client
            .post("http://127.0.0.1:3030/api/checkInputs")
            .json(&payload)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);
        let body = response.text().await.expect("Failed to read response body");
        assert_eq!(body, "true");

        // Terminate the server process
        server.kill().expect("Failed to kill server process");
    }

    #[tokio::test]
    async fn test_check_encrypted_inputs() {
        use serde::{Deserialize, Serialize};

        let mut server = spawn_server();

        todo!("complete tests with expected reverts");

        // Terminate the server process
        server.kill().expect("Failed to kill server process");
    }

}
