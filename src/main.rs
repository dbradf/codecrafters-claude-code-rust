use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{env, fs, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt
                }
            ],
            "model": "anthropic/claude-haiku-4.5",
            "tools": [
                {
                    "type": "function",
                    "function": {
                        "name": "Read",
                        "description": "read and return the contents of a file",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "file_path": {
                                    "type": "string",
                                    "description": "The path to the file to read"
                                }
                            },
                            "required": ["file_path"]
                        }
                    }
                }
            ]
        }))
        .await?;

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    handle_response(response);
    // if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
    //     println!("{}", content);
    // }

    Ok(())
}

fn handle_response(response: Value) {
    dbg!(&response);
    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        for tool_call in tool_calls {
            dbg!(&tool_call);
            match tool_call["type"].as_str() {
                Some("function") => handle_function(&tool_call["function"]),
                _ => todo!(),
            }
        }
    }
    if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        if !content.is_empty() {
            println!("{}", content);
        }
    }
}

fn handle_function(fun: &Value) {
    dbg!(&fun);

    match fun["name"].as_str() {
        Some("Read") => {
            let arguments = fun["arguments"].as_str().unwrap();
            let args: ReadArgs = serde_json::from_str(arguments).unwrap();
            let contents = fs::read_to_string(&args.file_path).unwrap();
            println!("{}", contents);
        }
        _ => {}
    }
}

#[derive(Debug, Deserialize)]
struct ReadArgs {
    file_path: String,
}
