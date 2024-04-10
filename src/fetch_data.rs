use crate::Error;

pub enum PostType {
    FORM,
    BODY
}

pub async fn fetch_data(endpoint: &str, content_type: &str, content: &str, post_type: PostType) -> Result<String, Error> {
    let client = reqwest::Client::new();

    println!("{}", content);
    let request = match post_type {
        PostType::FORM => {
            let data = match serde_urlencoded::from_str(&content){
                Ok(data) => data,
                Err(error) => return Err(error.into())
            };
            client.post(endpoint).form(&data)
        },
        PostType::BODY => client.post(endpoint).header("Content-Type", content_type).body(content.to_string()),
    };

    let response = request.send().await?;


    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        println!("omg");
        Ok(text)  
    }
    else { 
        match response.error_for_status() {
            Ok(_) => Err(Error::from("Response error".into())),
            Err(error) => Err(error.into())
        }
    }
}
