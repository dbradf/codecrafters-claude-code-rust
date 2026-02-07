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
    vec![define_read()]
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
