use anyhow::Result;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

const SKILLS_DIR: &str = ".claude/skills";

#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
}

pub fn format_system_skills_message(skills: Vec<Skill>) -> String {
    let skill_messages: Vec<String> = skills.iter().map(|s| s.format_desc_message()).collect();
    format!(
        "You have access to the following skills:\n\n{}\n",
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
        })
    }

    pub fn format_desc_message(&self) -> String {
        format!("- {}: {}", self.name, self.description)
    }
}
