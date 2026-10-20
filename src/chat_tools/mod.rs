use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::skills::Skill;

mod bash;
mod read;
mod skill;
mod write;

use bash::BashTool;
use read::ReadTool;
use skill::SkillTool;
use write::WriteTool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTool {
    #[serde(rename = "type")]
    kind: String,
    function: ToolCallFunction,
}

pub trait ChatTool {
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
pub struct ToolCallFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ToolCallFunctionParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallFunctionParameters {
    #[serde(rename = "type")]
    pub kind: String,
    pub properties: HashMap<String, Value>,
    pub required: Vec<String>,
}
