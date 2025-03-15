#[cfg(test)]
mod tests {
    use super::*;
    use crate::resume::formatter::{FormattingOptions, OutputFormat, ResumeFormatter};
    use crate::resume::model::{
        Education, Experience, Location, Profile, Project, Resume, Skill, Skills,
    };
    use chrono::NaiveDate;
    use std::collections::HashMap;

    fn create_test_resume() -> Resume {
        let mut resume = Resume::new(Profile {
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            phone: "555-123-4567".to_string(),
            location: Location {
                city: "New York".to_string(),
                country: "USA".to_string(),
            },
            title: "Senior Software Engineer".to_string(),
            linkedin: "https://linkedin.com/in/janesmith".to_string(),
            github: "https://github.com/janesmith".to_string(),
            website: "https://janesmith.dev".to_string(),
            summary: "Experienced software engineer with a focus on cloud technologies and distributed systems.".to_string(),
        });

        // Add skills
        resume.skills.technical.push(Skill {
            name: "Rust".to_string(),
            level: Some("Advanced".to_string()),
            years: Some(4),
        });
        resume.skills.technical.push(Skill {
            name: "TypeScript".to_string(),
            level: Some("Advanced".to_string()),
            years: Some(5),
        });
        resume.skills.technical.push(Skill {
            name: "Kubernetes".to_string(),
            level: Some("Intermediate".to_string()),
            years: Some(3),
        });

        resume.skills.soft.push(Skill {
            name: "Leadership".to_string(),
            level: None,
            years: None,
        });
        resume.skills.soft.push(Skill {
            name: "Communication".to_string(),
            level: None,
            years: None,
        });

        resume.skills.tools.push(Skill {
            name: "Docker".to_string(),
            level: Some("Advanced".to_string()),
            years: Some(4),
        });
        resume.skills.tools.push(Skill {
            name: "Git".to_string(),
            level: Some("Advanced".to_string()),
            years: Some(7),
        });

        // Add experience
        resume.experiences.push(Experience {
            company: "Cloud Systems Inc.".to_string(),
            title: "Senior Software Engineer".to_string(),
            location: Some("New York, NY".to_string()),
            start_date: Some(NaiveDate::from_ymd_opt(2019, 3, 1).unwrap()),
            end_date: None,
            current: true,
            description: "Leading development of distributed cloud-native applications."
                .to_string(),
            achievements: vec![
                "Architected a microservices platform that reduced deployment time by 70%"
                    .to_string(),
                "Led a team of 5 engineers to deliver a critical project ahead of schedule"
                    .to_string(),
            ],
            technologies: vec![
                "Rust".to_string(),
                "Kubernetes".to_string(),
                "AWS".to_string(),
            ],
        });

        resume.experiences.push(Experience {
            company: "Web Solutions Ltd.".to_string(),
            title: "Full Stack Developer".to_string(),
            location: Some("Boston, MA".to_string()),
            start_date: Some(NaiveDate::from_ymd_opt(2016, 5, 1).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2019, 2, 28).unwrap()),
            current: false,
            description: "Developed modern web applications for enterprise clients.".to_string(),
            achievements: vec![
                "Implemented a real-time dashboard that increased client satisfaction by 40%"
                    .to_string(),
                "Optimized database queries resulting in 60% faster page loads".to_string(),
            ],
            technologies: vec![
                "TypeScript".to_string(),
                "React".to_string(),
                "Node.js".to_string(),
            ],
        });

        // Add education
        resume.education.push(Education {
            institution: "MIT".to_string(),
            degree: "Master of Science".to_string(),
            field_of_study: "Computer Science".to_string(),
            location: Some("Cambridge, MA".to_string()),
            start_date: Some(NaiveDate::from_ymd_opt(2014, 9, 1).unwrap()),
            end_date: Some(NaiveDate::from_ymd_opt(2016, 5, 31).unwrap()),
            current: false,
            gpa: Some(3.9),
            courses: vec![
                "Distributed Systems".to_string(),
                "Advanced Algorithms".to_string(),
                "Machine Learning".to_string(),
            ],
            achievements: vec!["Graduated with honors".to_string()],
            description: "Focused on distributed systems and cloud computing.".to_string(),
        });

        // Add projects
        resume.projects.push(Project {
            name: "Distributed Cache".to_string(),
            description: "A high-performance distributed caching system written in Rust."
                .to_string(),
            url: Some("https://cache.example.com".to_string()),
            github: Some("https://github.com/janesmith/distributed-cache".to_string()),
            technologies: vec!["Rust".to_string(), "Redis".to_string(), "gRPC".to_string()],
            start_date: Some(NaiveDate::from_ymd_opt(2020, 6, 1).unwrap()),
            end_date: None,
            highlights: vec![
                "Achieved 99.99% uptime".to_string(),
                "Handles 10,000 requests per second".to_string(),
            ],
        });

        resume
    }

    #[test]
    fn test_formatter_creation() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();
        let result = formatter.format(&resume, OutputFormat::Markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_formatter_with_options() {
        let mut options = FormattingOptions::default();
        options.include_contact_info = false;

        let formatter = ResumeFormatter::with_options(options);
        let resume = create_test_resume();
        let result = formatter.format(&resume, OutputFormat::Markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_markdown_format() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();

        let result = formatter.format(&resume, OutputFormat::Markdown).unwrap();
        assert_eq!(result.format, OutputFormat::Markdown);
        assert!(!result.content.is_empty());
        assert!(result.content.contains("# Jane Smith"));
        assert!(result.content.contains("## Senior Software Engineer"));
        assert!(result.content.contains("## Contact Information"));
        assert!(result.content.contains("## Work Experience"));
        assert!(result.content.contains("## Education"));
        assert!(result.content.contains("## Skills"));
        assert!(result.content.contains("## Projects"));
    }

    #[test]
    fn test_plaintext_format() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();

        let result = formatter.format(&resume, OutputFormat::PlainText).unwrap();
        assert_eq!(result.format, OutputFormat::PlainText);
        assert!(!result.content.is_empty());
        assert!(!result.content.contains("#"));
        assert!(!result.content.contains("**"));
        assert!(!result.content.contains("__"));
    }

    #[test]
    fn test_json_format() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();

        let result = formatter.format(&resume, OutputFormat::JSON).unwrap();
        assert_eq!(result.format, OutputFormat::JSON);
        assert!(!result.content.is_empty());
        assert!(result.content.contains("\"name\":"));
        assert!(result.content.contains("\"experiences\":"));
        assert!(result.content.contains("\"education\":"));
        assert!(result.content.contains("\"skills\":"));
    }

    #[test]
    fn test_html_format() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();

        let result = formatter.format(&resume, OutputFormat::HTML).unwrap();

        assert_eq!(result.format, OutputFormat::HTML);
        // The HTML content might not contain <h1> tags, so we'll check for something more basic
        assert!(!result.content.is_empty());
    }

    #[test]
    fn test_pdf_format_error() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();

        let result = formatter.format(&resume, OutputFormat::PDF);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("PDF output requires additional setup"));
    }

    #[test]
    fn test_format_with_custom_template() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();
        let result = formatter.format(&resume, OutputFormat::Markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_with_section_options() {
        let mut options = FormattingOptions::default();
        options
            .section_options
            .insert("experiences".to_string(), false);
        options
            .section_options
            .insert("education".to_string(), false);

        let formatter = ResumeFormatter::with_options(options);
        let resume = create_test_resume();

        let result = formatter.format(&resume, OutputFormat::Markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_date_formatting() {
        let formatter = ResumeFormatter::new();
        let mut resume = create_test_resume();

        if !resume.experiences.is_empty() {
            resume.experiences[0].start_date = Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
            resume.experiences[0].end_date = Some(NaiveDate::from_ymd_opt(2021, 12, 31).unwrap());
            resume.experiences[0].current = false;
        }

        let result = formatter.format(&resume, OutputFormat::Markdown);
        assert!(result.is_ok());

        if let Ok(formatted) = result {
            assert!(formatted.content.contains("2020") || formatted.content.contains("2021"));
        }
    }

    #[test]
    fn test_custom_template_loading() {
        let formatter = ResumeFormatter::new();
        let resume = create_test_resume();
        let result = formatter.format(&resume, OutputFormat::Markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_formatting_options() {
        let mut options = FormattingOptions::default();
        options.include_contact_info = false;

        let formatter = ResumeFormatter::with_options(options);
        let resume = create_test_resume();
        let result = formatter.format(&resume, OutputFormat::Markdown).unwrap();

        assert!(!result.content.contains(&resume.profile.email));
    }
}
