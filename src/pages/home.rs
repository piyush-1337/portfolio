use crate::components::sections::hero::HeroSection;
use crate::components::sections::projects::ProjectsSection;
use crate::components::sections::skills::SkillsSection;
use crate::components::sections::timeline::TimelineSection;
use crate::components::sections::top_nav::TopNav;
use crate::content::{PROFILE, PROJECTS, SKILL_CATEGORIES, TIMELINE};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="portfolio-bg min-h-screen text-slate-100">
            // <TopNav />
            <main class="mx-auto w-full max-w-6xl px-5 md:px-8">
                <HeroSection profile=&PROFILE />
                <TimelineSection entries=TIMELINE />
                <SkillsSection categories=SKILL_CATEGORIES />
                <ProjectsSection projects=PROJECTS />
            </main>
            <footer class="border-t border-slate-500/20 py-8 text-center text-sm text-slate-400">
                "Built with Leptos + Rust UI :)"
            </footer>
        </div>
    }
}
