use crate::components::icons::SkillChipIcon;
use crate::components::ui::primitives::{GlassCard, Pill, SectionTitle};
use crate::content::SkillCategory;
use leptos::prelude::*;

#[component]
pub fn SkillsSection(categories: &'static [SkillCategory]) -> impl IntoView {
    view! {
        <section id="skills" class="section-shell">
            <SectionTitle class="mb-10 text-center">"Skills"</SectionTitle>

            <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                {categories
                    .iter()
                    .map(|category| {
                        view! {
                            <GlassCard>
                                <h3 class="text-3xl font-bold text-slate-100">{category.title}</h3>
                                <p class="mt-4 text-lg leading-relaxed text-slate-300">
                                    {category.description}
                                </p>

                                <div class="mt-6 flex flex-wrap gap-2">
                                    {category
                                        .items
                                        .iter()
                                        .map(|item| {
                                            view! {
                                                <Pill class="skill-chip bg-slate-800/90 text-slate-100 border-slate-400/20">
                                                    <span class="skill-chip-icon" aria-hidden="true">
                                                        <SkillChipIcon icon=item.icon />
                                                    </span>
                                                    <span class="skill-chip-text">{item.name}</span>
                                                </Pill>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </GlassCard>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
