use leptos::prelude::*;
use leptos_ui::clx;

clx! {
    GlassCard,
    div,
    "rounded-2xl border border-slate-400/20 bg-slate-900/45 p-6 backdrop-blur-md shadow-[0_8px_40px_rgba(8,12,25,0.35)]"
}

clx! {
    Pill,
    span,
    "inline-flex items-center rounded-lg border border-sky-300/25 bg-slate-800/80 px-3 py-1 text-sm font-medium text-slate-100"
}

clx! {
    SectionTitle,
    h2,
    "text-3xl md:text-4xl font-bold tracking-tight text-slate-50"
}
