use crate::components::icons::ProjectTagIcon;
use crate::components::ui::primitives::{GlassCard, Pill, SectionTitle};
use crate::content::ProjectEntry;
use icons::{ExternalLink, Code};
use leptos::prelude::*;

#[component]
pub fn ProjectsSection(projects: &'static [ProjectEntry]) -> impl IntoView {
    view! {
        <section id="projects" class="section-shell pb-24">
            <SectionTitle class="mb-10">"Projects"</SectionTitle>

            <div class="grid grid-cols-1 gap-6 lg:grid-cols-3 md:grid-cols-2">
                {projects
                    .iter()
                    .map(|project| {
                        let preview_link = project.github_url.or(project.live_url);
                        view! {
                            <GlassCard class="flex h-full flex-col overflow-hidden p-0">
                                {preview_link
                                    .map(|url| {
                                        view! {
                                            <a href=url target="_blank" rel="noreferrer">
                                                <img
                                                    class="h-48 w-full border-b border-slate-500/25 object-cover"
                                                    src=project.image_path
                                                    alt=format!("{} preview", project.title)
                                                />
                                            </a>
                                        }
                                            .into_any()
                                    })
                                    .unwrap_or_else(|| {
                                        view! {
                                            <img
                                                class="h-48 w-full border-b border-slate-500/25 object-cover"
                                                src=project.image_path
                                                alt=format!("{} preview", project.title)
                                            />
                                        }
                                            .into_any()
                                    })}

                                <div class="flex h-full flex-col p-5">
                                    <div class="mb-3 flex items-start justify-between gap-3">
                                        <h3 class="text-2xl font-bold text-slate-100">
                                            {project.title}
                                        </h3>
                                        <div class="flex items-center gap-2 text-sm">
                                            {project
                                                .github_url
                                                .map(|url| {
                                                    view! {
                                                        <a
                                                            class="icon-link"
                                                            href=url
                                                            target="_blank"
                                                            rel="noreferrer"
                                                            aria-label={format!("{} GitHub", project.title)}
                                                        >
                                                            <Code class="h-4 w-4" />
                                                        </a>
                                                    }
                                                })}
                                            {project
                                                .live_url
                                                .map(|url| {
                                                    view! {
                                                        <a
                                                            class="icon-link"
                                                            href=url
                                                            target="_blank"
                                                            rel="noreferrer"
                                                            aria-label={format!("{} Live Demo", project.title)}
                                                        >
                                                            <ExternalLink class="h-4 w-4" />
                                                        </a>
                                                    }
                                                })}
                                        </div>
                                    </div>

                                    <p class="text-base leading-relaxed text-slate-300">
                                        {project.description}
                                    </p>

                                    <div class="mt-5 flex flex-wrap gap-2">
                                        {project
                                            .tags
                                            .iter()
                                            .map(|tag| {
                                                view! {
                                                    <Pill class="skill-chip">
                                                        <span class="skill-chip-icon" aria-hidden="true">
                                                            <ProjectTagIcon tag=*tag />
                                                        </span>
                                                        <span class="skill-chip-text">{tag.to_string()}</span>
                                                    </Pill>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </div>
                            </GlassCard>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
