use anyhow::Result;
use dotenv::dotenv;
use std::env::{self, VarError};
use thiserror::Error;

use serde::Deserialize;

#[derive(Error, Debug)]
pub enum CompletionApiError {
    #[error("Request to fetch completion failed: {0}")]
    RequestFailed(Box<ureq::Error>),
    #[error("Failed to get your OpenAI key. Did you create a .env file?")]
    FailedToGetVarEnv(VarError),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Response JSON parsing failed")]
    ChoiceParseFailed(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletionApiResponse {
    pub choices: Vec<Choice>,
}

pub fn get_ai_completion(prompt: String) -> Result<CompletionApiResponse> {
    dotenv().ok();
    let api_key = env::var("API_KEY").map_err(CompletionApiError::FailedToGetVarEnv)?;

    let response_json: String = ureq::post("https://api.openai.com/v1/completions")
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", &api_key))
        .send_json(ureq::json!({
            "model": "text-davinci-003",
            "prompt": prompt,
            "temperature": 0.9,
            "max_tokens": 2048,
        }))
        .map_err(|e| CompletionApiError::RequestFailed(Box::new(e)))?
        .into_string()
        .map_err(CompletionApiError::FailedResponseToString)?;
    let parsed_response: CompletionApiResponse =
        serde_json::from_str(&response_json).map_err(CompletionApiError::ChoiceParseFailed)?;
    Ok(parsed_response)
}
