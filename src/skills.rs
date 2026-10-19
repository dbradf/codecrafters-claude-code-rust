use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::skills::ContentState::InArgument;

const SKILLS_DIR: &str = ".claude/skills";

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
    pub path: String,
}

pub fn format_system_skills_message(skills: &[Skill]) -> String {
    let skill_messages: Vec<String> = skills.iter().map(|s| s.format_desc_message()).collect();
    format!(
        "You have access to the following skills:\n\n{}\nIf a skill matches the user's request, call the Skill tool with its name and follow the instructions it returns.",
        skill_messages.join("\n")
    )
}

#[derive(Debug, Deserialize)]
struct SkillFrontmatter {
    name: String,
    description: String,
}

impl Skill {
    pub fn load_skills() -> Result<Vec<Self>> {
        let skills_path = PathBuf::from(SKILLS_DIR);
        let skills = if skills_path.exists() {
            fs::read_dir(SKILLS_DIR)?
                .filter_map(|entry| {
                    let entry = entry.unwrap().path();
                    if entry.is_dir() {
                        Some(Self::load_skill(&entry).unwrap())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![]
        };

        Ok(skills)
    }

    pub fn load_skill(dir: &Path) -> Result<Self> {
        let path = dir.join("SKILL.md");
        let contents = fs::read_to_string(&path)?;
        let (frontmatter, body) = markdown_frontmatter::parse::<SkillFrontmatter>(&contents)?;

        Ok(Self {
            name: frontmatter.name,
            description: frontmatter.description,
            content: body.to_string(),
            path: dir.to_str().unwrap().to_string(),
        })
    }

    pub fn format_desc_message(&self) -> String {
        format!("- {}: {}", self.name, self.description)
    }

    pub fn load_content(&self, arguments: &[&str]) -> String {
        let mut contents = format!("Skill: {} (located at {})", self.name, self.path);
        let mut state = ContentState::Default;
        for ch in self.content.chars() {
            match &state {
                ContentState::Default => {
                    if ch == '$' {
                        state = InArgument("".to_string());
                    } else {
                        contents.push(ch);
                    }
                }
                ContentState::InArgument(running) => {
                    if ch.is_alphanumeric() || ch == '[' || ch == ']' {
                        state = InArgument(format!("{running}{ch}"));
                    } else {
                        let value = parse_argument(running, arguments);
                        contents.push_str(&value);
                        contents.push(ch);
                        state = ContentState::Default;
                    }
                }
            }
        }

        if let ContentState::InArgument(running) = state {
            let value = parse_argument(&running, arguments);
            contents.push_str(&value);
        }

        contents
    }
}

enum ContentState {
    Default,
    InArgument(String),
}

fn parse_argument(running: &str, arguments: &[&str]) -> String {
    if running == "ARGUMENTS" {
        return arguments.join(" ");
    }

    let index = if running.chars().nth(0).unwrap().is_numeric() {
        running.parse::<usize>().unwrap()
    } else {
        let re = Regex::new(r"ARGUMENTS[(?P<index>\d+)]").unwrap();
        if let Some(caps) = re.captures(running) {
            let value = caps.name("index").unwrap().as_str();
            value.parse::<usize>().unwrap()
        } else {
            0
        }
    };

    return arguments.get(index).unwrap_or(&&"").to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load_content() {
        let skill = Skill {
            name: "hello".into(),
            description: "description".into(),
            content: "Deploy to $0 in region $1. Full request was: $ARGUMENTS".into(),
            path: "".to_string(),
        };

        let actual = skill.load_content(&["staging".into(), "eu-west".into()]);
        let expected =
            "Deploy to staging in region eu-west. Full request was: staging eu-west".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_load_content_with_missing_arg() {
        let skill = Skill {
            name: "hello".into(),
            description: "description".into(),
            content: "Deploy to $0 in region $11. Full request was: $ARGUMENTS".into(),
            path: "".to_string(),
        };

        let actual = skill.load_content(&["staging".into(), "eu-west".into()]);
        let expected =
            "Deploy to staging in region . Full request was: staging eu-west".to_string();
        assert_eq!(actual, expected);
    }
}
