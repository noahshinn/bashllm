use std::env;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use reqwest;
use tokio;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Prompt {
    messages: Vec<Message>,
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
    message: Message,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bashllm <natural language description>");
        return;
    }
    let base_prompt = "You are an AI coding assistant that helps the user to write bash commands for penetration testing purposes. You should read the intent message, then print a valid bash command or command template that the user can use. For example:\nIntent: scan for open ports in a certain range using nmap and determine services or versions\nCommand: sudo nmap -p <port_range> -sV -A <target_ip>\nExplanation:\nLet's break down the options used in this command:\n\n-p <port_range>: Specifies the port range to scan. You can specify a single port or a range of ports. For example, you can use -p 1-100 to scan ports 1 to 100.\n-sV: Performs service/version detection to determine the service running on the open ports.\n-A: Enables aggressive scanning options including OS detection, version detection, script scanning, and traceroute.\n<target_ip>: Replace this with the IP address or hostname of the target you want to scan for open ports.\n\nIntent: ";
    let user_prompt = args[1].clone();

    let prompt = Prompt {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: format!("{}{}\n>", base_prompt, user_prompt)
            }
        ],
        temperature: 0.0,
        max_tokens: 100,
        top_p: 1,
        n: 1
    };


    let api_key = env::var("OPENAI_API_KEY").or_else(|_| env::var("OPENAI_APIKEY"))
    .expect("Either OPENAI_API_KEY or OPENAI_APIKEY not set");
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&prompt)
        .send();

    match response.await {
        Ok(res) => {
            if res.status().is_success() {
                let result: Response = res.json().await.unwrap();
                let response: Vec<&str> = result.choices[0].message.content.trim().split("Command:").collect::<Vec<&str>>().last().unwrap_or(&"").split("Explanation:").collect();
                let command_text = response.first().unwrap_or(&"").trim();
                let explanation_text = response.last().unwrap_or(&"").trim();
                let is_copied: bool;
                if let Ok(mut ctx) = ClipboardContext::new() {
                    if let Err(err) = ctx.set_contents(command_text.to_owned()) {
                        println!("Failed to copy to clipboard: {}", err);
                        is_copied = false;
                    } else {
                        is_copied = true;
                    }
                } else {
                    println!("Failed to access clipboard");
                    is_copied = false;
                }
                println!("\nCommand:\n\n    {}\n\nExplanation:\n\n{}", command_text, explanation_text);
                if is_copied {
                    println!("\nCommand has been copied to clipboard!\n");
                }
            } else {
                println!("Request failed with status: {}", res.status());
            }
        }
        Err(err) => {
            println!("There was an issue with the request: {}", err);
        }
    }
}
