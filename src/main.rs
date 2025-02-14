
use std::env;
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a terminal command using Gemini API
    Cmd {
        /// The prompt describing what command you need
        prompt: String,
    },
    /// Get documentation or syntax help using Gemini API
    Doc {
        /// The prompt for documentation help
        prompt: String,
    },
}

// These structs match the JSON structure required by the API
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// Adjust the response structures based on the API’s response
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

async fn call_gemini_api(prompt: &str) -> Result<String, reqwest::Error> {
    // Retrieve the API key from the environment variables
    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in .env file or your environment");

    // Construct the request payload
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    // Build the URL with the API key as a query parameter
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Deserialize the JSON response into our struct
    let gemini_response: GeminiResponse = response.json().await?;
    // Retrieve the first candidate's text
    let result = gemini_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.clone())
        .unwrap_or_else(|| "No response received".to_string());

    Ok(result)
}

#[tokio::main]
async fn main() {
    // Load environment variables from the .env file
    dotenv().ok();

    let cli = Cli::parse();

    // Select the prompt based on the subcommand used
    let prompt = match &cli.command {
        Commands::Cmd { prompt } => prompt,
        Commands::Doc { prompt } => prompt,
    };

    // Call the Gemini API with the prompt and print the result
    match call_gemini_api(prompt).await {
        Ok(result) => println!("{result}"),
        Err(e) => eprintln!("Error calling Gemini API: {e}"),
    }
}
