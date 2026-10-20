use std::{collections::HashMap, process::Command};

use serde::Deserialize;
use serde_json::json;

use super::{AvailableTool, ChatTool, ToolCallFunction, ToolCallFunctionParameters};

#[derive(Debug, Deserialize)]
struct BashArgs {
    command: String,
}

pub struct BashTool {}

impl BashTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ChatTool for BashTool {
    fn define(&self) -> AvailableTool {
        let mut parameters = HashMap::new();
        parameters.insert(
            "command".to_string(),
            json!({ "type": "string", "description": "The command to execute" }),
        );
        AvailableTool {
            kind: "function".to_string(),
            function: ToolCallFunction {
                name: "Bash".to_string(),
                description: Some("Execute a shell command".to_string()),
                parameters: Some(ToolCallFunctionParameters {
                    kind: "object".to_string(),
                    properties: parameters,
                    required: vec!["command".to_string()],
                }),
                strict: None,
            },
        }
    }

    fn execute(&self, arguments: &str) -> String {
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
}
