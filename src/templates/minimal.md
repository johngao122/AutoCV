# {{name}}

{{#if email}}{{email}} | {{/if}}{{#if phone}}{{phone}} | {{/if}}{{#if location}}{{location}}{{/if}}
{{#if linkedin}}[LinkedIn]({{linkedin}}) | {{/if}}{{#if github}}[GitHub]({{github}}) | {{/if}}{{#if website}}[Website]({{website}}){{/if}}

{{#if summary}}
{{summary}}
{{/if}}

## Experience

{{#each experiences}}
**{{title}}**, {{company}} ({{date_range}})  
{{description}}
{{#if achievements}}
{{#each achievements}}

-   {{this}}
    {{/each}}
    {{/if}}

{{/each}}

## Education

{{#each education}}
**{{degree}}** in {{field_of_study}}, {{institution}} ({{date_range}})  
{{#if gpa}}GPA: {{gpa}} {{/if}}
{{description}}

{{/each}}

## Skills

{{#if skills.technical}}**Technical:** {{skills.technical}} {{/if}}
{{#if skills.tools}}**Tools:** {{skills.tools}} {{/if}}
{{#if skills.soft}}**Soft Skills:** {{skills.soft}} {{/if}}

{{#if projects}}

## Projects

{{#each projects}}
**{{name}}** {{#if github}}([GitHub]({{github}})){{/if}} {{#if url}}([Link]({{url}})){{/if}}  
{{description}}
{{#if technologies}}Technologies: {{technologies}}{{/if}}

{{/each}}
{{/if}}
