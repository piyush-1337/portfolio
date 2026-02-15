#[derive(Clone, Copy)]
pub struct ProfileData {
    pub title: &'static str,
    pub description: &'static str,
    pub summary: &'static str,
    pub email: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TimelineKind {
    Experience,
    Education,
}

#[derive(Clone, Copy)]
pub struct TimelineEntry {
    pub kind: TimelineKind,
    pub organization: &'static str,
    pub role: &'static str,
    pub date_range: &'static str,
    pub summary: &'static str,
    pub stack: &'static [&'static str],
}

#[derive(Clone, Copy)]
pub enum SkillIconSpec {
    Rust,
    TypeScript,
    NodeJs,
    PostgreSql,
    Redis,
    GraphQl,
    Kafka,
    Git,
    Leptos,
    React,
    NextJs,
    Tailwind,
    Figma,
    Playwright,
    Aws,
    Docker,
    Kubernetes,
    Terraform,
    Cloudflare,
    Linux,
    GithubActions,
    Blockchain,
    P2pNetworking,
    BitTorrent,
    Consensus,
    SystemsProgramming,
}

#[derive(Clone, Copy)]
pub struct SkillItem {
    pub name: &'static str,
    pub icon: SkillIconSpec,
}

#[derive(Clone, Copy)]
pub struct SkillCategory {
    pub title: &'static str,
    pub description: &'static str,
    pub items: &'static [SkillItem],
}

#[derive(Clone, Copy)]
pub struct ProjectEntry {
    pub title: &'static str,
    pub description: &'static str,
    pub image_path: &'static str,
    pub tags: &'static [&'static str],
    pub github_url: Option<&'static str>,
    pub live_url: Option<&'static str>,
}

pub const PROFILE: ProfileData = ProfileData {
    title: "Hi, I'm Piyush",
    description: "Full Stack Developer",
    summary: "I build robust systems focused on networking, distributed architecture, and low-level performance with Rust and modern backend tooling.",
    email: "piyushkatkar9421@gmail.com",
    github: "https://github.com/piyush-1337",
    linkedin: "https://www.linkedin.com/in/piyush-katkar-a59997335/",
};

pub const TIMELINE: &[TimelineEntry] = &[
    TimelineEntry {
        kind: TimelineKind::Education,
        organization: "Indian Institute of Technology Tirupati",
        role: "B.Tech in Computer Science and Engineering",
        date_range: "2024 - 2028 (Expected)",
        summary: "Pursuing an undergraduate degree with focus on systems, software engineering, and core computer science foundations.",
        stack: &["DSA", "OS", "DBMS", "CN", "TOC", "COA", "OOP"],
    },
];

pub const SKILL_CATEGORIES: &[SkillCategory] = &[
    SkillCategory {
        title: "Backend",
        description: "I design robust APIs and data pipelines with a strong focus on correctness and observability.",
        items: &[
            SkillItem {
                name: "Rust",
                icon: SkillIconSpec::Rust,
            },
            SkillItem {
                name: "TypeScript",
                icon: SkillIconSpec::TypeScript,
            },
            SkillItem {
                name: "Node.js",
                icon: SkillIconSpec::NodeJs,
            },
            SkillItem {
                name: "PostgreSQL",
                icon: SkillIconSpec::PostgreSql,
            },
            SkillItem {
                name: "Redis",
                icon: SkillIconSpec::Redis,
            },
            SkillItem {
                name: "GraphQL",
                icon: SkillIconSpec::GraphQl,
            },
            SkillItem {
                name: "Kafka",
                icon: SkillIconSpec::Kafka,
            },
            SkillItem {
                name: "Git",
                icon: SkillIconSpec::Git,
            },
        ],
    },
    SkillCategory {
        title: "Frontend & Design",
        description: "I build polished interfaces with strong UX fundamentals, smooth interactions, and scalable component systems.",
        items: &[
            SkillItem {
                name: "Leptos",
                icon: SkillIconSpec::Leptos,
            },
            SkillItem {
                name: "React",
                icon: SkillIconSpec::React,
            },
            SkillItem {
                name: "Next.js",
                icon: SkillIconSpec::NextJs,
            },
            SkillItem {
                name: "Tailwind",
                icon: SkillIconSpec::Tailwind,
            },
            SkillItem {
                name: "Figma",
                icon: SkillIconSpec::Figma,
            },
            SkillItem {
                name: "Playwright",
                icon: SkillIconSpec::Playwright,
            },
            SkillItem {
                name: "TypeScript",
                icon: SkillIconSpec::TypeScript,
            },
        ],
    },
    SkillCategory {
        title: "Cloud & DevOps",
        description: "I ship applications with repeatable CI/CD pipelines and production-ready cloud infrastructure.",
        items: &[
            SkillItem {
                name: "AWS",
                icon: SkillIconSpec::Aws,
            },
            SkillItem {
                name: "Docker",
                icon: SkillIconSpec::Docker,
            },
            SkillItem {
                name: "Kubernetes",
                icon: SkillIconSpec::Kubernetes,
            },
            SkillItem {
                name: "Terraform",
                icon: SkillIconSpec::Terraform,
            },
            SkillItem {
                name: "Cloudflare",
                icon: SkillIconSpec::Cloudflare,
            },
            SkillItem {
                name: "Linux",
                icon: SkillIconSpec::Linux,
            },
            SkillItem {
                name: "GitHub Actions",
                icon: SkillIconSpec::GithubActions,
            },
        ],
    },
    SkillCategory {
        title: "Distributed Systems",
        description: "I am focused on systems-level engineering including blockchain, peer-to-peer protocols, and high-scale distributed architecture.",
        items: &[
            SkillItem {
                name: "Blockchain",
                icon: SkillIconSpec::Blockchain,
            },
            SkillItem {
                name: "P2P Networking",
                icon: SkillIconSpec::P2pNetworking,
            },
            SkillItem {
                name: "BitTorrent",
                icon: SkillIconSpec::BitTorrent,
            },
            SkillItem {
                name: "Consensus",
                icon: SkillIconSpec::Consensus,
            },
            SkillItem {
                name: "Systems Programming",
                icon: SkillIconSpec::SystemsProgramming,
            },
        ],
    },
];

pub const PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        title: "BitBorrower",
        description: "A minimalist BitTorrent client in Rust implementing core torrent parsing, tracker announce, and peer handshake flows from BEP 0003.",
        image_path: "https://opengraph.githubassets.com/1/piyush-1337/bit-borrower",
        tags: &["Rust", "BitTorrent", "Tokio", "Bencode", "TCP/UDP", "CLI"],
        github_url: Some("https://github.com/piyush-1337/bit-borrower"),
        live_url: None,
    },
    ProjectEntry {
        title: "rsbtc",
        description: "A simplified Bitcoin implementation in Rust with node, miner, and wallet binaries demonstrating P2P consensus, PoW mining, and Merkle roots.",
        image_path: "https://opengraph.githubassets.com/1/piyush-1337/rsbtc",
        tags: &["Rust", "Blockchain", "P2P", "Proof of Work", "Merkle Tree", "CBOR"],
        github_url: Some("https://github.com/piyush-1337/rsbtc"),
        live_url: None,
    },
    ProjectEntry {
        title: "Unisphere",
        description: "A full-stack real-time campus community platform with structured servers/channels, direct messaging, and anonymous confessions in one unified app.",
        image_path: "https://opengraph.githubassets.com/1/SoftwareEnggProjectsIITT/unisphere",
        tags: &["Next.js", "TypeScript", "Prisma", "PostgreSQL", "Socket.io"],
        github_url: Some("https://github.com/SoftwareEnggProjectsIITT/unisphere"),
        live_url: None,
    },
    ProjectEntry {
        title: "LegalEase",
        description: "A Flutter mobile app for legal and policy awareness using a RAG pipeline, with chat, bookmarks, personalized feed, and speech interfaces.",
        image_path: "https://opengraph.githubassets.com/1/SoftwareEnggProjectsIITT/AppDevProject",
        tags: &["Flutter", "FastAPI", "RAG", "Firebase", "Python", "Google Auth"],
        github_url: Some("https://github.com/SoftwareEnggProjectsIITT/AppDevProject"),
        live_url: None,
    },
    ProjectEntry {
        title: "Scribble",
        description: "A JavaFX multiplayer drawing and guessing game inspired by Skribbl.io, built with a client-server architecture and JSON-based messaging.",
        image_path: "https://opengraph.githubassets.com/1/piyush-1337/Game",
        tags: &["Java", "JavaFX", "Client-Server", "Multithreading", "Jackson", "Maven"],
        github_url: Some("https://github.com/piyush-1337/Game"),
        live_url: None,
    },
];
