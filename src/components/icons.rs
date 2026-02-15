use crate::content::SkillIconSpec;
use icondata as id;
use icons::{
    Binary, Blocks, Boxes, Cable, CircuitBoard, Cloud, Cpu, FileCode, FileJson, GitBranch,
    Github, Globe, Hammer, Linkedin, Mail, MapPin, Network, Package, ScrollText, Share2,
    Sparkles, Terminal, Timer, Workflow, Wrench,
};
use leptos::prelude::*;
use leptos_icons::Icon as BrandIcon;

#[derive(Clone, Copy)]
pub enum SocialIconKind {
    Mail,
    Github,
    Linkedin,
    MapPin,
}

#[component]
pub fn SocialIcon(kind: SocialIconKind, #[prop(into, optional)] class: String) -> impl IntoView {
    match kind {
        SocialIconKind::Mail => view! { <Mail class=class /> }.into_any(),
        SocialIconKind::Github => view! { <Github class=class /> }.into_any(),
        SocialIconKind::Linkedin => view! { <Linkedin class=class /> }.into_any(),
        SocialIconKind::MapPin => view! { <MapPin class=class /> }.into_any(),
    }
}

#[component]
pub fn SkillChipIcon(icon: SkillIconSpec) -> impl IntoView {
    match icon {
        SkillIconSpec::Rust => {
            view! { <BrandIcon icon=id::SiRust width="1rem" height="1rem" style="color:#D5D4D8;" /> }
                .into_any()
        }
        SkillIconSpec::TypeScript => {
            view! { <BrandIcon icon=id::SiTypescript width="1rem" height="1rem" style="color:#3178C6;" /> }
                .into_any()
        }
        SkillIconSpec::NodeJs => {
            view! { <BrandIcon icon=id::SiNodedotjs width="1rem" height="1rem" style="color:#5FA04E;" /> }
                .into_any()
        }
        SkillIconSpec::PostgreSql => {
            view! { <BrandIcon icon=id::SiPostgresql width="1rem" height="1rem" style="color:#4169E1;" /> }
                .into_any()
        }
        SkillIconSpec::Redis => {
            view! { <BrandIcon icon=id::SiRedis width="1rem" height="1rem" style="color:#DC382D;" /> }
                .into_any()
        }
        SkillIconSpec::GraphQl => {
            view! { <BrandIcon icon=id::SiGraphql width="1rem" height="1rem" style="color:#E10098;" /> }
                .into_any()
        }
        SkillIconSpec::Kafka => {
            view! { <BrandIcon icon=id::SiApachekafka width="1rem" height="1rem" style="color:#E5E7EB;" /> }
                .into_any()
        }
        SkillIconSpec::Git => view! { <GitBranch class="h-4 w-4 text-slate-200" /> }.into_any(),
        SkillIconSpec::Leptos => view! { <Blocks class="h-4 w-4 text-sky-300" /> }.into_any(),
        SkillIconSpec::React => {
            view! { <BrandIcon icon=id::SiReact width="1rem" height="1rem" style="color:#61DAFB;" /> }
                .into_any()
        }
        SkillIconSpec::NextJs => {
            view! { <BrandIcon icon=id::SiNextdotjs width="1rem" height="1rem" style="color:#E5E7EB;" /> }
                .into_any()
        }
        SkillIconSpec::Tailwind => {
            view! { <BrandIcon icon=id::SiTailwindcss width="1rem" height="1rem" style="color:#06B6D4;" /> }
                .into_any()
        }
        SkillIconSpec::Figma => {
            view! { <BrandIcon icon=id::SiFigma width="1rem" height="1rem" style="color:#F24E1E;" /> }
                .into_any()
        }
        SkillIconSpec::Playwright => {
            view! { <Wrench class="h-4 w-4 text-lime-400" /> }.into_any()
        }
        SkillIconSpec::Aws => {
            view! { <Cloud class="h-4 w-4 text-amber-400" /> }.into_any()
        }
        SkillIconSpec::Docker => {
            view! { <BrandIcon icon=id::SiDocker width="1rem" height="1rem" style="color:#2496ED;" /> }
                .into_any()
        }
        SkillIconSpec::Kubernetes => {
            view! { <BrandIcon icon=id::SiKubernetes width="1rem" height="1rem" style="color:#326CE5;" /> }
                .into_any()
        }
        SkillIconSpec::Terraform => {
            view! { <BrandIcon icon=id::SiTerraform width="1rem" height="1rem" style="color:#7B42BC;" /> }
                .into_any()
        }
        SkillIconSpec::Cloudflare => {
            view! { <BrandIcon icon=id::SiCloudflare width="1rem" height="1rem" style="color:#F38020;" /> }
                .into_any()
        }
        SkillIconSpec::Linux => {
            view! { <BrandIcon icon=id::SiLinux width="1rem" height="1rem" style="color:#FBC02D;" /> }
                .into_any()
        }
        SkillIconSpec::GithubActions => {
            view! { <BrandIcon icon=id::SiGithubactions width="1rem" height="1rem" style="color:#2088FF;" /> }
                .into_any()
        }
        SkillIconSpec::Blockchain => view! { <Boxes class="h-4 w-4 text-cyan-300" /> }.into_any(),
        SkillIconSpec::P2pNetworking => view! { <Network class="h-4 w-4 text-cyan-300" /> }.into_any(),
        SkillIconSpec::BitTorrent => view! { <Share2 class="h-4 w-4 text-cyan-300" /> }.into_any(),
        SkillIconSpec::Consensus => view! { <Workflow class="h-4 w-4 text-cyan-300" /> }.into_any(),
        SkillIconSpec::SystemsProgramming => view! { <Cpu class="h-4 w-4 text-cyan-300" /> }.into_any(),
    }
}

#[component]
pub fn ProjectTagIcon(tag: &'static str) -> impl IntoView {
    match tag {
        "Rust" => {
            view! { <BrandIcon icon=id::SiRust width="1rem" height="1rem" style="color:#D5D4D8;" /> }
                .into_any()
        }
        "BitTorrent" => {
            view! { <BrandIcon icon=id::SiBittorrent width="1rem" height="1rem" style="color:#10B981;" /> }
                .into_any()
        }
        "Tokio" => view! { <Timer class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Bencode" => view! { <Binary class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "TCP/UDP" => view! { <Cable class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "CLI" => view! { <Terminal class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Blockchain" => {
            view! { <BrandIcon icon=id::SiBlockchaindotcom width="1rem" height="1rem" style="color:#2563EB;" /> }
                .into_any()
        }
        "P2P" => view! { <Network class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Proof of Work" => view! { <Hammer class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Merkle Tree" => view! { <Workflow class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "CBOR" => view! { <FileJson class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Next.js" => {
            view! { <BrandIcon icon=id::SiNextdotjs width="1rem" height="1rem" style="color:#E5E7EB;" /> }
                .into_any()
        }
        "TypeScript" => {
            view! { <BrandIcon icon=id::SiTypescript width="1rem" height="1rem" style="color:#3178C6;" /> }
                .into_any()
        }
        "Prisma" => {
            view! { <BrandIcon icon=id::SiPrisma width="1rem" height="1rem" style="color:#A5B4FC;" /> }
                .into_any()
        }
        "PostgreSQL" => {
            view! { <BrandIcon icon=id::SiPostgresql width="1rem" height="1rem" style="color:#4169E1;" /> }
                .into_any()
        }
        "Socket.io" => {
            view! { <BrandIcon icon=id::SiSocketdotio width="1rem" height="1rem" style="color:#E5E7EB;" /> }
                .into_any()
        }
        "Clerk" => {
            view! { <BrandIcon icon=id::SiClerk width="1rem" height="1rem" style="color:#8B5CF6;" /> }
                .into_any()
        }
        "Flutter" => {
            view! { <BrandIcon icon=id::SiFlutter width="1rem" height="1rem" style="color:#54C5F8;" /> }
                .into_any()
        }
        "FastAPI" => {
            view! { <BrandIcon icon=id::SiFastapi width="1rem" height="1rem" style="color:#059669;" /> }
                .into_any()
        }
        "RAG" => view! { <Sparkles class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Firebase" => {
            view! { <BrandIcon icon=id::SiFirebase width="1rem" height="1rem" style="color:#F59E0B;" /> }
                .into_any()
        }
        "Python" => {
            view! { <BrandIcon icon=id::SiPython width="1rem" height="1rem" style="color:#3776AB;" /> }
                .into_any()
        }
        "Google Auth" => {
            view! { <BrandIcon icon=id::SiGoogleauthenticator width="1rem" height="1rem" style="color:#4285F4;" /> }
                .into_any()
        }
        "Java" => {
            view! { <BrandIcon icon=id::SiOpenjdk width="1rem" height="1rem" style="color:#EA580C;" /> }
                .into_any()
        }
        "JavaFX" => view! { <CircuitBoard class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Client-Server" => view! { <Globe class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Multithreading" => view! { <Boxes class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Jackson" => view! { <ScrollText class="h-4 w-4 text-cyan-300" /> }.into_any(),
        "Maven" => view! { <Package class="h-4 w-4 text-cyan-300" /> }.into_any(),
        _ => view! { <FileCode class="h-4 w-4 text-cyan-300" /> }.into_any(),
    }
}
