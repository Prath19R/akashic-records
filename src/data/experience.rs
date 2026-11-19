#[derive(Clone, Debug, PartialEq)]
pub struct ExperienceItem {
    pub role: &'static str,
    pub company: &'static str,
    pub period: &'static str,
    pub location: &'static str,
    pub description: Vec<&'static str>,
    pub tech: Vec<&'static str>,
}

pub fn get_experiences() -> Vec<ExperienceItem> {
    vec![
        ExperienceItem {
            role: "Associate Software Engineer — R&D",
            company: "NICE CXone (ACDHub Gateway & WFM Forecaster)",
            period: "Jul 2024 – Present",
            location: "Pune, India",
            description: vec![
                "Cut real-time contact-center data latency by 50%+ (p99 under 99 ms) and historical latency from 30s to 10s on ACDHub gateway.",
                "Engineered Rust/WASM Envoy Normalizer plugins for real-time Amazon Connect agent states and Salesforce TTI events publishing to Redis Streams and Kafka.",
                "Delivered AWS CDK infrastructure provisioning Amazon MSK clusters, dynamic topics, SSM parameters, and IAM policies across 3 repositories.",
                "Led Redis Cluster sharding & migration epic, Valkey cluster support, and automated scaling policies across microservices.",
                "Owned Cisco UCCE historical integration Lambda with Cisco file parsing and Redis-based deduplication delivered end-to-end in a single sprint.",
                "Directed security compliance (Veracode & Black Duck) and performance tooling (Gatling simulations, load generators) across 15+ microservices.",
            ],
            tech: vec!["Rust", "WASM", "Envoy", "Java", "Spring Boot 3", "AWS (MSK, ECS, CDK)", "Redis", "Kafka", "Go"],
        },
        ExperienceItem {
            role: "Trainee Programmer & Development Intern",
            company: "YASH Technologies",
            period: "Sep 2023 – Jul 2024",
            location: "Indore, India",
            description: vec![
                "Built Yash Training Management System module (authentication, task tracking, calendar scheduling) with Java, Spring Boot, Angular, and React.",
                "Contributed to task management, user authentication, and reporting across internal enterprise services.",
            ],
            tech: vec!["Java", "Spring Boot", "Angular", "React/Redux", "SQL", "JUnit"],
        },
    ]
}
