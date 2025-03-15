# {{name}}

## {{title}}

{{#if summary}}
{{summary}}
{{/if}}

## Contact

{{#if email}}Email: {{email}}{{/if}}
{{#if phone}}Phone: {{phone}}{{/if}}
{{#if location}}Location: {{location}}{{/if}}
{{#if linkedin}}LinkedIn: {{linkedin}}{{/if}}
{{#if github}}GitHub: {{github}}{{/if}}
{{#if website}}Website: {{website}}{{/if}}

## Experience

{{#each experiences}}

### {{title}} at {{company}}

_{{date_range}}_
{{#if location}}**Location:** {{location}}{{/if}}

{{description}}

{{#if achievements}}
**Achievements:**
{{#each achievements}}

-   {{this}}
    {{/each}}
    {{/if}}

{{#if technologies}}
**Technologies:** {{technologies}}
{{/if}}

{{/each}}

## Education

{{#each education}}

### {{degree}} in {{field_of_study}}

**{{institution}}**
_{{date_range}}_

{{#if gpa}}**GPA:** {{gpa}}{{/if}}

{{description}}

{{#if courses}}
**Relevant Courses:**
{{#each courses}}

-   {{this}}
    {{/each}}
    {{/if}}

{{/each}}

## Skills

{{#if skills.technical}}

### Technical

{{skills.technical}}
{{/if}}

{{#if skills.soft}}

### Soft Skills

{{skills.soft}}
{{/if}}

{{#if skills.tools}}

### Tools

{{skills.tools}}
{{/if}}

{{#if skills.languages}}

### Languages

{{#each skills.languages}}

-   {{name}} ({{proficiency}})
    {{/each}}
    {{/if}}

{{#if projects}}

## Projects

{{#each projects}}

### {{name}}

{{description}}

{{#if url}}**Link:** [Project Link]({{url}}){{/if}}
{{#if github}}**GitHub:** [Repository]({{github}}){{/if}}

{{#if technologies}}**Technologies:** {{technologies}}{{/if}}

{{#if highlights}}
**Highlights:**
{{#each highlights}}

-   {{this}}
    {{/each}}
    {{/if}}

{{/each}}
{{/if}}
