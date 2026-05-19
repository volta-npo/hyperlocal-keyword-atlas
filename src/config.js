export const config = {
    "number": 28,
    "slug": "hyperlocal-keyword-atlas",
    "title": "Hyperlocal Keyword Atlas",
    "category": "Marketing & Content",
    "tagline": "A keyword research framework for neighborhoods, not national search markets.",
    "persona": "SEO students improving discoverability.",
    "gap": "Keyword tools are built for scale and paid subscriptions. Local businesses need nearby intent.",
    "niche": "Local SEO strategy for small service businesses.",
    "metric": "pages/posts targeting validated local intent",
    "modules": [
        "Neighborhood term map",
        "Service-intent matrix",
        "Competitor SERP notes",
        "Content brief generator"
    ],
    "theme": {
        "accent": "#db2777",
        "accent2": "#f9a8d4",
        "emoji": "\ud83d\udce3",
        "metricLabel": "Content readiness",
        "workflow": [
            "Capture owner voice",
            "Generate channel-ready assets",
            "Review for local fit",
            "Export approved content"
        ],
        "privacy": "Do not publish quotes, photos, or testimonials without explicit owner/client approval."
    },
    "statuses": [
        "not-started",
        "blocked",
        "in-progress",
        "ready",
        "approved"
    ],
    "criteria": [
        {
            "id": "neighborhood-term-map",
            "label": "Neighborhood term map",
            "weight": 15,
            "defaultStatus": "not-started",
            "prompt": "Implement and verify neighborhood term map with evidence that a Volta student pod, mentor, and owner can understand."
        },
        {
            "id": "service-intent-matrix",
            "label": "Service-intent matrix",
            "weight": 15,
            "defaultStatus": "not-started",
            "prompt": "Implement and verify service-intent matrix with evidence that a Volta student pod, mentor, and owner can understand."
        },
        {
            "id": "competitor-serp-notes",
            "label": "Competitor SERP notes",
            "weight": 15,
            "defaultStatus": "not-started",
            "prompt": "Implement and verify competitor serp notes with evidence that a Volta student pod, mentor, and owner can understand."
        },
        {
            "id": "content-brief-generator",
            "label": "Content brief generator",
            "weight": 15,
            "defaultStatus": "not-started",
            "prompt": "Implement and verify content brief generator with evidence that a Volta student pod, mentor, and owner can understand."
        },
        {
            "id": "evidence-quality",
            "label": "Evidence quality",
            "weight": 10,
            "defaultStatus": "not-started",
            "prompt": "Attach proof, source notes, screenshots, owner confirmation, or reviewer rationale."
        },
        {
            "id": "owner-handoff",
            "label": "Owner handoff",
            "weight": 10,
            "defaultStatus": "not-started",
            "prompt": "Make the output understandable and maintainable by a nontechnical owner."
        },
        {
            "id": "mission-alignment",
            "label": "Mission alignment",
            "weight": 10,
            "defaultStatus": "not-started",
            "prompt": "Show how this advances digital equity, student growth, or pro bono delivery."
        },
        {
            "id": "qa-safety",
            "label": "QA and safety",
            "weight": 10,
            "defaultStatus": "not-started",
            "prompt": "Resolve privacy, accessibility, accuracy, and operational risks before handoff."
        }
    ],
    "templates": {
        "actions": [
            "Run a real Volta scenario for Hyperlocal Keyword Atlas and capture baseline evidence.",
            "Complete the neighborhood term map workflow with owner-safe notes.",
            "Resolve all blocked rubric items and add evidence for every ready item.",
            "Export the handoff packet and review it with a mentor before client use."
        ]
    },
    "sample": {
        "clientName": "Oak & Olive Cafe",
        "chapter": "Dallas",
        "studentLead": "Volta Student Lead",
        "notes": "Neighborhood marketing project with owner-approved content assets. Hyperlocal Keyword Atlas sample.",
        "evidencePrefix": "Hyperlocal Keyword Atlas",
        "evidence": [
            "Discovery call notes captured with owner confirmation.",
            "Public digital footprint reviewed and summarized.",
            "Mentor QA comments attached before handoff."
        ]
    }
};
//# sourceMappingURL=config.js.map