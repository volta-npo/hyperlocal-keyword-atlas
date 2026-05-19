export const domain = {
  "kind": "keyword-atlas",
  "title": "Hyperlocal Keyword Atlas",
  "purpose": "A purpose-built keyword atlas interface for a keyword research framework for neighborhoods, not national search markets.",
  "inputTitle": "Product-specific inputs",
  "previewTitle": "Generated working outputs",
  "tableTitle": "Keyword candidates",
  "metricLabels": [
    "Intent Coverage",
    "Evidence Confidence",
    "Brief Readiness"
  ],
  "fields": [
    {
      "id": "organization-client",
      "label": "Organization / client",
      "type": "text",
      "sample": "Oak & Olive Cafe",
      "placeholder": "Enter organization / client"
    },
    {
      "id": "primary-goal",
      "label": "Primary goal",
      "type": "text",
      "sample": "pages/posts targeting validated local intent",
      "placeholder": "Enter primary goal"
    },
    {
      "id": "owner-reviewer",
      "label": "Owner / reviewer",
      "type": "text",
      "sample": "Volta project lead",
      "placeholder": "Enter owner / reviewer"
    },
    {
      "id": "evidence-source",
      "label": "Evidence source",
      "type": "text",
      "sample": "Owner interview + public audit",
      "placeholder": "Enter evidence source"
    },
    {
      "id": "input-asset",
      "label": "Input asset",
      "type": "text",
      "sample": "Neighborhoods entered",
      "placeholder": "Enter input asset"
    },
    {
      "id": "output-format",
      "label": "Output format",
      "type": "text",
      "sample": "Keyword CSV",
      "placeholder": "Enter output format"
    },
    {
      "id": "review-threshold",
      "label": "Review threshold",
      "type": "number",
      "sample": 85,
      "placeholder": "Enter review threshold"
    },
    {
      "id": "approved-channel",
      "label": "Approved channel",
      "type": "text",
      "sample": "Owner handoff packet",
      "placeholder": "Enter approved channel"
    }
  ],
  "rows": [
    "Neighborhoods entered",
    "Services entered",
    "Keyword candidates generated",
    "Intent labels assigned",
    "Competitors noted",
    "Difficulty confidence set",
    "Title/meta suggestions generated",
    "SEO brief exported",
    "Service/geography/intent modeled",
    "Source confidence scored",
    "POST scoring payload validated",
    "SERP evidence attached"
  ],
  "artifacts": [
    "Keyword CSV",
    "SEO brief",
    "Title/meta pack"
  ],
  "checks": [
    "Keyword needs service/geography/intent/source",
    "No unsupported search-volume claims",
    "Difficulty confidence required"
  ],
  "sampleClient": "Oak & Olive Cafe",
  "modules": [
    { "name": "Keyword scoring API", "description": "POST-based scoring for submitted keyword sets with confidence, intent, and evidence validation." },
    { "name": "Service/geography model", "description": "Service line, city, neighborhood, modifier, language, and seasonal segment taxonomy." },
    { "name": "SERP source confidence", "description": "Source type, recency, authority, competitor fit, and local-pack evidence scoring." },
    { "name": "Content brief generator", "description": "Title/meta, FAQ, internal link, proof need, and CTA guidance for every cluster." }
  ],
  "saas": {
    "playbooks": [
      "Local keyword discovery workflow",
      "SERP evidence qualification",
      "Service-area landing page brief",
      "Source confidence audit"
    ],
    "automations": [
      "Keyword cluster scoring",
      "SERP confidence calculation",
      "Title/meta pack validation",
      "Rust API release fingerprinting"
    ],
    "revenueModel": "Agency keyword workspace with paid service-area packs, API scoring, and brief exports",
    "integrationTargets": [
      "Google Search Console export",
      "Google Business Profile categories",
      "CSV SERP tools",
      "CMS landing page briefs"
    ]
  }
};
