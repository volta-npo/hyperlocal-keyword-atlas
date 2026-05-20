//! Rust release-gate and keyword scoring engine for Hyperlocal Keyword Atlas.
//!
//! This backend crate is for fast, deterministic checks that can later power
//! CLIs, API workers, batch validators, or WebAssembly modules.

pub const PRODUCT_SLUG: &str = "hyperlocal-keyword-atlas";
pub const PRODUCT_TITLE: &str = "Hyperlocal Keyword Atlas";
pub const DOMAIN_ROWS: &[&str] = &[
    "Neighborhoods entered",
    "Services entered",
    "Keyword candidates generated",
    "Intent labels assigned",
    "Competitors noted",
    "Difficulty confidence set",
    "Title/meta suggestions generated",
    "SEO brief exported",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseGate {
    pub label: String,
    pub status: String,
    pub evidence: String,
    pub severity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordCandidate {
    pub term: String,
    pub service: String,
    pub geography: String,
    pub intent: String,
    pub source: String,
    pub evidence: String,
    pub difficulty: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordScore {
    pub term: String,
    pub service: String,
    pub geography: String,
    pub intent: String,
    pub confidence: u8,
    pub priority: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordCluster {
    pub key: String,
    pub service: String,
    pub geography: String,
    pub keyword_count: usize,
    pub average_confidence: u8,
    pub high_priority_count: usize,
    pub recommended_asset: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentBrief {
    pub cluster_key: String,
    pub title: String,
    pub meta_description: String,
    pub primary_cta: String,
    pub proof_requirements: Vec<String>,
    pub internal_links: Vec<String>,
}

pub fn status_points(status: &str) -> u8 {
    match status {
        "approved" => 100,
        "ready" => 80,
        "in-progress" => 45,
        _ => 0,
    }
}

pub fn release_score(gates: &[ReleaseGate]) -> u8 {
    if gates.is_empty() {
        return 0;
    }
    let total: u32 = gates
        .iter()
        .map(|gate| status_points(gate.status.as_str()) as u32)
        .sum();
    (total / gates.len() as u32) as u8
}

pub fn blocks_release(gates: &[ReleaseGate]) -> bool {
    gates.iter().any(|gate| {
        gate.status == "blocked"
            || (gate.severity == "critical" && gate.status != "approved")
            || gate.evidence.trim().is_empty()
    })
}

fn sha256_hex(input: &[u8]) -> String {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let bit_len = (input.len() as u64) * 8;
    let mut message = input.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = H0;
    for chunk in message.chunks(64) {
        let mut w = [0u32; 64];
        for (index, word) in chunk.chunks(4).take(16).enumerate() {
            w[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    h.iter().map(|part| format!("{part:08x}")).collect()
}

pub fn release_fingerprint(gates: &[ReleaseGate]) -> String {
    let mut source = PRODUCT_SLUG.to_string();
    for gate in gates {
        source.push('|');
        source.push_str(gate.label.as_str());
        source.push('|');
        source.push_str(gate.status.as_str());
        source.push('|');
        source.push_str(gate.evidence.as_str());
        source.push('|');
        source.push_str(gate.severity.as_str());
    }
    sha256_hex(source.as_bytes())
}

pub fn source_confidence(source: &str, evidence: &str) -> u8 {
    let lower = format!("{} {}", source.to_lowercase(), evidence.to_lowercase());
    let mut score = 30;
    if lower.contains("search console")
        || lower.contains("business profile")
        || lower.contains("serp")
    {
        score += 25;
    }
    if lower.contains("competitor") || lower.contains("customer") || lower.contains("owner") {
        score += 15;
    }
    if evidence.starts_with("http") || evidence.len() > 20 {
        score += 20;
    }
    score.min(100)
}

pub fn score_keyword(candidate: &KeywordCandidate) -> KeywordScore {
    let mut warnings = Vec::new();
    if candidate.term.trim().is_empty() {
        warnings.push("keyword term is required".to_string());
    }
    if candidate.service.trim().is_empty() {
        warnings.push("service line is required".to_string());
    }
    if candidate.geography.trim().is_empty() {
        warnings.push("geography is required".to_string());
    }
    if ![
        "transactional",
        "commercial",
        "informational",
        "navigational",
    ]
    .contains(&candidate.intent.as_str())
    {
        warnings.push(
            "intent should be transactional, commercial, informational, or navigational"
                .to_string(),
        );
    }
    let source_score = source_confidence(candidate.source.as_str(), candidate.evidence.as_str());
    let difficulty_penalty = candidate.difficulty.min(100) / 3;
    let completeness = 100 - (warnings.len() as u8 * 20).min(80);
    let confidence =
        ((source_score as u16 + completeness as u16 + (100 - difficulty_penalty) as u16) / 3) as u8;
    let priority = if warnings.iter().any(|warning| warning.contains("required")) {
        "blocked"
    } else if confidence >= 82
        && matches!(candidate.intent.as_str(), "transactional" | "commercial")
    {
        "high"
    } else if confidence >= 65 {
        "medium"
    } else {
        "research"
    };
    KeywordScore {
        term: candidate.term.clone(),
        service: candidate.service.clone(),
        geography: candidate.geography.clone(),
        intent: candidate.intent.clone(),
        confidence,
        priority: priority.to_string(),
        warnings,
    }
}

pub fn cluster_keywords(candidates: &[KeywordCandidate]) -> Vec<KeywordCluster> {
    let mut clusters: Vec<KeywordCluster> = Vec::new();
    for candidate in candidates {
        let score = score_keyword(candidate);
        let key = format!(
            "{}::{}",
            candidate.service.to_lowercase(),
            candidate.geography.to_lowercase()
        );
        if let Some(cluster) = clusters.iter_mut().find(|item| item.key == key) {
            let total_confidence = cluster.average_confidence as usize * cluster.keyword_count
                + score.confidence as usize;
            cluster.keyword_count += 1;
            cluster.average_confidence = (total_confidence / cluster.keyword_count) as u8;
            if score.priority == "high" {
                cluster.high_priority_count += 1;
            }
        } else {
            let recommended_asset = if score.priority == "high" {
                "service-area landing page"
            } else if score.priority == "medium" {
                "FAQ or local proof section"
            } else {
                "research backlog item"
            };
            clusters.push(KeywordCluster {
                key,
                service: candidate.service.clone(),
                geography: candidate.geography.clone(),
                keyword_count: 1,
                average_confidence: score.confidence,
                high_priority_count: usize::from(score.priority == "high"),
                recommended_asset: recommended_asset.to_string(),
            });
        }
    }
    clusters.sort_by(|left, right| {
        right
            .high_priority_count
            .cmp(&left.high_priority_count)
            .then(right.average_confidence.cmp(&left.average_confidence))
            .then(left.key.cmp(&right.key))
    });
    clusters
}

pub fn build_content_brief(cluster: &KeywordCluster) -> ContentBrief {
    let title = format!(
        "{} in {} | Local service guide",
        title_case(&cluster.service),
        cluster.geography
    );
    let meta_description = format!(
        "Compare trusted {} options in {} with local proof, service details, FAQs, and clear next steps.",
        cluster.service, cluster.geography
    );
    ContentBrief {
        cluster_key: cluster.key.clone(),
        title,
        meta_description,
        primary_cta: "Request a local quote or consultation".to_string(),
        proof_requirements: vec![
            "Owner-approved service description".to_string(),
            "Local testimonial or project example".to_string(),
            "Neighborhood-specific FAQ evidence".to_string(),
        ],
        internal_links: vec![
            "Main services page".to_string(),
            "Contact or booking page".to_string(),
            "Related neighborhood page".to_string(),
        ],
    }
}

fn title_case(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn sample_gates() -> Vec<ReleaseGate> {
    DOMAIN_ROWS
        .iter()
        .map(|row| ReleaseGate {
            label: (*row).to_string(),
            status: "approved".to_string(),
            evidence: format!("Verified evidence for {row}"),
            severity: "normal".to_string(),
        })
        .collect()
}

pub fn sample_keywords() -> Vec<KeywordCandidate> {
    vec![
        KeywordCandidate {
            term: "emergency plumber riverside".to_string(),
            service: "plumbing".to_string(),
            geography: "Riverside".to_string(),
            intent: "transactional".to_string(),
            source: "SERP competitor + Google Business Profile".to_string(),
            evidence: "https://example.com/local-pack-capture".to_string(),
            difficulty: 32,
        },
        KeywordCandidate {
            term: "same day leak repair riverside".to_string(),
            service: "plumbing".to_string(),
            geography: "Riverside".to_string(),
            intent: "commercial".to_string(),
            source: "Customer call notes + SERP".to_string(),
            evidence: "Owner provided repeated call phrasing and local competitor examples"
                .to_string(),
            difficulty: 41,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_sample_is_release_ready() {
        let gates = sample_gates();
        assert_eq!(release_score(&gates), 100);
        assert!(!blocks_release(&gates));
        assert_eq!(release_fingerprint(&gates).len(), 64);
    }

    #[test]
    fn missing_evidence_blocks_release() {
        let gates = vec![ReleaseGate {
            label: "Critical gate".to_string(),
            status: "approved".to_string(),
            evidence: "".to_string(),
            severity: "critical".to_string(),
        }];
        assert!(blocks_release(&gates));
    }

    #[test]
    fn keyword_scoring_prioritizes_local_commercial_intent() {
        let score = score_keyword(&sample_keywords()[0]);
        assert_eq!(score.priority, "high");
        assert!(score.confidence >= 82);
        assert!(score.warnings.is_empty());
    }

    #[test]
    fn keyword_clusters_generate_content_briefs() {
        let clusters = cluster_keywords(&sample_keywords());
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].keyword_count, 2);
        assert!(clusters[0].average_confidence >= 80);
        let brief = build_content_brief(&clusters[0]);
        assert!(brief.title.contains("Riverside"));
        assert_eq!(brief.proof_requirements.len(), 3);
    }
}
