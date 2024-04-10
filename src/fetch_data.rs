use reqwest::Error;

pub async fn fetch_data(endpoint: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let response = client.post(endpoint)
        .header("Content-Type", "application/json")
        .body(r#"{"key":"value"}"#)
        .send()
        .await?;


    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        Ok(text)  
    }
    else { response.error_for_status().map(|_| String::from("")).map_err(|e| e) }
}
