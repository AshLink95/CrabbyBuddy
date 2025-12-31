use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiFreeLLM { response: String, status: String }

impl ApiFreeLLM {
    pub async fn new(body: &str) -> Self {
        let client = reqwest::Client::new();
        let res = client.post("https://apifreellm.com/api/chat")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&serde_json::json!({"message": body}))
            .send()
            .await;

        match res {
            Ok(res) => {
                let status = res.status().to_string();
                match res.text().await {
                    Ok(resp) => match serde_json::from_str::<Self>(&resp) {
                        Ok(p) => Self { response: p.response, status },
                        Err(_) => Self { response: "".to_string(), status }
                    },
                    Err(_) => Self { response: "".to_string(), status }
                }
            },
            Err(_) => Self { response: "".to_string(), status: "Failure".to_string() }
        }
    }

    pub fn get_resp(&self) -> &str { &(self.response) }

    pub fn get_stat(&self) -> &str { &(self.status) }
}
