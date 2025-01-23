use reqwest::{multipart, Client};
use serde::Deserialize;
use std::{error::Error, fs::File, io::Read};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SulUploadResponse {
    domain: String,
    filename: String,
    protocol: String,
    url: String,
}

pub async fn sul_upload(file_path: &str, api_key: String) -> Result<String, Box<dyn Error>> {
    const API_URL: &str = "https://s-ul.eu/api/v1/upload";
    let client = Client::new();

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let form = multipart::Form::new()
        .text("wizard", "true")
        .text("key", api_key)
        .file("file", file_path)
        .await?;

    let response = client.post(API_URL).multipart(form).send().await?;

    if response.status().is_success() {
        let upload_response: SulUploadResponse = response.json().await?;
        Ok(upload_response.url)
    } else {
        Err(format!("Failed with status: {}", response.status()).into())
    }
}

pub async fn imgur_upload(file_path: &str, client_id: String) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    todo!("Implement Imgur upload")
}
