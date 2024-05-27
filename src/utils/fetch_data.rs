use crate::prelude::*;

pub async fn fetch_data(endpoint: &str, content_type: &str, content: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let request = client
        .post(endpoint)
        .header("Content-Type", content_type)
        .header("Content-Length", content.len())
        .header("User-Agent", "curl/8.7.1")
        .header("Accept", "*/*")
        .body(content.to_owned());

    let response = request.send().await?;

    if response.status().is_success() {
        let text = response.text().await?;
        let text = text.replace("\\u00f1", "ñ");
        let text = text.replace("\\u00e1", "á");
        let text = text.replace("\\u00e9", "é");
        let text = text.replace("\\u00ed", "í");
        let text = text.replace("\\u00f3", "ó");
        let text = text.replace("\\u00fa", "ú");
        let text = text.replace("\\u00fc", "ü");
        let text = text.replace("\\u00e7", "ç");
        let text = text.replace("\\u00c1", "Á");
        let text = text.replace("\\u00c9", "É");
        let text = text.replace("\\u00cd", "Í");
        let text = text.replace("\\u00d3", "Ó");
        let text = text.replace("\\u00da", "Ú");
        let text = text.replace("\\u00dc", "Ü");
        let text = text.replace("\\u00c7", "Ç");

        Ok(text)
    } else {
        match response.error_for_status() {
            Ok(_) => Err(Error::Generic("hola".to_string())),
            Err(error) => Err(Error::Reqwest(error)),
        }
    }
}
