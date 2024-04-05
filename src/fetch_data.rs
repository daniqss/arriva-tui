use reqwest::Error;

pub async fn fetch_data(endpoint: &str) -> Result<String, Error> {
    let url = format!("https://arriva.gal/plataforma/api/{}", endpoint);
    let client = reqwest::Client::new();

    println!("Fetching data from: {}", url);
    let response = client.post(&url)
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
