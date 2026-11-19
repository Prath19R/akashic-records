#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
    pub metrics: Vec<&'static str>,
    pub live_url: Option<&'static str>,
    pub github_url: Option<&'static str>,
    pub is_featured: bool,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: "athena",
            title: "Athena — Autonomous AI Portfolio Guide",
            tagline: "100% Rust WASM AI Assistant & Cloudflare Edge Gateway",
            description: "Zero-client-secret conversational guide built with Leptos 0.7+ CSR and an edge Cloudflare Worker proxy in pure Rust. Connects to NVIDIA NIM (Nemotron 3.5 30B reasoning model), enforcing strict origin controls, KV token-bucket rate limiting, and prompt-injection filtering.",
            tags: vec!["Rust", "Leptos", "WebAssembly", "Cloudflare Workers", "workers-rs", "NVIDIA NIM", "Tailwind CSS"],
            metrics: vec!["100% Rust", "Zero Client Secrets", "10 req/min KV Rate Limit", "Dual Standalone + Iframe Embed"],
            live_url: Some("https://prath19r.github.io/athena/"),
            github_url: Some("https://github.com/Prath19R/athena"),
            is_featured: true,
        },
        Project {
            id: "acdhub-envoy",
            title: "ACDHub Gateway & Envoy WASM Normalizers",
            tagline: "Ultra-Low Latency Contact Center Event Streaming (<99ms p99)",
            description: "Greenfield real-time data ingestion platform at NICE CXone. Developed custom Rust/WASM Envoy proxy plugins for Amazon Connect agent-state payloads and Salesforce TTI event ingestion, mapping streams to Universal Event Schema v1.1 and publishing to Redis Streams and AWS MSK (Kafka).",
            tags: vec!["Rust", "WebAssembly", "Envoy Proxy", "AWS MSK (Kafka)", "Redis Streams", "AWS CDK", "Go", "Java"],
            metrics: vec!["p99 < 99ms Latency (-50%)", "Historical -67% Latency", "15+ Microservices", "Zero-Loss Kafka Publish"],
            live_url: None,
            github_url: None,
            is_featured: true,
        },
    ]
}
