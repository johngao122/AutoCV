use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resume {
    pub profile: Profile,
    pub experiences: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Skills,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub certifications: Vec<Certification>,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    pub publications: Vec<Publication>,
    #[serde(default)]
    pub volunteer: Vec<Volunteer>,
    #[serde(default)]
    pub metadata: ResumeMetadata,
}

impl Resume {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile,
            experiences: vec![],
            education: vec![],
            skills: Skills::default(),
            projects: vec![],
            certifications: vec![],
            languages: vec![],
            publications: vec![],
            volunteer: vec![],
            metadata: ResumeMetadata::default(),
        }
    }

    #[allow(unused_mut, clippy::let_and_return)]
    pub fn count_keywords(&self) -> HashMap<String, usize> {
        let mut keywords_count = HashMap::new();

        // Think of an algorithm

        keywords_count
    }

    #[allow(unused_variables)]
    pub fn contains_keyword(&self, keyword: &str) -> bool {
        // Think of an algorithm

        false
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profile.name.is_empty() {
            return Err("Name is required".to_string());
        }

        if self.profile.email.is_empty() {
            return Err("Email is required".to_string());
        }

        // Could add more validation here

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub additional_links: HashMap<String, String>,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            location: "".to_string(),
            website: "".to_string(),
            linkedin: "".to_string(),
            github: "".to_string(),
            summary: "".to_string(),
            title: "".to_string(),
            additional_links: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Location {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub postal_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub company: String,
    pub title: String,
    pub location: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    pub description: String,
    #[serde(default)]
    pub achievements: Vec<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub field_of_study: String,
    pub location: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    #[serde(default)]
    pub gpa: Option<f32>,
    #[serde(default)]
    pub courses: Vec<String>,
    #[serde(default)]
    pub achievements: Vec<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Skills {
    #[serde(default)]
    pub technical: Vec<Skill>,
    #[serde(default)]
    pub soft: Vec<Skill>,
    #[serde(default)]
    pub languages: Vec<Skill>,
    #[serde(default)]
    pub tools: Vec<Skill>,
    #[serde(default)]
    pub other: Vec<Skill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub years: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub github: Option<String>,
    #[serde(default)]
    pub technologies: Vec<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    #[serde(default)]
    pub highlights: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Certification {
    pub name: String,
    pub issuer: String,
    pub date_obtained: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    #[serde(default)]
    pub credential_id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Language {
    pub name: String,
    pub proficiency: LanguageProficiency,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LanguageProficiency {
    Elementary,
    Limited,
    Professional,
    FullProfessional,
    Native,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Publication {
    pub title: String,
    pub publisher: String,
    pub published_date: Option<NaiveDate>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Volunteer {
    pub organization: String,
    pub role: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub current: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeMetadata {
    #[serde(default = "Utc::now")]
    pub last_updated: DateTime<Utc>,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub custom_fields: HashMap<String, String>,
}

impl Default for ResumeMetadata {
    fn default() -> Self {
        Self {
            last_updated: Utc::now(),
            version: "1.0.0".to_string(),
            template: "default".to_string(),
            custom_fields: HashMap::new(),
        }
    }
}
