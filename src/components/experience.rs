use leptos::prelude::*;
use crate::data::experience::get_experiences;

#[component]
pub fn Experience() -> impl IntoView {
    let experiences = get_experiences();

    view! {
        <section id="experience" class="py-20 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto border-t border-zinc-900">
            <div class="space-y-4 mb-10">
                <div class="text-xs font-mono text-zinc-500 uppercase tracking-widest">"// 04. Career Track"</div>
                <h2 class="text-2xl sm:text-3xl font-bold tracking-tight text-zinc-100">"Professional Experience"</h2>
                <p class="text-sm text-zinc-400 max-w-xl font-sans">
                    "Track record in production R&D engineering, latency optimization, and distributed streaming reliability."
                </p>
            </div>

            <div class="space-y-8 relative before:absolute before:inset-0 before:left-3 sm:before:left-4 before:w-0.5 before:bg-zinc-800">
                {experiences.into_iter().map(|exp| {
                    view! {
                        <div class="relative pl-8 sm:pl-10 group">
                            // Timeline dot
                            <div class="absolute left-1.5 sm:left-2.5 top-1.5 -translate-x-1/2 w-3 h-3 rounded-full bg-zinc-800 border-2 border-zinc-500 group-hover:border-zinc-200 transition-colors"></div>

                            <div class="bg-zinc-900/40 border border-zinc-800/80 rounded-2xl p-6 hover:border-zinc-700/80 transition-colors">
                                <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-1 mb-2">
                                    <h3 class="text-base font-bold text-zinc-100 font-sans">
                                        {exp.role}
                                    </h3>
                                    <span class="text-xs font-mono text-zinc-400 bg-zinc-900 border border-zinc-800 px-2 py-0.5 rounded w-fit">
                                        {exp.period}
                                    </span>
                                </div>

                                <div class="text-xs font-mono text-zinc-400 mb-4 flex items-center gap-2">
                                    <span class="text-zinc-200 font-medium">{exp.company}</span>
                                    <span>"·"</span>
                                    <span>{exp.location}</span>
                                </div>

                                <ul class="space-y-2 mb-4 text-xs sm:text-sm text-zinc-400 font-sans leading-relaxed">
                                    {exp.description.into_iter().map(|item| {
                                        view! {
                                            <li class="flex items-start gap-2">
                                                <span class="text-zinc-600 font-mono mt-1">"›"</span>
                                                <span>{item}</span>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>

                                <div class="flex flex-wrap gap-1.5 pt-3 border-t border-zinc-800/60">
                                    {exp.tech.into_iter().map(|t| {
                                        view! {
                                            <span class="text-[11px] font-mono text-zinc-400 bg-zinc-900/80 border border-zinc-800 px-2 py-0.5 rounded">
                                                {t}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
