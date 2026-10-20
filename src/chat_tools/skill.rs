use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;

use crate::skills::Skill;

use super::{AvailableTool, ChatTool, ToolCallFunction, ToolCallFunctionParameters};

#[derive(Debug, Deserialize)]
struct SkillArgs {
    name: String,
    args: Option<String>,
}

pub struct SkillTool {
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
