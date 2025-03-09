use crate::resume::model::*;
use pretty_assertions::{assert_eq, assert_ne};
use rstest::rstest;

#[test]
fn test_create_empty_resume() {
    let profile = Profile {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile.clone());

    assert_eq!(resume.profile.name, "John Doe");
    assert_eq!(resume.profile.email, "john@example.com");
    assert!(resume.experiences.is_empty());
    assert!(resume.education.is_empty());
}

#[rstest]
#[case("", "john@example.com", "Profile name is required")]
#[case("John Doe", "", "Profile email is required")]
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
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        ..Profile::default()
    };

    let resume = Resume::new(profile);
    assert!(resume.validate().is_ok());
}
