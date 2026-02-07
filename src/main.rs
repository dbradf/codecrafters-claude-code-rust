use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    env, fs,
    process::{self, Command},
};

use crate::{
    chat_message::{ChatMessage, UserContent},
    chat_tools::{AvailableTool, available_tools},
};

mod chat_message;
mod chat_tools;

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

    let tools = available_tools();
    // let tools = vec![ChatTool {
    //     tool_type: "function".to_string(),
    //     function: ChatFunction {
    //         name: "Read".to_string(),
    //         description: "read and return the contents of a file".to_string(),
    //         parameters: ChatFunctionParams {
    //             param_type: "object".to_string(),
    //             properties: json!({"file_path": {"type": "string", "description": "The path to the file to read"}}),
    //             required: vec!["file_path".to_string()],
    //         },
    //     },
    // }];

    // let message = vec![ChatMessage {
    //     role: "user".to_string(),
    //     content: args.prompt,
    // }];

    let message = ChatMessage::User {
        content: UserContent::String(args.prompt.clone()),
        name: None,
    };

    let mut request = ChatRequest {
        messages: vec![message],
        model: "anthropic/claude-haiku-4.5".to_string(),
        tools,
    };

    loop {
        let request_str = serde_json::to_string_pretty(&request)?;
        eprintln!("Request: {}", &request_str);

        let response: Value = client
            .chat()
            .create_byot(&request)
            // .create_byot(json!({
            //     "messages": [
            //         {
            //             "role": "user",
            //             "content": args.prompt
            //         }
            //     ],
            //     "model": "anthropic/claude-haiku-4.5",
            //     "tools": [
            //         {
            //             "type": "function",
            //             "function": {
            //                 "name": "Read",
            //                 "description": "read and return the contents of a file",
            //                 "parameters": {
            //                     "type": "object",
            //                     "properties": {
            //                         "file_path": {
            //                             "type": "string",
            //                             "description": "The path to the file to read"
            //                         }
            //                     },
            //                     "required": ["file_path"]
            //                 }
            //             }
            //         }
            //     ]
            // }))
            .await?;

        eprintln!("Response: {}", &response);

        if let message_str = &response["choices"][0]["message"] {
            dbg!(&message_str);
            let message: ChatMessage = serde_json::from_value(message_str.clone())?;
            eprintln!("Adding response: {}", message_str);
            request.messages.push(message);
        }

        let result = handle_response(response);
        if let Some(tool_message) = result {
            request.messages.push(tool_message);
        } else {
            break;
        }
    }

    Ok(())
}

fn handle_response(response: Value) -> Option<ChatMessage> {
    let mut result = None;
    if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
        for tool_call in tool_calls {
            match tool_call["type"].as_str() {
                Some("function") => {
                    let content = handle_function(&tool_call["function"]);
                    let id = tool_call["id"].as_str().unwrap();

                    result = Some(ChatMessage::Tool {
                        content,
                        tool_call_id: id.to_string(),
                    })
                }
                _ => todo!(),
            }
        }
    } else if let Some(content) = response["choices"][0]["message"]["content"].as_str()
        && !content.is_empty()
    {
        println!("{}", content);
    }

    result
}

fn handle_function(fun: &Value) -> String {
    match fun["name"].as_str() {
        Some("Read") => {
            let arguments = fun["arguments"].as_str().unwrap();
            let args: ReadArgs = serde_json::from_str(arguments).unwrap();

            fs::read_to_string(&args.file_path).unwrap()
        }
        Some("Write") => {
            let arguments = fun["arguments"].as_str().unwrap();
            let args: WriteArgs = serde_json::from_str(arguments).unwrap();

            fs::write(&args.file_path, &args.content).unwrap();
            "".to_string()
        }
        Some("Bash") => {
            let arguments = fun["arguments"].as_str().unwrap();
            let args: BashArgs = serde_json::from_str(arguments).unwrap();

            let command: Vec<&str> = args.command.split_ascii_whitespace().collect();

            let result = Command::new(command[0]).args(&command[1..]).output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        String::from_utf8_lossy(&output.stdout).to_string()
                    } else {
                        String::from_utf8_lossy(&output.stderr).to_string()
                    }
                }
                Err(err) => err.to_string(),
            }
        }
        _ => todo!(),
    }
}

#[derive(Debug, Deserialize)]
struct ReadArgs {
    file_path: String,
}

#[derive(Debug, Deserialize)]
struct WriteArgs {
    file_path: String,
    content: String,
}
#[derive(Debug, Deserialize)]
struct BashArgs {
    command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ChatRequest {
    messages: Vec<ChatMessage>,
    model: String,
    tools: Vec<AvailableTool>,
}
