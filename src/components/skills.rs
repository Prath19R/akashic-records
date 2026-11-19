use leptos::prelude::*;
use crate::data::skills::get_skill_categories;

#[component]
pub fn Skills() -> impl IntoView {
    let categories = get_skill_categories();

    view! {
        <section id="skills" class="py-20 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto border-t border-zinc-900">
            <div class="space-y-4 mb-10">
                <div class="text-xs font-mono text-zinc-500 uppercase tracking-widest">"// 02. Technical Arsenal"</div>
                <h2 class="text-2xl sm:text-3xl font-bold tracking-tight text-zinc-100">"Skills & Technologies"</h2>
                <p class="text-sm text-zinc-400 max-w-xl font-sans">
                    "Battle-tested in enterprise production environments, low-latency streaming infrastructure, and high-frequency quantitative computing."
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
                {categories.into_iter().map(|cat| {
                    view! {
                        <div class="bg-zinc-900/40 border border-zinc-800/80 rounded-2xl p-5 hover:border-zinc-700/80 transition-all duration-200 group">
                            <div class="flex items-center justify-between mb-2">
                                <h3 class="text-sm font-semibold tracking-tight text-zinc-200 group-hover:text-white transition-colors">
                                    {cat.name}
                                </h3>
                                <span class="w-2 h-2 rounded-full bg-zinc-700 group-hover:bg-zinc-400 transition-colors"></span>
                            </div>
                            <p class="text-xs text-zinc-500 mb-4 font-sans">
                                {cat.description}
                            </p>
                            <div class="flex flex-wrap gap-1.5">
                                {cat.skills.into_iter().map(|skill| {
                                    view! {
                                        <span class="text-xs font-mono px-2.5 py-1 rounded-md bg-zinc-900 text-zinc-300 border border-zinc-800">
                                            {skill}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
