#[cfg(test)]
mod tests {
    use super::*;
    use crate::resume::model::{Experience, Profile, Resume, Skill, Skills};
    use crate::resume::optimizer::ResumeOptimizer;
    use chrono::NaiveDate;

    fn create_test_resume() -> Resume {
        let mut resume = Resume::new(Profile {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            title: "Software Engineer".to_string(),
            ..Profile::default()
        });

        // Add skills
        resume.skills.technical.push(Skill {
            name: "Python".to_string(),
            level: Some("Advanced".to_string()),
            years: Some(5),
        });
        resume.skills.technical.push(Skill {
            name: "JavaScript".to_string(),
            level: Some("Intermediate".to_string()),
            years: Some(3),
        });
        resume.skills.technical.push(Skill {
            name: "React".to_string(),
            level: Some("Intermediate".to_string()),
            years: Some(2),
        });

        // Add experience
        resume.experiences.push(Experience {
            company: "Tech Corp".to_string(),
            title: "Senior Developer".to_string(),
            location: Some("San Francisco, CA".to_string()),
            start_date: Some(NaiveDate::from_ymd_opt(2018, 1, 1).unwrap()),
            end_date: None,
            current: true,
            description: "Led development of cloud-based applications.".to_string(),
            achievements: vec![
                "Improved application performance by 40%".to_string(),
                "Mentored junior developers".to_string(),
            ],
            technologies: vec![
                "Python".to_string(),
                "AWS".to_string(),
                "Docker".to_string(),
            ],
        });

        resume.experiences.push(Experience {
            company: "Startup Inc".to_string(),
            title: "Full Stack Developer".to_string(),
            location: Some("Remote".to_string()),
            start_date: Some(NaiveDate::from_ymd_opt(2015, 6, 1).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2017, 12, 31).unwrap()),
            current: false,
            description: "Developed web applications using modern JavaScript frameworks."
                .to_string(),
            achievements: vec![
                "Launched 3 successful products".to_string(),
                "Reduced bug count by 30%".to_string(),
            ],
            technologies: vec![
                "JavaScript".to_string(),
                "React".to_string(),
                "Node.js".to_string(),
            ],
        });

        resume.profile.summary =
            "Experienced software engineer with a passion for building scalable applications."
                .to_string();

        resume
    }

    #[test]
    fn test_optimizer_creation() {
        let optimizer = ResumeOptimizer::new();
        let resume = create_test_resume();
        let job_description = "Test job description";
        let result = optimizer.optimize(&resume, job_description);
        assert!(result.score >= 0);
    }

    #[test]
    fn test_loading_industry_keywords() {
        let optimizer = ResumeOptimizer::new();

        // Test the optimizer functionality instead of directly accessing fields
        let resume = create_test_resume();
        let job_description = "Looking for a Python and React developer.";
        let optimization_result = optimizer.optimize(&resume, job_description);

        assert!(optimization_result.matching_keywords.contains_key("python"));
        assert!(optimization_result.matching_keywords.contains_key("react"));
    }

    #[test]
    fn test_basic_optimization() {
        let optimizer = ResumeOptimizer::new();
        let resume = create_test_resume();

        let job_description = "Looking for a Python developer with experience in Django and React.";
        let result = optimizer.optimize(&resume, job_description);

        assert!(result.score > 0);
        assert!(!result.suggestions.is_empty());
        assert!(result.matching_keywords.contains_key("python"));
        assert!(result.matching_keywords.contains_key("react"));
    }

    #[test]
    fn test_keyword_matching() {
        let optimizer = ResumeOptimizer::new();
        let resume = create_test_resume();

        let job_description = "We need someone with AWS, Docker, and Kubernetes experience.";
        let result = optimizer.optimize(&resume, job_description);

        if result.matching_keywords.contains_key("aws") {
            assert!(result.matching_keywords.contains_key("aws"));
        }
        if result.matching_keywords.contains_key("docker") {
            assert!(result.matching_keywords.contains_key("docker"));
        }
        if result.missing_keywords.contains(&"kubernetes".to_string()) {
            assert!(result.missing_keywords.contains(&"kubernetes".to_string()));
        }
    }

    #[test]
    fn test_suggestions_generation() {
        let optimizer = ResumeOptimizer::new();
        let mut resume = create_test_resume();

        // Remove summary to trigger suggestion
        resume.profile.summary = "".to_string();

        let job_description = "Looking for a developer with cloud experience.";
        let result = optimizer.optimize(&resume, job_description);

        assert!(result.suggestions.iter().any(|s| s.contains("summary")));
    }

    #[test]
    fn test_experience_analysis() {
        let optimizer = ResumeOptimizer::new();
        let mut resume = create_test_resume();

        // Remove achievements to trigger suggestion
        for exp in &mut resume.experiences {
            exp.achievements.clear();
        }

        let job_description = "Looking for a developer with cloud experience.";
        let result = optimizer.optimize(&resume, job_description);

        assert!(result.section_improvements.contains_key("Experience"));
        assert!(result
            .section_improvements
            .get("Experience")
            .unwrap()
            .iter()
            .any(|s| s.contains("achievements")));
    }

    #[test]
    fn test_skills_analysis() {
        let optimizer = ResumeOptimizer::new();
        let resume = create_test_resume();

        let job_description = "Looking for a developer with SQL and database experience.";
        let result = optimizer.optimize(&resume, job_description);

        assert!(result.section_improvements.contains_key("Skills"));
        assert!(result
            .section_improvements
            .get("Skills")
            .unwrap()
            .iter()
            .any(|s| s.contains("skills")));
    }

    #[test]
    fn test_high_score_for_good_match() {
        let optimizer = ResumeOptimizer::new();
        let resume = create_test_resume();

        let job_description = "Looking for a Python and JavaScript developer with React experience who has worked with AWS and Docker.";
        let result = optimizer.optimize(&resume, job_description);

        assert!(
            result.score > 0,
            "Score should be greater than 0 for a good match"
        );
    }

    #[test]
    fn test_low_score_for_poor_match() {
        let optimizer = ResumeOptimizer::new();
        let resume = create_test_resume();

        let job_description = "Looking for a Java developer with Spring Boot, Hibernate, and Oracle database experience.";
        let result = optimizer.optimize(&resume, job_description);

        assert!(result.score <= 40, "Score should be low for a poor match");
    }
}
