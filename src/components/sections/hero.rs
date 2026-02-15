use crate::components::icons::{SocialIcon, SocialIconKind};
use crate::content::ProfileData;
use leptos::prelude::*;

#[component]
pub fn HeroSection(profile: &'static ProfileData) -> impl IntoView {
    view! {
        <section id="about" class="section-shell pt-20 md:pt-28">
            <h1 class="max-w-3xl text-4xl font-black leading-tight text-slate-50 md:text-6xl">
                {profile.title} <span class="mt-2 block text-sky-300/95">{profile.description}</span>
            </h1>

            <p class="mt-6 max-w-2xl text-lg leading-relaxed text-slate-300">{profile.summary}</p>

            <div class="mt-8 flex flex-wrap gap-3 text-sm">
                <a class="icon-action" href=format!("mailto:{}", profile.email) aria-label="Email">
                    <SocialIcon kind=SocialIconKind::Mail class="h-5 w-5" />
                    <span class="sr-only">"Email"</span>
                </a>
                <a
                    class="icon-action"
                    href=profile.github
                    target="_blank"
                    rel="noreferrer"
                    aria-label="GitHub"
                >
                    <SocialIcon kind=SocialIconKind::Github class="h-5 w-5" />
                    <span class="sr-only">"GitHub"</span>
                </a>
                <a
                    class="icon-action"
                    href=profile.linkedin
                    target="_blank"
                    rel="noreferrer"
                    aria-label="LinkedIn"
                >
                    <SocialIcon kind=SocialIconKind::Linkedin class="h-5 w-5" />
                    <span class="sr-only">"LinkedIn"</span>
                </a>
                <span class="location-chip">
                    <SocialIcon kind=SocialIconKind::MapPin class="h-4 w-4" />
                    <span>"India"</span>
                </span>
            </div>
        </section>
    }
}
