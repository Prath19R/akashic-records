#[derive(Clone, Debug, PartialEq)]
pub struct SkillCategory {
    pub name: &'static str,
    pub description: &'static str,
    pub skills: Vec<&'static str>,
}

pub fn get_skill_categories() -> Vec<SkillCategory> {
    vec![
        SkillCategory {
            name: "Systems & Backend Languages",
            description: "High-concurrency, memory-efficient, and distributed service development",
            skills: vec!["Rust", "Java (Spring Boot 3)", "Go", "Python", "C", "SQL", "TypeScript"],
        },
        SkillCategory {
            name: "Distributed Systems & Streaming",
            description: "Low-latency message streaming, proxies, and in-memory caches",
            skills: vec![
                "Envoy Proxy (WASM Plugins)",
                "Apache Kafka (Amazon MSK)",
                "Redis Cluster & Streams",
                "Valkey",
                "HTTP/2 & TLS",
                "WebAssembly (WASM)",
            ],
        },
        SkillCategory {
            name: "Cloud & DevOps Infrastructure",
            description: "Scalable cloud architecture, infrastructure as code, and edge computing",
            skills: vec![
                "AWS (ECS, MSK, RDS, Lambda, CDK)",
                "Cloudflare Workers & KV",
                "Docker & Compose",
                "GitHub Actions (CI/CD)",
                "CloudWatch & AWS X-Ray",
            ],
        },
        SkillCategory {
            name: "Architecture, Testing & Quality",
            description: "Enterprise compliance, performance benchmarking, and resilience testing",
            skills: vec![
                "Performance Engineering (Gatling)",
                "Playwright & Cucumber E2E",
                "SonarQube, Veracode & Black Duck",
                "JSON Schema & UES v1.1",
                "JUnit & Test Automation",
            ],
        },
    ]
}
