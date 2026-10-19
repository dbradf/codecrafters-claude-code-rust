use std::{collections::HashMap, fs, path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::skills::Skill;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTool {
    #[serde(rename = "type")]
    kind: String,
    function: ToolCallFunction,
}

trait ChatTool {
    fn define(&self) -> AvailableTool;
    fn execute(&self, arguments: &str) -> String;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ChatTool>>,
}

impl ToolRegistry {
    pub fn init(skills: Vec<Skill>) -> Self {
        let mut tools: HashMap<String, Box<dyn ChatTool>> = HashMap::new();
        tools.insert("Read".into(), Box::new(ReadTool::new()));
        tools.insert("Write".into(), Box::new(WriteTool::new()));
        tools.insert("Bash".into(), Box::new(BashTool::new()));
        tools.insert("Skill".into(), Box::new(SkillTool::new(skills)));

        Self { tools }
    }

    pub fn execute(&self, name: &str, arguments: &str) -> String {
        let tool = self.tools.get(name).unwrap();
        tool.execute(arguments)
    }

    pub fn available_tools(&self) -> Vec<AvailableTool> {
        self.tools
            .values()
            .into_iter()
            .map(|t| t.define())
            .collect()
    }
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

#[derive(Debug, Deserialize)]
struct ReadArgs {
    file_path: String,
}

struct ReadTool {}

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

#[derive(Debug, Deserialize)]
struct WriteArgs {
    file_path: String,
    content: String,
}

struct WriteTool {}

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

        fs::write(&args.file_path, &args.content).unwrap();
        "".to_string()
    }
}

#[derive(Debug, Deserialize)]
struct BashArgs {
    command: String,
}

struct BashTool {}

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

#[derive(Debug, Deserialize)]
struct SkillArgs {
    name: String,
    args: Option<String>,
}

struct SkillTool {
    skills: Vec<Skill>,
}

impl SkillTool {
    pub fn new(skills: Vec<Skill>) -> Self {
        Self { skills }
    }
}

impl ChatTool for SkillTool {
    fn define(&self) -> AvailableTool {
        let mut parameters = HashMap::new();
        parameters.insert(
            "name".to_string(),
            json!({ "type": "string", "description": "The name of the still to use" }),
        );
        parameters.insert(
            "args".to_string(),
            json!({ "type": "string", "description": "Optional arguments for the skill" }),
        );
        AvailableTool {
            kind: "function".to_string(),
            function: ToolCallFunction {
                name: "Skill".to_string(),
                description: Some("Load a skill's instructions into the conversation".to_string()),
                parameters: Some(ToolCallFunctionParameters {
                    kind: "object".to_string(),
                    properties: parameters,
                    required: vec!["name".to_string()],
                }),
                strict: None,
            },
        }
    }

    fn execute(&self, arguments: &str) -> String {
        let args: SkillArgs = serde_json::from_str(arguments).unwrap();

        let argument_str = args.args.unwrap_or_default();
        let argument_list: Vec<&str> = argument_str.split(" ").collect();
        let skill_name = args.name;
        for s in &self.skills {
            if s.name.as_str() == skill_name {
                return s.load_content(&argument_list);
            }
        }
        "".to_string()
    }
}
