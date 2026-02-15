use crate::components::ui::primitives::{GlassCard, Pill, SectionTitle};
use crate::content::{TimelineEntry, TimelineKind};
use leptos::prelude::*;

#[component]
pub fn TimelineSection(entries: &'static [TimelineEntry]) -> impl IntoView {
    view! {
        <section id="experience" class="section-shell">
            <SectionTitle class="mb-10">"Education / Experience"</SectionTitle>

            <div class="relative pl-12">
                <div class="space-y-8">
                    {entries
                        .iter()
                        .enumerate()
                        .map(|(idx, entry)| {
                            let label = match entry.kind {
                                TimelineKind::Experience => "Experience",
                                TimelineKind::Education => "Education",
                            };
                            let is_last = idx + 1 == entries.len();

                            view! {
                                <article class="relative">
                                    <span class="timeline-node"></span>
                                    {(!is_last)
                                        .then(|| {
                                            view! { <span class="timeline-connector"></span> }
                                        })}
                                    <GlassCard>
                                        <div class="mb-3 flex flex-wrap items-center gap-3">
                                            <h3 class="text-xl font-bold text-sky-300">
                                                {entry.organization}
                                            </h3>
                                            <Pill class="bg-slate-900 text-xs text-slate-300 border-slate-500/30">
                                                {label}
                                            </Pill>
                                        </div>

                                        <p class="text-2xl font-semibold text-slate-100">
                                            {entry.role}
                                        </p>
                                        <p class="mt-1 text-sm text-slate-400">
                                            {entry.date_range}
                                        </p>

                                        <p class="mt-4 text-base leading-relaxed text-slate-300">
                                            {entry.summary}
                                        </p>

                                        <div class="mt-5 flex flex-wrap gap-2">
                                            {entry
                                                .stack
                                                .iter()
                                                .map(|tech| {
                                                    view! { <Pill>{tech.to_string()}</Pill> }
                                                })
                                                .collect_view()}
                                        </div>
                                    </GlassCard>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
