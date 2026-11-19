#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectCategory {
    Company,
    Personal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub company: Option<&'static str>,
    pub category: ProjectCategory,
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
        // ─── COMPANY PROJECTS (NICE CXone & YASH Technologies) ─────────────────────
        Project {
            id: "acdhub-envoy",
            title: "ACDHub Gateway & Envoy WASM Normalizers",
            company: Some("NICE CXone · Enterprise R&D"),
            category: ProjectCategory::Company,
            tagline: "Ultra-Low Latency Contact Center Event Streaming (<99ms p99)",
            description: "Greenfield real-time data ingestion platform at NICE CXone. Developed custom Rust/WASM Envoy proxy plugins for Amazon Connect agent-state payloads and Salesforce TTI event ingestion, mapping streams to Universal Event Schema v1.1 and publishing to Redis Streams and AWS MSK (Kafka).",
            tags: vec!["Rust", "WebAssembly", "Envoy Proxy", "AWS MSK (Kafka)", "Redis Streams", "AWS CDK", "Go", "Java"],
            metrics: vec!["p99 < 99ms Latency (-50%)", "Historical -67% Latency", "15+ Microservices", "Zero-Loss Kafka Publish"],
            live_url: None,
            github_url: None,
            is_featured: true,
        },
        Project {
            id: "wfm-forecaster",
            title: "WFM Forecaster — Platform Modernization & Performance",
            company: Some("NICE CXone"),
            category: ProjectCategory::Company,
            tagline: "High-Throughput Contact Center Staffing Forecasts & Spring Boot 3 Migration",
            description: "Core platform engineering for NICE CXone's WFM Forecaster platform spanning 15+ microservices. Optimized long-running batch cleanup jobs from 2 hours to under 30 minutes (-75%), resolved critical forecasting and staffing calculation anomalies, built automated Gatling load simulations, and led Black Duck & Veracode compliance across 2 enterprise release cycles.",
            tags: vec!["Java", "Spring Boot 3", "AWS Batch", "ECS", "RDS", "Gatling", "Docker", "Hibernate", "CloudWatch"],
            metrics: vec!["15+ Microservices", "Job Time: 2h → 30m (-75%)", "Zero Critical CVEs", "Gatling Load Simulator"],
            live_url: None,
            github_url: None,
            is_featured: true,
        },
        Project {
            id: "yash-tms",
            title: "Yash Training Management System (YTMS)",
            company: Some("YASH Technologies"),
            category: ProjectCategory::Company,
            tagline: "Enterprise Training Platform with Dynamic Calendar Scheduling & Gemini AI",
            description: "Internal enterprise training and task orchestration platform at YASH Technologies. Built secure JWT/session-based role authentication, dynamic calendar event schedulers, trainee progress dashboards, and integrated Gemini Generative AI into Angular frontends for automated learning workflows.",
            tags: vec!["Java 8", "Spring Boot", "Angular", "TypeScript", "MySQL", "Spring Security", "Gemini AI", "React"],
            metrics: vec!["Calendar Scheduler Module", "Role-Based Auth", "Gemini AI Integration", "Multi-Track Management"],
            live_url: None,
            github_url: None,
            is_featured: true,
        },
        Project {
            id: "task-management",
            title: "Trainee Task Management & Ingestion Platform",
            company: Some("YASH Technologies"),
            category: ProjectCategory::Company,
            tagline: "Full-Stack Task Assignment, Lifecycle Tracking & Enterprise Servlets",
            description: "Interactive workflow tracking system built during tenure at YASH Technologies. Implemented end-to-end task assignment pipelines, status state machines, role-based access control, and responsive web UIs using Java Servlets, JSP, and MySQL.",
            tags: vec!["Core Java", "Java 8", "Servlets", "JSP", "MySQL", "JavaScript", "HTML/CSS"],
            metrics: vec!["State Machine Task Flow", "RBAC Authentication", "Responsive UI", "Relational Schema Design"],
            live_url: None,
            github_url: None,
            is_featured: false,
        },

        // ─── PERSONAL & OPEN-SOURCE SYSTEMS PROJECTS ──────────────────────────────
        Project {
            id: "athena",
            title: "Athena — Autonomous AI Portfolio Guide",
            company: None,
            category: ProjectCategory::Personal,
            tagline: "100% Rust WASM AI Assistant & Cloudflare Edge Gateway",
            description: "Zero-client-secret conversational guide built with Leptos 0.7+ CSR and an edge Cloudflare Worker proxy in pure Rust. Connects to NVIDIA NIM (Nemotron 3.5 30B reasoning model), enforcing strict origin controls, KV token-bucket rate limiting (10 req/min/IP), and prompt-injection filtering. Embedded as an interactive thought bubble companion in Akashic Records.",
            tags: vec!["Rust", "Leptos", "WebAssembly", "Cloudflare Workers", "workers-rs", "NVIDIA NIM", "Tailwind CSS"],
            metrics: vec!["100% Rust", "Zero Client Secrets", "10 req/min KV Rate Limit", "Dual Standalone + Iframe Embed"],
            live_url: Some("https://prath19r.github.io/athena/"),
            github_url: Some("https://github.com/Prath19R/athena"),
            is_featured: true,
        },
        Project {
            id: "akashic-records",
            title: "Akashic Records — Personal Systems Engineering Portfolio",
            company: None,
            category: ProjectCategory::Personal,
            tagline: "Rust Leptos 0.7+ CSR WASM Personal Engineering Showcase",
            description: "High-performance personal systems engineering portfolio built with Leptos 0.7+ in client-side WebAssembly. Features an interactive pure Rust state-machine chibi companion with real-time thought bubble chat integration and automated GitHub Pages deployment.",
            tags: vec!["Rust", "Leptos 0.7+", "WebAssembly", "Trunk", "Tailwind CSS"],
            metrics: vec!["Pure Rust WASM", "Zero Node Runtime", "Dark/Zinc Minimalist UI", "Interactive State Machine"],
            live_url: Some("https://prath19r.github.io/akashic-records/"),
            github_url: Some("https://github.com/Prath19R/akashic-records"),
            is_featured: false,
        },
    ]
}
