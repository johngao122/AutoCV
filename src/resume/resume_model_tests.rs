use crate::resume::model::*;
use chrono::NaiveDate;
use pretty_assertions::assert_eq;
use rstest::rstest;

#[test]
fn test_create_empty_resume() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john.gao@example.com".to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile.clone());

    assert_eq!(resume.profile.name, "John Gao");
    assert_eq!(resume.profile.email, "john.gao@example.com");
    assert!(resume.experiences.is_empty());
    assert!(resume.education.is_empty());
}

#[rstest]
#[case("", "john@example.com", "Name is required")]
#[case("John Gao", "", "Email is required")]
#[case("John Gao", "invalid-email", "Invalid email format")]
fn test_resume_validation(#[case] name: &str, #[case] email: &str, #[case] expected_error: &str) {
    let profile = Profile {
        name: name.to_string(),
        email: email.to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile);
    match resume.validate() {
        Ok(_) => panic!("Expected error for invalid resume"),
        Err(e) => assert_eq!(e, expected_error),
    }
}

#[test]
fn test_valid_resume() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        ..Profile::default()
    };

    let mut resume = Resume::new(profile);
    resume.skills.technical.push(Skill {
        name: "Rust".to_string(),
        level: None,
        years: None,
    });
    assert!(resume.validate().is_ok());
}

#[test]
fn test_experience_date_validation() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        ..Profile::default()
    };

    let mut resume = Resume::new(profile);
    resume.skills.technical.push(Skill {
        name: "Rust".to_string(),
        level: None,
        years: None,
    });

    resume.experiences.push(Experience {
        company: "Tech Corp".to_string(),
        title: "Engineer".to_string(),
        location: None,
        start_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()),
        current: false,
        description: "Work description".to_string(),
        achievements: vec![],
        technologies: vec![],
    });

    assert_eq!(
        resume.validate(),
        Err("Invalid dates for experience at Tech Corp: end date before start date".to_string())
    );
}

#[test]
fn test_education_validation() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        ..Profile::default()
    };

    let mut resume = Resume::new(profile);
    resume.skills.technical.push(Skill {
        name: "Rust".to_string(),
        level: None,
        years: None,
    });

    resume.education.push(Education {
        institution: "University".to_string(),
        degree: "BS".to_string(),
        field_of_study: "Computer Science".to_string(),
        location: None,
        start_date: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()),
        end_date: Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()),
        current: false,
        gpa: Some(4.5),
        courses: vec![],
        achievements: vec![],
        description: "".to_string(),
    });

    let result = resume.validate();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Invalid dates for education at University"));

    resume.education[0].start_date = Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
    resume.education[0].end_date = Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

    let result = resume.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid GPA for University"));
}

#[test]
fn test_url_validation() {
    let mut profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        linkedin: "linkedin.com/in/johndoe".to_string(),
        github: "github.com/johndoe".to_string(),
        ..Profile::default()
    };

    let mut resume = Resume::new(profile.clone());
    resume.skills.technical.push(Skill {
        name: "Rust".to_string(),
        level: None,
        years: None,
    });

    assert_eq!(
        resume.validate(),
        Err("LinkedIn URL must start with https://".to_string())
    );

    profile.linkedin = "https://linkedin.com/in/johndoe".to_string();
    resume = Resume::new(profile.clone());
    resume.skills.technical.push(Skill {
        name: "Rust".to_string(),
        level: None,
        years: None,
    });

    assert_eq!(
        resume.validate(),
        Err("GitHub URL must start with https://".to_string())
    );
}

#[test]
fn test_keyword_counting() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        title: "Senior Software Engineer".to_string(),
        summary: "Experienced in AWS and Kubernetes".to_string(),
        ..Profile::default()
    };

    let mut resume = Resume::new(profile);
    resume.skills.technical.push(Skill {
        name: "AWS".to_string(),
        level: None,
        years: None,
    });

    let keywords = resume.count_keywords();
    assert!(keywords.contains_key("amazon web services"));
    assert!(keywords.contains_key("kubernetes"));
    assert!(keywords.contains_key("software"));
    assert!(keywords.contains_key("engineer"));

    assert!(!keywords.contains_key("in"));
    assert!(!keywords.contains_key("and"));
}

#[test]
fn test_keyword_normalization() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        summary: "Experience with AWS, K8s, and CI/CD pipelines".to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile);
    let keywords = resume.count_keywords();

    assert!(keywords.contains_key("amazon web services"));
    assert!(keywords.contains_key("kubernetes"));
    assert!(keywords.contains_key("continuous integration / continuous deployment"));
}

#[test]
fn test_contains_keyword() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        summary: "Experience with AWS and Kubernetes".to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile);
    assert!(resume.contains_keyword("AWS"));
    assert!(resume.contains_keyword("amazon web services"));
    assert!(resume.contains_keyword("kubernetes"));
    assert!(!resume.contains_keyword("docker"));
}

#[test]
fn test_skills_validation() {
    let profile = Profile {
        name: "John Gao".to_string(),
        email: "john@example.com".to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile);
    assert_eq!(
        resume.validate(),
        Err("At least one technical skill is required".to_string())
    );
}
