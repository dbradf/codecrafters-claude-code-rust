use std::{collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;
use serde_json::json;

use super::{AvailableTool, ChatTool, ToolCallFunction, ToolCallFunctionParameters};

#[derive(Debug, Deserialize)]
struct ReadArgs {
    file_path: String,
}

pub struct ReadTool {}

impl ReadTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ChatTool for ReadTool {
    fn define(&self) -> AvailableTool {
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

    fn execute(&self, arguments: &str) -> String {
        let args: ReadArgs = serde_json::from_str(arguments).unwrap();
        let path = PathBuf::from(&args.file_path);

        if path.is_dir() {
            format!("{} is a directory", path.to_str().unwrap())
        } else {
            fs::read_to_string(path).unwrap()
        }
    }
}
