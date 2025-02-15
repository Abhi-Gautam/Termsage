use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Returns the appropriate URL depending on whether streaming is enabled.
pub fn get_url(api_key: &str, streaming: bool) -> String {
    if streaming {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:streamGenerateContent?alt=sse&key={}",
            api_key
        )
    } else {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            api_key
        )
    }
}

/// Sends the request to the provided URL with the given JSON payload.
pub async fn send_request<T: Serialize>(
    url: &str,
    request_body: &T,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    client
        .post(url)
        .header("Content-Type", "application/json")
        .json(request_body)
        .send()
        .await
}

// -------------------
// Common response structs
// -------------------

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: ContentResponse,
}

#[derive(Deserialize)]
pub struct ContentResponse {
    pub parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
pub struct ResponsePart {
    pub text: String,
}

// -------------------
// Request types and payload builders
// -------------------

#[derive(Serialize)]
pub struct GeminiRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Instruction>,
    pub contents: Vec<Content>,
}

#[derive(Serialize)]
pub struct Instruction {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize)]
pub struct Part {
    pub text: String,
}

/// Builds the request payload for generating commands.
/// System instruction: Respond as a command line expert with a concise CLI solution.
pub fn build_command_request(prompt: &str) -> GeminiRequest {
    let instruction_text = "You are a command line expert. Respond only with a concise CLI solution. Only answer questions related to coding, programming, and command line usage.";
    GeminiRequest {
        system_instruction: Some(Instruction {
            parts: vec![Part {
                text: instruction_text.to_string(),
            }],
        }),
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    }
}

/// Builds the request payload for generating documentation.
/// System instruction: Respond as a documentation expert providing precise and official documentation support.
pub fn build_documentation_request(prompt: &str) -> GeminiRequest {
    let instruction_text = "You are a documentation expert. Provide official and precise documentation support for coding and programming queries. Answer only with factual, code-related documentation.";
    GeminiRequest {
        system_instruction: Some(Instruction {
            parts: vec![Part {
                text: instruction_text.to_string(),
            }],
        }),
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    }
}
