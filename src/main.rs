use std::env;
use reqwest;
use tokio;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Prompt {
    prompt: String,
    model: String,
    temperature: f64,
    max_tokens: u32,
    top_p: u32,
    n: u32,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bashhelp <prompt>");
        return;
    }
    let base_prompt = "You are an AI coding assistant that helps the user to write bash commands for penetration testing purposes. You have sudo access. You should read the command or intent message, then print a valid bash command that the user can use. Put the command on the same line as the \">\". For example:\nCommand/intent: scan for open ports using nmap\n> sudo nmap 192.168.0.1\n\nCommand/intent: ";
    let user_prompt = args[1].clone();
    let prompt = Prompt {
        prompt: format!("{}{}\n>", base_prompt, user_prompt),
        model: "text-davinci-003".to_string(),
        temperature: 0.0,
        max_tokens: 100,
        top_p: 1,
        n: 1,
    };

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&prompt)
        .send();

    match response.await {
        Ok(res) => {
            if res.status().is_success() {
                let result: Response = res.json().await.unwrap();
                let text = result.choices[0].text.trim();
                println!("{}", text);
            } else {
                println!("Request failed with status: {}", res.status());
            }
        }
        Err(err) => {
            println!("There was an issue with the request: {}", err);
        }
    }
}
