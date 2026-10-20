use std::{collections::HashMap, fs, path::PathBuf};

use log::debug;
use serde::Deserialize;
use serde_json::json;

use super::{AvailableTool, ChatTool, ToolCallFunction, ToolCallFunctionParameters};

#[derive(Debug, Deserialize)]
struct WriteArgs {
    file_path: String,
    content: String,
}

pub struct WriteTool {}

impl WriteTool {
    pub fn new() -> Self {
        Self {}
    }
}

impl ChatTool for WriteTool {
    fn define(&self) -> AvailableTool {
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

    fn execute(&self, arguments: &str) -> String {
        let args: WriteArgs = serde_json::from_str(arguments).unwrap();

        debug!("Writing file: {}", &args.file_path);

        let path = PathBuf::from(&args.file_path);
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
        }

        fs::write(&args.file_path, &args.content).unwrap();
        "".to_string()
    }
}
