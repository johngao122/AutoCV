use chrono::{DateTime, NaiveDate, Utc};
use regex::Regex;
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
        let mut text_content = String::new();

        text_content.push_str(&self.profile.summary);
        text_content.push_str(&self.profile.title);
        text_content.push_str(&self.profile.name);

        for experience in &self.experiences {
            text_content.push_str(&experience.title);
            text_content.push_str(&experience.description);
            for achievement in &experience.achievements {
                text_content.push_str(achievement);
            }
            for technology in &experience.technologies {
                text_content.push_str(technology);
            }
        }

        for education in &self.education {
            text_content.push_str(&education.degree);
            text_content.push_str(&education.field_of_study);
            text_content.push_str(&education.description);
            for course in &education.courses {
                text_content.push_str(course);
            }
        }

        for skill in &self.skills.technical {
            text_content.push_str(&skill.name);
        }

        for skill in &self.skills.soft {
            text_content.push_str(&skill.name);
        }

        for skill in &self.skills.languages {
            text_content.push_str(&skill.name);
        }

        for skill in &self.skills.tools {
            text_content.push_str(&skill.name);
        }

        for project in &self.projects {
            text_content.push_str(&project.name);
            text_content.push_str(&project.description);
            for technology in &project.technologies {
                text_content.push_str(technology);
            }
        }

        let text_content = text_content.to_lowercase();

        let re = Regex::new(r"\b[a-zA-Z0-9-]+\b").unwrap();
        for word in re.find_iter(&text_content) {
            let word = word.as_str().to_string();
            if !Self::is_common_word(&word) {
                let normalized = Self::normalize_keyword(&word);
                *keywords_count.entry(normalized).or_insert(0) += 1;
            }
        }

        keywords_count
    }

    fn is_common_word(word: &str) -> bool {
        let stopwords = [
            "the", "and", "in", "of", "on", "with", "for", "to", "a", "an", "at", "by", "is",
            "this", "that", "it", "as", "or", "be", "are", "was", "were", "not",
        ];
        stopwords.contains(&word)
    }

    pub fn contains_keyword(&self, keyword: &str) -> bool {
        let normalized_keyword = Self::normalize_keyword(keyword);
        let keywords = self.count_keywords();
        keywords.contains_key(&normalized_keyword)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profile.name.is_empty() {
            return Err("Name is required".to_string());
        }
        if self.profile.email.is_empty() {
            return Err("Email is required".to_string());
        }

        if !self.profile.email.contains('@') {
            return Err("Invalid email format".to_string());
        }

        for exp in &self.experiences {
            if let (Some(start), Some(end)) = (exp.start_date, exp.end_date) {
                if end < start {
                    return Err(format!(
                        "Invalid dates for experience at {}: end date before start date",
                        exp.company
                    ));
                }
            }
        }

        for edu in &self.education {
            if let (Some(start), Some(end)) = (edu.start_date, edu.end_date) {
                if end < start {
                    return Err(format!(
                        "Invalid dates for education at {}: end date before start date",
                        edu.institution
                    ));
                }
            }
            if let Some(gpa) = edu.gpa {
                if !(0.0..=4.0).contains(&gpa) {
                    return Err(format!(
                        "Invalid GPA for {}: must be between 0.0 and 4.0",
                        edu.institution
                    ));
                }
            }
        }

        if !self.profile.linkedin.is_empty() && !self.profile.linkedin.starts_with("https://") {
            return Err("LinkedIn URL must start with https://".to_string());
        }
        if !self.profile.github.is_empty() && !self.profile.github.starts_with("https://") {
            return Err("GitHub URL must start with https://".to_string());
        }

        if self.skills.technical.is_empty() {
            return Err("At least one technical skill is required".to_string());
        }

        Ok(())
    }

    fn get_tech_acronyms() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            // Cloud & Infrastructure
            ("AWS", "Amazon Web Services"),
            ("GCP", "Google Cloud Platform"),
            ("Azure", "Microsoft Azure"),
            ("IaaS", "Infrastructure as a Service"),
            ("PaaS", "Platform as a Service"),
            ("SaaS", "Software as a Service"),
            ("CDN", "Content Delivery Network"),
            ("DNS", "Domain Name System"),
            // Programming & Software Development
            ("API", "Application Programming Interface"),
            ("SDK", "Software Development Kit"),
            ("CLI", "Command Line Interface"),
            ("GUI", "Graphical User Interface"),
            ("OOP", "Object-Oriented Programming"),
            ("FP", "Functional Programming"),
            ("CI/CD", "Continuous Integration / Continuous Deployment"),
            ("MVC", "Model-View-Controller"),
            ("TDD", "Test-Driven Development"),
            ("ORM", "Object-Relational Mapping"),
            // Data Science & AI
            ("ML", "Machine Learning"),
            ("AI", "Artificial Intelligence"),
            ("NLP", "Natural Language Processing"),
            ("CV", "Computer Vision"),
            ("DL", "Deep Learning"),
            ("RNN", "Recurrent Neural Network"),
            ("CNN", "Convolutional Neural Network"),
            ("LSTM", "Long Short-Term Memory"),
            ("GAN", "Generative Adversarial Network"),
            ("ETL", "Extract, Transform, Load"),
            // Databases & Storage
            ("SQL", "Structured Query Language"),
            ("NoSQL", "Not Only SQL"),
            ("RDBMS", "Relational Database Management System"),
            ("ACID", "Atomicity, Consistency, Isolation, Durability"),
            ("OLTP", "Online Transaction Processing"),
            ("OLAP", "Online Analytical Processing"),
            // Networking & Security
            (
                "TCP/IP",
                "Transmission Control Protocol / Internet Protocol",
            ),
            ("HTTP", "HyperText Transfer Protocol"),
            ("HTTPS", "HyperText Transfer Protocol Secure"),
            ("SSH", "Secure Shell"),
            ("SSL", "Secure Sockets Layer"),
            ("TLS", "Transport Layer Security"),
            ("VPN", "Virtual Private Network"),
            ("DDoS", "Distributed Denial of Service"),
            // DevOps & Tools
            ("K8s", "Kubernetes"),
            ("IaC", "Infrastructure as Code"),
            ("BPM", "Business Process Management"),
            ("VM", "Virtual Machine"),
            ("VCS", "Version Control System"),
            (
                "Git",
                "Version control system (not an acronym but widely used)",
            ),
        ])
    }

    fn normalize_keyword(word: &str) -> String {
        let word = word.to_lowercase();
        Self::get_tech_acronyms()
            .iter()
            .find(|(k, _)| k.to_lowercase() == word)
            .map(|(_, v)| v.to_string())
            .unwrap_or(word)
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
