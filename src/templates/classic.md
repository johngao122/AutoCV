# {{name}}

{{title}}

---

## CONTACT INFORMATION

-   Email: {{email}}
-   Phone: {{phone}}
    {{#if location}}- Location: {{location}}{{/if}}
    {{#if linkedin}}- LinkedIn: {{linkedin}}{{/if}}
    {{#if github}}- GitHub: {{github}}{{/if}}
    {{#if website}}- Website: {{website}}{{/if}}

---

## SUMMARY

{{summary}}

---

## PROFESSIONAL EXPERIENCE

{{#each experiences}}
**{{title}}** | {{company}} | {{date_range}}
{{#if location}}{{location}}{{/if}}

{{description}}

{{#if achievements}}
Key Achievements:
{{#each achievements}}

-   {{this}}
    {{/each}}
    {{/if}}

{{#if technologies}}
Technologies: {{technologies}}
{{/if}}

{{/each}}

---

## EDUCATION

{{#each education}}
**{{degree}} in {{field_of_study}}** | {{institution}} | {{date_range}}
{{#if gpa}}GPA: {{gpa}}{{/if}}

{{description}}

{{#if courses}}
Relevant Courses: {{courses}}
{{/if}}

{{/each}}

---

## SKILLS

{{#if skills.technical}}
**Technical:** {{skills.technical}}
{{/if}}

{{#if skills.soft}}
**Soft Skills:** {{skills.soft}}
{{/if}}

{{#if skills.tools}}
**Tools:** {{skills.tools}}
{{/if}}

{{#if skills.languages}}
**Languages:**
{{#each skills.languages}}

-   {{name}} ({{proficiency}})
    {{/each}}
    {{/if}}

---

{{#if projects}}

## PROJECTS

{{#each projects}}
**{{name}}**
{{description}}

{{#if url}}Link: {{url}}{{/if}}
{{#if github}}GitHub: {{github}}{{/if}}

{{#if technologies}}Technologies: {{technologies}}{{/if}}

{{#if highlights}}
Highlights:
{{#each highlights}}

-   {{this}}
    {{/each}}
    {{/if}}

{{/each}}
{{/if}}
