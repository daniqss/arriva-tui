use std::error::Error;

pub async fn fetch_data(endpoint: &str, content_type: &str, content: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    println!("{}", content);
    let request = client.post(endpoint)
        .header("Content-Type", content_type)
        .header("Content-Length", content.len())
        .header("User-Agent", "curl/8.7.1")
        .header("Accept", "*/*")
        .body(content.to_owned());

    let response = request.send().await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        println!("omg");
        Ok(text)  
    }
    else { 
        match response.error_for_status() {
            Ok(_) => Err(Box::from("Error in response") as Box<dyn Error>),
            Err(error) => Err(Box::from(error) as Box<dyn Error>)
        }
    }
}
