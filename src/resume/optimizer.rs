use crate::resume::model::Resume;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct OptimizationResult {
    pub score: u8,
    pub missing_keywords: Vec<String>,
    pub matching_keywords: HashMap<String, usize>,
    pub overused_keywords: Vec<String>,
    pub suggestions: Vec<String>,
    pub section_improvements: HashMap<String, Vec<String>>,
}

pub struct ResumeOptimizer {
    industry_keywords: HashMap<String, HashSet<String>>,
    action_verbs: HashSet<String>,
    weak_terms: HashSet<String>,
}

impl Default for ResumeOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl ResumeOptimizer {
    pub fn new() -> Self {
        let mut optimizer = Self {
            industry_keywords: HashMap::new(),
            action_verbs: HashSet::new(),
            weak_terms: HashSet::new(),
        };
        optimizer.load_industry_keywords();
        optimizer
    }

    fn load_industry_keywords(&mut self) {
        self.load_keywords_for_industry("software development").ok();
        self.load_default_keywords();
    }

    pub fn load_keywords_for_industry(&mut self, industry: &str) -> Result<(), String> {
        //Placeholder data

        if industry.to_lowercase().contains("software")
            || industry.to_lowercase().contains("developer")
            || industry.to_lowercase().contains("engineering")
        {
            let keywords = vec![
                "python",
                "javascript",
                "typescript",
                "rust",
                "java",
                "c++",
                "react",
                "node.js",
                "docker",
                "kubernetes",
                "ci/cd",
                "aws",
                "azure",
                "gcp",
                "microservices",
                "rest api",
                "git",
                "agile",
                "scrum",
                "devops",
                "test-driven development",
                "distributed systems",
                "database",
                "sql",
                "nosql",
                "mongodb",
                "postgresql",
                "full-stack",
                "backend",
                "frontend",
                "mobile",
                "machine learning",
                "data science",
                "algorithms",
                "system design",
            ];

            let keyword_set: HashSet<String> = keywords.into_iter().map(String::from).collect();
            self.industry_keywords
                .insert(industry.to_string(), keyword_set);
            Ok(())
        } else {
            Err(format!("No keywords found for industry: {}", industry))
        }
    }

    pub fn load_default_keywords(&mut self) {
        let action_verbs = vec![
            "achieved",
            "improved",
            "solved",
            "delivered",
            "created",
            "implemented",
            "developed",
            "designed",
            "launched",
            "managed",
            "led",
            "coordinated",
            "increased",
            "decreased",
            "reduced",
            "optimized",
            "negotiated",
            "trained",
            "mentored",
            "supervised",
            "analyzed",
            "researched",
            "integrated",
            "authored",
        ];
        self.action_verbs = action_verbs.into_iter().map(String::from).collect();

        let weak_terms = vec![
            "responsible for",
            "duties included",
            "worked on",
            "helped with",
            "assisted",
            "was tasked with",
            "participated in",
            "familiar with",
            "exposure to",
            "involved in",
            "experience with",
            "very",
            "various",
            "numerous",
            "different",
            "several",
            "many",
            "part of",
            "things",
        ];
        self.weak_terms = weak_terms.into_iter().map(String::from).collect();
    }

    pub fn optimize(&self, resume: &Resume, job_description: &str) -> OptimizationResult {
        let mut result = OptimizationResult::default();

        let job_keywords = self.extract_keywords(job_description);

        let resume_keywords = resume.count_keywords();
        let mut matching_keywords = HashMap::new();
        for keyword in job_keywords.keys() {
            if let Some(resume_count) = resume_keywords.get(keyword) {
                matching_keywords.insert(keyword.clone(), *resume_count);
            }
        }

        let mut missing_keywords = Vec::new();
        for (keyword, importance) in &job_keywords {
            if !resume_keywords.contains_key(keyword) && *importance > 1 {
                missing_keywords.push(keyword.clone());
            }
        }

        let mut overused_keywords = Vec::new();
        for (keyword, count) in &resume_keywords {
            if *count > 4 {
                overused_keywords.push(keyword.clone());
            }
        }

        let total_important_keywords = job_keywords
            .iter()
            .filter(|(_, importance)| **importance > 1)
            .count();

        let matched_important_keywords = job_keywords
            .iter()
            .filter(|(keyword, importance)| {
                **importance > 1 && resume_keywords.contains_key(*keyword)
            })
            .count();

        let score = if total_important_keywords > 0 {
            ((matched_important_keywords as f32 / total_important_keywords as f32) * 100.0) as u8
        } else {
            0
        };

        self.add_general_suggestions(resume, &mut result);

        self.analyze_experience_section(resume, &job_keywords, &mut result);
        self.analyze_skills_section(resume, &job_keywords, &mut result);

        result.score = score;
        result.missing_keywords = missing_keywords;
        result.matching_keywords = matching_keywords;
        result.overused_keywords = overused_keywords;

        result
    }

    fn extract_keywords(&self, text: &str) -> HashMap<String, usize> {
        let mut keywords = HashMap::new();
        let text = text.to_lowercase();

        // improve with NLP?
        let re = Regex::new(r"\b[a-zA-Z0-9][\w-]*\b").unwrap();
        for word in re.find_iter(&text) {
            let word = word.as_str().to_string();
            if word.len() > 2 {
                *keywords.entry(word).or_insert(0) += 1;
            }
        }

        for industry_keywords in self.industry_keywords.values() {
            for keyword in industry_keywords {
                let count = text.matches(&keyword.to_lowercase()).count();
                if count > 0 {
                    keywords.insert(keyword.clone(), count + 1);
                }
            }
        }

        keywords
    }

    fn add_general_suggestions(&self, resume: &Resume, result: &mut OptimizationResult) {
        if resume.profile.summary.is_empty() {
            result
                .suggestions
                .push("Add a professional summary to highlight your qualifications".to_string());
        } else if resume.profile.summary.split_whitespace().count() < 20 {
            result.suggestions.push(
                "Expand your professional summary to better highlight your experience".to_string(),
            );
        }

        if resume.profile.phone.is_empty() {
            result
                .suggestions
                .push("Add your phone number to contact information".to_string());
        }

        if resume.profile.linkedin.is_empty() {
            result
                .suggestions
                .push("Add your LinkedIn profile to contact information".to_string());
        }

        let mut has_weak_terms = false;
        let mut has_action_verbs = false;

        for exp in &resume.experiences {
            let description = exp.description.to_lowercase();

            for weak in &self.weak_terms {
                if description.contains(&weak.to_lowercase()) {
                    has_weak_terms = true;
                    break;
                }
            }

            for verb in &self.action_verbs {
                if description.contains(&verb.to_lowercase()) {
                    has_action_verbs = true;
                    break;
                }
            }
        }

        if has_weak_terms {
            result
                .suggestions
                .push("Replace passive or weak phrases with strong action verbs".to_string());
        }

        if !has_action_verbs {
            result
                .suggestions
                .push("Use more action verbs to describe your accomplishments".to_string());
        }
    }

    fn analyze_experience_section(
        &self,
        resume: &Resume,
        job_keywords: &HashMap<String, usize>,
        result: &mut OptimizationResult,
    ) {
        let mut improvements = Vec::new();

        let has_achievements = resume
            .experiences
            .iter()
            .any(|exp| !exp.achievements.is_empty());

        if !has_achievements {
            improvements.push("Add quantifiable achievements to your work experience".to_string());
        }

        let mut missing_in_experience = Vec::new();
        for (keyword, importance) in job_keywords {
            if *importance > 1 {
                let found = resume.experiences.iter().any(|exp| {
                    exp.description
                        .to_lowercase()
                        .contains(&keyword.to_lowercase())
                        || exp.title.to_lowercase().contains(&keyword.to_lowercase())
                        || exp
                            .technologies
                            .iter()
                            .any(|tech| tech.to_lowercase().contains(&keyword.to_lowercase()))
                });

                if !found {
                    missing_in_experience.push(keyword.clone());
                }
            }
        }

        if !missing_in_experience.is_empty() && missing_in_experience.len() <= 5 {
            improvements.push(format!(
                "Consider incorporating these keywords into your experience section: {}",
                missing_in_experience.join(", ")
            ));
        } else if !missing_in_experience.is_empty() {
            improvements.push("Many important keywords from the job description are missing from your experience section".to_string());
        }

        if !improvements.is_empty() {
            result
                .section_improvements
                .insert("Experience".to_string(), improvements);
        }
    }

    fn analyze_skills_section(
        &self,
        resume: &Resume,
        job_keywords: &HashMap<String, usize>,
        result: &mut OptimizationResult,
    ) {
        let mut improvements = Vec::new();

        if resume.skills.technical.is_empty() && !resume.skills.other.is_empty() {
            improvements.push(
                "Organize your skills into categories (technical, soft, tools, etc.)".to_string(),
            );
        }

        let all_skills: Vec<String> = resume
            .skills
            .technical
            .iter()
            .chain(resume.skills.soft.iter())
            .chain(resume.skills.tools.iter())
            .chain(resume.skills.other.iter())
            .map(|skill| skill.name.clone())
            .collect();

        let mut missing_skills = Vec::new();
        for (keyword, importance) in job_keywords {
            if *importance > 1 {
                let is_skill = all_skills
                    .iter()
                    .any(|skill| skill.to_lowercase().contains(&keyword.to_lowercase()));

                if !is_skill {
                    missing_skills.push(keyword.clone());
                }
            }
        }

        if !missing_skills.is_empty() && missing_skills.len() <= 5 {
            improvements.push(format!(
                "Consider adding these skills if you have them: {}",
                missing_skills.join(", ")
            ));
        } else if !missing_skills.is_empty() {
            improvements.push("Many important skills from the job description are missing from your skills section".to_string());
        }

        if !improvements.is_empty() {
            result
                .section_improvements
                .insert("Skills".to_string(), improvements);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resume::model::{Profile, Skill};

    #[test]
    fn test_optimizer_creation() {
        let optimizer = ResumeOptimizer::new();
        assert!(!optimizer.action_verbs.is_empty());
        assert!(!optimizer.weak_terms.is_empty());
    }

    #[test]
    fn test_loading_industry_keywords() {
        let mut optimizer = ResumeOptimizer::new();
        let result = optimizer.load_keywords_for_industry("software development");
        assert!(result.is_ok());

        let keywords = optimizer
            .industry_keywords
            .get("software development")
            .unwrap();
        assert!(keywords.contains("python"));
        assert!(keywords.contains("react"));
    }

    #[test]
    fn test_basic_optimization() {
        let optimizer = ResumeOptimizer::new();

        let profile = Profile {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            ..Profile::default()
        };

        let mut resume = Resume::new(profile);
        resume.skills.technical.push(Skill {
            name: "Python".to_string(),
            level: None,
            years: None,
        });

        let job_description = "Looking for a Python developer with experience in Django and React.";

        let result = optimizer.optimize(&resume, job_description);

        assert!(result.score > 0);
        assert!(!result.suggestions.is_empty());
        assert!(result.matching_keywords.contains_key("python"));
        assert!(
            result.missing_keywords.contains(&"django".to_string())
                || result.missing_keywords.contains(&"react".to_string())
        );
    }
}
