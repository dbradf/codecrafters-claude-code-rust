use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTool {
    #[serde(rename = "type")]
    kind: String,
    function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolCallFunction {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<ToolCallFunctionParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolCallFunctionParameters {
    #[serde(rename = "type")]
    kind: String,
    properties: HashMap<String, Value>,
    required: Vec<String>,
}

pub fn available_tools() -> Vec<AvailableTool> {
    vec![define_read(), define_write(), define_bash()]
}

fn define_read() -> AvailableTool {
    let mut parameters = HashMap::new();
    parameters.insert(
        "file_path".to_string(),
        json!({ "type": "string", "description": "The path to the file to read" }),
    );
    AvailableTool {
        kind: "function".to_string(),
        function: ToolCallFunction {
            name: "Read".to_string(),
            description: Some("read and return the contents of a file".to_string()),
            parameters: Some(ToolCallFunctionParameters {
                kind: "object".to_string(),
                properties: parameters,
                required: vec!["file_path".to_string()],
            }),
            strict: None,
        },
    }
}

fn define_write() -> AvailableTool {
    let mut parameters = HashMap::new();
    parameters.insert(
        "file_path".to_string(),
        json!({ "type": "string", "description": "The path to the file to read" }),
    );
    parameters.insert(
        "content".to_string(),
        json!({"type": "string", "description": "The content to write to the file"}),
    );
    AvailableTool {
        kind: "function".to_string(),
        function: ToolCallFunction {
            name: "Write".to_string(),
            description: Some("Write content to a file".to_string()),
            parameters: Some(ToolCallFunctionParameters {
                kind: "object".to_string(),
                properties: parameters,
                required: vec!["file_path".to_string(), "content".to_string()],
            }),
            strict: None,
        },
    }
}

fn define_bash() -> AvailableTool {
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
