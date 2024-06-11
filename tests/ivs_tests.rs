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

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct EncryptedInputPayload {
            pub acl: String,
            pub encrypted_secrets: String,
            pub me_decryption_url: String,
            pub market_id: String,
        }

        let client = Client::new();

    
        let encrypted_data = "e87a1e02d89aae5eb8e497dbdc72e5bd58fc509522283f699ac31972041b4b80";
        let acl_data = "\u{4}�1�RcB\u{5}�\u{2}V�c\u{e}\0�q�LQ\u{6}o.M6��&R��U�Iw�ż�\u{1a}��1MA`�\u{7}��\u{c}��9_\u{7}C��l�us\u{1b}���F�\u{5}��\u{6}�z��\u{7f}�V)R �O$�&\u{1c}o����-�w�\u{8}�.����\u{19}�Ύ \u{f}YfѺ-��f<\u{33b}��m���";
        let payload = EncryptedInputPayload {
            acl: acl_data.to_string(),
            encrypted_secrets: encrypted_data.to_string(),
            me_decryption_url: "http://localhost:3000/decryptRequest".to_string(),
            market_id: "1".to_string(),
        };
        
        let response = client
            .post("http://127.0.0.1:3030/api/checkEncryptedInputs")
            .json(&payload)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), 200);
        let body = response.text().await.expect("Failed to read response body");
        assert_eq!(body, "true");

    // Terminate the server process
    server.kill().expect("Failed to kill server process");
        // Terminate the server process
        server.kill().expect("Failed to kill server process");
    }



}
