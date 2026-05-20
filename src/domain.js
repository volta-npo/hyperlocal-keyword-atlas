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
        { "id": "service-area-count", "label": "Service area count", "type": "number", "sample": 8, "placeholder": "Enter service area count" },
        { "id": "content-capacity", "label": "Content capacity", "type": "number", "sample": 12, "placeholder": "Enter content capacity" },
        { "id": "cms-target", "label": "CMS target", "type": "text", "sample": "WordPress service pages", "placeholder": "Enter CMS target" },
        { "id": "ranking-review-cadence", "label": "Ranking review cadence", "type": "text", "sample": "Monthly Search Console review", "placeholder": "Enter ranking review cadence" },
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
        "SERP evidence attached",
        "Keyword workspace provisioned",
        "Cluster ownership assigned",
        "Content calendar capacity matched",
        "SERP evidence archive linked",
        "Cannibalization risk reviewed",
        "Local pack opportunity scored",
        "CMS publishing queue created",
        "Monthly rank review scheduled",
    ],
    "artifacts": [
        "Keyword CSV",
        "SEO brief",
        "Title/meta pack",
        "Keyword opportunity dashboard",
        "Content calendar backlog",
        "SERP evidence archive",
    ],
    "checks": [
        "Keyword needs service/geography/intent/source",
        "No unsupported search-volume claims",
        "Difficulty confidence required",
        "No search-volume claim without cited source",
        "Landing page briefs require unique service/geography pair",
        "Cannibalization warning must be reviewed before export",
    ],
    "sampleClient": "Oak & Olive Cafe",
    "modules": [
        { "name": "Opportunity pipeline", "description": "Ranks clusters by confidence, intent, difficulty, service priority, content capacity, and local proof strength." },
        { "name": "Cannibalization guard", "description": "Detects duplicate service/geography targets, overlapping modifiers, and competing page briefs before publishing." },
        { "name": "SERP evidence vault", "description": "Stores source type, capture date, competitor, local pack status, proof URL, and confidence notes." },
        { "name": "Content calendar queue", "description": "Turns approved clusters into publishable page, FAQ, post, and GBP update briefs with owners and dates." },
        { "name": "Rank review monitor", "description": "Monthly check-in template for impressions, clicks, ranking changes, and content refresh decisions." },
        { "name": "API scoring console", "description": "Backend-grade scoring output for batch keyword validation, evidence warnings, and confidence distribution." }
    ],
    "saas": {
        "customerSegments": [
            "Student SEO pods researching neighborhood demand",
            "Local service businesses planning service-area pages",
            "Nonprofit chapters publishing local resource pages",
            "Mentors validating evidence-backed SEO briefs"
        ],
        "pricingTiers": [
            "Free: single keyword atlas export",
            "Local Pro: cluster scoring, SERP vault, and page briefs",
            "Agency: multi-location dashboards, API scoring, and CMS queues",
            "Portfolio: chapter-wide opportunity benchmarks and bulk exports"
        ],
        "onboardingChecklist": [
            "Create local SEO workspace",
            "Import service and geography taxonomy",
            "Attach SERP and customer evidence sources",
            "Assign content owners and review cadence",
            "Generate first service-area brief backlog"
        ],
        "successMetrics": [
            "Every high-priority keyword has service/geography/intent/source",
            "Cannibalization risk reviewed for all publishable clusters",
            "At least one brief queued per priority service",
            "Monthly rank review cadence scheduled"
        ],
        "dashboards": [
            "Keyword opportunity pipeline",
            "SERP source confidence",
            "Content backlog by capacity",
            "Rank review and refresh queue"
        ],
        "dataModel": [
            "KeywordWorkspace",
            "ServiceArea",
            "KeywordCandidate",
            "SourceEvidence",
            "KeywordCluster",
            "ContentBrief",
            "CannibalizationWarning",
            "RankReview"
        ],
        "permissions": [
            "SEO lead: taxonomy and scoring settings",
            "Researcher: candidate and evidence entry",
            "Content editor: brief ownership and publishing status",
            "Mentor: release certification and source review"
        ],
        "compliance": [
            "Unsupported volume claims blocked",
            "Evidence source retained for confidence score",
            "Medical/legal SEO claims require owner approval",
            "Client-safe briefs exclude private competitor notes"
        ],
        "lifecycle": [
            "Discover",
            "Score",
            "Cluster",
            "Brief",
            "Publish",
            "Measure",
            "Refresh"
        ],
        "retentionSignals": [
            "Rank review overdue",
            "Approved cluster not published",
            "Low confidence source needs replacement",
            "Content refresh opportunity detected"
        ],
        "exportChannels": [
            "Keyword CSV",
            "SEO brief markdown",
            "Title/meta pack",
            "SERP evidence archive",
            "Content calendar backlog",
            "Rust scoring JSON"
        ],
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
//# sourceMappingURL=domain.js.map