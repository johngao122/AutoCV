use crate::resume::model::Resume;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Markdown,
    PlainText,
    JSON,
    HTML,
    PDF,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::PlainText => "txt",
            Self::JSON => "json",
            Self::HTML => "html",
            Self::PDF => "pdf",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormattingOptions {
    pub template: String,
    pub include_contact_info: bool,
    pub include_picture: bool,
    pub date_format: String,
    pub section_options: HashMap<String, bool>,
    pub custom_options: HashMap<String, String>,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        let mut section_options = HashMap::new();
        section_options.insert("experiences".to_string(), true);
        section_options.insert("education".to_string(), true);
        section_options.insert("skills".to_string(), true);
        section_options.insert("projects".to_string(), true);
        section_options.insert("certifications".to_string(), true);
        section_options.insert("languages".to_string(), true);
        section_options.insert("publications".to_string(), true);
        section_options.insert("volunteer".to_string(), true);

        Self {
            template: "modern".to_string(),
            include_contact_info: true,
            include_picture: false,
            date_format: "%B %Y".to_string(),
            section_options,
            custom_options: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct FormattingResult {
    pub content: String,
    pub format: OutputFormat,
    pub warnings: Vec<String>,
}

pub struct ResumeFormatter {
    templates: HashMap<String, String>,
    options: FormattingOptions,
}

impl Default for ResumeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl ResumeFormatter {
    pub fn new() -> Self {
        let mut formatter = Self {
            templates: HashMap::new(),
            options: FormattingOptions::default(),
        };

        formatter.load_default_templates();
        formatter
    }

    pub fn with_options(options: FormattingOptions) -> Self {
        let mut formatter = Self {
            templates: HashMap::new(),
            options,
        };

        formatter.load_default_templates();
        formatter
    }

    pub fn load_template_from_file(&mut self, name: &str, path: &Path) -> Result<(), String> {
        let template_content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read template file: {}", e))?;

        self.templates.insert(name.to_string(), template_content);
        Ok(())
    }

    fn load_default_templates(&mut self) {
        self.templates.insert(
            "modern".to_string(),
            include_str!("../templates/modern.md").to_string(),
        );

        self.templates.insert(
            "classic".to_string(),
            include_str!("../templates/classic.md").to_string(),
        );

        self.templates.insert(
            "minimal".to_string(),
            include_str!("../templates/minimal.md").to_string(),
        );
    }

    pub fn format(
        &self,
        resume: &Resume,
        format: OutputFormat,
    ) -> Result<FormattingResult, String> {
        let mut warnings = Vec::new();

        if !self.templates.contains_key(&self.options.template) {
            return Err(format!("Template '{}' not found", self.options.template));
        }

        let content = match format {
            OutputFormat::Markdown => self.format_markdown(resume, &mut warnings)?,
            OutputFormat::PlainText => self.format_plaintext(resume, &mut warnings)?,
            OutputFormat::JSON => self.format_json(resume, &mut warnings)?,
            OutputFormat::HTML => self.format_html(resume, &mut warnings)?,
            OutputFormat::PDF => return Err("PDF output requires additional setup".to_string()),
        };

        Ok(FormattingResult {
            content,
            format,
            warnings,
        })
    }

    fn format_markdown(
        &self,
        resume: &Resume,
        warnings: &mut Vec<String>,
    ) -> Result<String, String> {
        let _template = self
            .templates
            .get(&self.options.template)
            .ok_or_else(|| format!("Template '{}' not found", self.options.template))?;

        // hmm template engine goes here

        let mut content = String::new();

        content.push_str(&format!("# {}\n", resume.profile.name));

        if !resume.profile.title.is_empty() {
            content.push_str(&format!("## {}\n\n", resume.profile.title));
        }

        if self.options.include_contact_info {
            content.push_str("## Contact Information\n\n");

            if !resume.profile.email.is_empty() {
                content.push_str(&format!("- Email: {}\n", resume.profile.email));
            }

            if !resume.profile.phone.is_empty() {
                content.push_str(&format!("- Phone: {}\n", resume.profile.phone));
            }

            if !resume.profile.location.city.is_empty()
                || !resume.profile.location.country.is_empty()
            {
                let location = match (
                    resume.profile.location.city.is_empty(),
                    resume.profile.location.country.is_empty(),
                ) {
                    (false, false) => format!(
                        "{}, {}",
                        resume.profile.location.city, resume.profile.location.country
                    ),
                    (false, true) => resume.profile.location.city.clone(),
                    (true, false) => resume.profile.location.country.clone(),
                    (true, true) => String::new(),
                };
                if !location.is_empty() {
                    content.push_str(&format!("- Location: {}\n", location));
                }
            }

            if !resume.profile.linkedin.is_empty() {
                content.push_str(&format!("- LinkedIn: {}\n", resume.profile.linkedin));
            }

            if !resume.profile.github.is_empty() {
                content.push_str(&format!("- GitHub: {}\n", resume.profile.github));
            }

            if !resume.profile.website.is_empty() {
                content.push_str(&format!("- Website: {}\n", resume.profile.website));
            }

            content.push('\n');
        }

        if !resume.profile.summary.is_empty() {
            content.push_str("## Professional Summary\n\n");
            content.push_str(&resume.profile.summary);
            content.push_str("\n\n");
        }

        if self
            .options
            .section_options
            .get("experiences")
            .copied()
            .unwrap_or(true)
            && !resume.experiences.is_empty()
        {
            content.push_str("## Work Experience\n\n");

            for exp in &resume.experiences {
                content.push_str(&format!("### {} at {}\n", exp.title, exp.company));

                let date_str = self.format_date_range(exp.start_date, exp.end_date, exp.current);
                content.push_str(&format!("_{}_\n\n", date_str));

                if let Some(location) = &exp.location {
                    content.push_str(&format!("**Location:** {}\n\n", location));
                }

                if !exp.description.is_empty() {
                    content.push_str(&exp.description);
                    content.push_str("\n\n");
                }

                if !exp.achievements.is_empty() {
                    content.push_str("**Key Achievements:**\n\n");
                    for achievement in &exp.achievements {
                        content.push_str(&format!("- {}\n", achievement));
                    }
                    content.push('\n');
                }

                if !exp.technologies.is_empty() {
                    content.push_str("**Technologies:** ");
                    content.push_str(&exp.technologies.join(", "));
                    content.push_str("\n\n");
                }
            }
        }

        if self
            .options
            .section_options
            .get("education")
            .copied()
            .unwrap_or(true)
            && !resume.education.is_empty()
        {
            content.push_str("## Education\n\n");

            for edu in &resume.education {
                content.push_str(&format!("### {} in {}\n", edu.degree, edu.field_of_study));
                content.push_str(&format!("**{}**\n", edu.institution));

                let date_str = self.format_date_range(edu.start_date, edu.end_date, edu.current);
                content.push_str(&format!("_{}_\n\n", date_str));

                if let Some(gpa) = edu.gpa {
                    content.push_str(&format!("**GPA:** {:.2}\n\n", gpa));
                }

                if !edu.description.is_empty() {
                    content.push_str(&edu.description);
                    content.push_str("\n\n");
                }

                if !edu.courses.is_empty() {
                    content.push_str("**Relevant Courses:**\n\n");
                    for course in &edu.courses {
                        content.push_str(&format!("- {}\n", course));
                    }
                    content.push('\n');
                }

                if !edu.achievements.is_empty() {
                    content.push_str("**Achievements:**\n\n");
                    for achievement in &edu.achievements {
                        content.push_str(&format!("- {}\n", achievement));
                    }
                    content.push('\n');
                }
            }
        }

        if self
            .options
            .section_options
            .get("skills")
            .copied()
            .unwrap_or(true)
        {
            content.push_str("## Skills\n\n");

            if !resume.skills.technical.is_empty() {
                content.push_str("### Technical Skills\n\n");
                content.push_str(
                    &resume
                        .skills
                        .technical
                        .iter()
                        .map(|skill| skill.name.clone())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                content.push_str("\n\n");
            }

            if !resume.skills.soft.is_empty() {
                content.push_str("### Soft Skills\n\n");
                content.push_str(
                    &resume
                        .skills
                        .soft
                        .iter()
                        .map(|skill| skill.name.clone())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                content.push_str("\n\n");
            }

            if !resume.skills.tools.is_empty() {
                content.push_str("### Tools\n\n");
                content.push_str(
                    &resume
                        .skills
                        .tools
                        .iter()
                        .map(|skill| skill.name.clone())
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                content.push_str("\n\n");
            }

            if !resume.skills.languages.is_empty() {
                content.push_str("### Languages\n\n");
                for lang in &resume.skills.languages {
                    content.push_str(&format!("- {}\n", lang.name));
                }
                content.push('\n');
            }
        }

        if self
            .options
            .section_options
            .get("projects")
            .copied()
            .unwrap_or(true)
            && !resume.projects.is_empty()
        {
            content.push_str("## Projects\n\n");

            for project in &resume.projects {
                content.push_str(&format!("### {}\n\n", project.name));

                if !project.description.is_empty() {
                    content.push_str(&project.description);
                    content.push_str("\n\n");
                }

                if let Some(url) = &project.url {
                    content.push_str(&format!("**Link:** [Project Link]({})\n\n", url));
                }

                if let Some(github) = &project.github {
                    content.push_str(&format!("**GitHub:** [Repository]({})\n\n", github));
                }

                if !project.technologies.is_empty() {
                    content.push_str("**Technologies:** ");
                    content.push_str(&project.technologies.join(", "));
                    content.push_str("\n\n");
                }

                if !project.highlights.is_empty() {
                    content.push_str("**Highlights:**\n\n");
                    for highlight in &project.highlights {
                        content.push_str(&format!("- {}\n", highlight));
                    }
                    content.push('\n');
                }
            }
        }

        // other sections

        if resume.experiences.is_empty() {
            warnings.push("Resume doesn't have any work experiences".to_string());
        }

        if resume.education.is_empty() {
            warnings.push("Resume doesn't have any education entries".to_string());
        }

        if resume.skills.technical.is_empty() {
            warnings.push("Resume doesn't have any technical skills".to_string());
        }

        Ok(content)
    }

    fn format_plaintext(
        &self,
        resume: &Resume,
        warnings: &mut Vec<String>,
    ) -> Result<String, String> {
        let mut plain_text = String::new();
        let markdown = self.format_markdown(resume, warnings)?;

        let re = regex::Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();

        for line in markdown.lines() {
            let line = line
                .replace("#", "")
                .replace("**", "")
                .replace("*", "")
                .replace("_", "");

            let line = re.replace_all(&line, "$1 ($2)").to_string();

            plain_text.push_str(line.as_str());
            plain_text.push('\n');
        }

        Ok(plain_text)
    }

    fn format_json(&self, resume: &Resume, _warnings: &mut [String]) -> Result<String, String> {
        serde_json::to_string_pretty(resume).map_err(|e| format!("Failed to format as JSON: {}", e))
    }

    fn format_html(&self, resume: &Resume, warnings: &mut Vec<String>) -> Result<String, String> {
        let markdown = self.format_markdown(resume, warnings)?;

        // In a real implementation, this would use a proper Markdown-to-HTML converter
        // like pulldown-cmark, but for simplicity we'll do a basic conversion

        let mut html = String::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!(
            "<title>{}'s Resume</title>\n",
            resume.profile.name
        ));

        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; line-height: 1.6; max-width: 800px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("h1, h2, h3 { color: #333; }\n");
        html.push_str("h1 { border-bottom: 2px solid #333; padding-bottom: 10px; }\n");
        html.push_str(
            "h2 { border-bottom: 1px solid #ddd; padding-bottom: 5px; margin-top: 20px; }\n",
        );
        html.push_str("ul { margin-top: 5px; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        let mut content = markdown;
        content = content
            .replace("\n# ", "\n<h1>")
            .replace("\n## ", "\n<h2>")
            .replace("\n### ", "\n<h3>");
        content = content
            .replace(" #\n", "</h1>\n")
            .replace(" ##\n", "</h2>\n")
            .replace(" ###\n", "</h3>\n");

        content = content.replace("\n- ", "\n<li>").replace("\n* ", "\n<li>");
        content = regex::Regex::new(r"<li>(.*?)(\n|$)")
            .unwrap()
            .replace_all(&content, "<li>$1</li>$2")
            .to_string();
        content = regex::Regex::new(r"(<li>.*?</li>)+")
            .unwrap()
            .replace_all(&content, "<ul>$0</ul>")
            .to_string();

        content = content.replace("**", "<strong>").replace("__", "<strong>");
        let strong_count = content.matches("<strong>").count();
        if strong_count % 2 != 0 {
            warnings.push("Mismatched bold markers in content".to_string());
        }
        for _ in 0..(strong_count / 2) {
            content = content
                .replacen("**", "<strong>", 1)
                .replacen("**", "</strong>", 1);
        }

        content = content.replace("*", "<em>").replace("_", "<em>");
        let em_count = content.matches("<em>").count();
        if em_count % 2 != 0 {
            warnings.push("Mismatched italic markers in content".to_string());
        }
        for _ in 0..(em_count / 2) {
            content = content
                .replacen("<em>", "</em>", 2)
                .replacen("</em>", "<em>", 1);
        }

        let link_re = regex::Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
        content = link_re
            .replace_all(&content, "<a href=\"$2\">$1</a>")
            .to_string();

        content = content.replace("\n\n", "\n</p>\n<p>\n");
        content = format!("<p>{}</p>", content);

        html.push_str(&content);
        html.push_str("\n</body>\n</html>");

        Ok(html)
    }

    fn format_date_range(
        &self,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
        current: bool,
    ) -> String {
        let start_str = match start {
            Some(date) => date.format(&self.options.date_format).to_string(),
            None => "".to_string(),
        };

        let end_str = if current {
            "Present".to_string()
        } else {
            match end {
                Some(date) => date.format(&self.options.date_format).to_string(),
                None => "".to_string(),
            }
        };

        if start_str.is_empty() && end_str.is_empty() {
            "".to_string()
        } else if start_str.is_empty() {
            format!("Until {}", end_str)
        } else if end_str.is_empty() {
            format!("From {}", start_str)
        } else {
            format!("{} - {}", start_str, end_str)
        }
    }
}
