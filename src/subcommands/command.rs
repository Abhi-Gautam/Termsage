use crate::prompt::{build_command_request, get_url, send_request, GeminiResponse};
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::env;
use std::io::{self, Write};
use std::process::Command;

pub async fn generate_command(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    let request_body = build_command_request(prompt);
    let url = get_url(&api_key, false);

    let response = send_request(&url, &request_body).await?;
    let gemini_response: GeminiResponse = response.json().await?;

    let result = gemini_response
        .candidates
        .get(0)
        .and_then(|candidate| candidate.content.parts.get(0))
        .map(|part| part.text.clone())
        .unwrap_or_else(|| "No response received".to_string());
    interactive_command_prompt(&result)?;

    Ok(())
}

/// Displays the generated command with formatting and prompts the user for action.
fn interactive_command_prompt(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", command.bold().green());
    // Present options to the user.
    print!("[c] Copy command");
    print!("  [ENTER] Execute command");
    print!("  [q] Cancel  :");
    io::stdout().flush()?;
    enable_raw_mode()?;
    let event = read()?;
    disable_raw_mode()?;
    if let Event::Key(KeyEvent { code, .. }) = event {
        match code {
            KeyCode::Char('c') | KeyCode::Char('C') => {
                let mut ctx: ClipboardContext = ClipboardProvider::new()?;
                ctx.set_contents(command.to_string())?;
                println!("\nCommand copied to clipboard.");
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                println!("\nCommand execution cancelled.");
            }
            KeyCode::Enter => {
                println!("\nExecuting command...");
                let status = Command::new("sh").arg("-c").arg(command).status()?;
                println!("Command executed with status: {}", status);
            }
            _ => {
                println!("\nUnrecognized option. Exiting.");
            }
        }
    }

    Ok(())
}
