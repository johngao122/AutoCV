# AutoCV - Automated Resume Creator & ATS Optimizer

![Rust CI](https://github.com/johngao122/AutoCV/actions/workflows/ci.yml/badge.svg)

AutoCV is a Rust-based tool that automates resume creation and optimization for Applicant Tracking Systems (ATS). It can process LinkedIn data and generate professionally formatted, ATS-friendly resumes in multiple formats.

## Features

-   LinkedIn data extraction and parsing
-   ATS-optimized resume generation
-   Multiple output formats (PDF, Markdown, Plain Text)
-   Keyword optimization for better ATS matching
-   Professional template system

## Project Status

### In Progress

-   LinkedIn data extraction system
    -   Manual data scraping implementation
    -   Browser automation framework
    -   LinkedIn resume export parser
-   Core resume data processing
    -   JSON/YAML data serialization
    -   Markdown to HTML conversion
    -   Template system integration

### Planned

-   ATS-optimized PDF generation
    -   Professional template designs
    -   Font optimization
    -   Layout customization
-   Text processing & ATS optimization
    -   Keyword analysis
    -   Format compatibility checks
    -   Content optimization
-   User interface development
    -   Command-line interface
    -   (Future) Web-based interface

## Technical Stack

### Data Collection & Parsing

-   `scraper` - LinkedIn profile parsing
-   `reqwest` - HTTP request handling
-   `serde_json` - JSON data parsing

### Data Processing

-   `serde` - Data serialization
-   `pulldown-cmark` - Markdown processing
-   `tera` - Template engine

### PDF Generation

-   `printpdf` - PDF creation
-   `typst` - Modern document formatting

### Text Processing

-   `textwrap` - Text formatting
-   `rust-stemmers` - Keyword optimization
-   `regex` - Data extraction

### User Interface

-   `clap` - Command-line interface

## Getting Started

[Coming Soon]

## Contributing

[Coming Soon]

## License

[Coming Soon]
