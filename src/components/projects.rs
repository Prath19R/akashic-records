use leptos::prelude::*;
use crate::data::projects::get_projects;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = get_projects();

    view! {
        <section id="projects" class="py-20 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto border-t border-zinc-900">
            <div class="space-y-4 mb-10">
                <div class="text-xs font-mono text-zinc-500 uppercase tracking-widest">"// 03. Selected Works"</div>
                <h2 class="text-2xl sm:text-3xl font-bold tracking-tight text-zinc-100">"Featured Engineering Projects"</h2>
                <p class="text-sm text-zinc-400 max-w-xl font-sans">
                    "From Rust WASM AI edge gateways to ultra-low latency Envoy proxies, WFM forecasting engines, and enterprise scheduling platforms."
                </p>
            </div>

            <div class="space-y-6">
                {projects.into_iter().map(|proj| {
                    let is_athena = proj.id == "athena";
                    view! {
                        <div class="bg-zinc-900/40 border border-zinc-800/80 rounded-2xl p-6 sm:p-7 hover:border-zinc-700/80 transition-all duration-200">
                            <div class="flex flex-col md:flex-row md:items-start justify-between gap-4 mb-3">
                                <div>
                                    <div class="flex items-center gap-2 mb-1">
                                        <h3 class="text-lg font-bold tracking-tight text-white font-sans">
                                            {proj.title}
                                        </h3>
                                        {if proj.is_featured {
                                            view! {
                                                <span class="px-2 py-0.5 rounded text-[10px] font-mono uppercase tracking-wider bg-zinc-800 text-zinc-300 border border-zinc-700/60">
                                                    "Featured"
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                    <p class="text-xs font-mono text-zinc-400">
                                        {proj.tagline}
                                    </p>
                                </div>

                                <div class="flex items-center gap-2 shrink-0">
                                    {proj.github_url.map(|url| {
                                        view! {
                                            <a
                                                href=url
                                                target="_blank"
                                                rel="noreferrer"
                                                class="text-xs font-mono px-3 py-1.5 rounded-lg border border-zinc-800 bg-zinc-900 text-zinc-300 hover:text-white hover:border-zinc-700 transition-colors flex items-center gap-1.5"
                                            >
                                                <span>"GitHub"</span>
                                                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                                    <polyline points="15 3 21 3 21 9"></polyline>
                                                    <line x1="10" y1="14" x2="21" y2="3"></line>
                                                </svg>
                                            </a>
                                        }
                                    })}
                                    {proj.live_url.map(|url| {
                                        view! {
                                            <a
                                                href=url
                                                target="_blank"
                                                rel="noreferrer"
                                                class="text-xs font-mono px-3 py-1.5 rounded-lg border border-zinc-700 bg-zinc-100 text-zinc-950 font-semibold hover:bg-white transition-all flex items-center gap-1.5 shadow-sm"
                                            >
                                                <span>"Live Demo"</span>
                                                <span class="text-xs">"↗"</span>
                                            </a>
                                        }
                                    })}
                                </div>
                            </div>

                            <p class="text-xs sm:text-sm text-zinc-400 leading-relaxed font-sans mb-4">
                                {proj.description}
                            </p>

                            // Metrics Badges
                            <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 mb-4">
                                {proj.metrics.into_iter().map(|m| {
                                    view! {
                                        <div class="bg-zinc-950/70 border border-zinc-800/80 px-2.5 py-1.5 rounded-lg text-[11px] font-mono text-zinc-300 flex items-center gap-1.5">
                                            <span class="w-1 h-1 rounded-full bg-zinc-400"></span>
                                            <span>{m}</span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>

                            // Technology Tags
                            <div class="flex flex-wrap gap-1.5 pt-2 border-t border-zinc-800/60">
                                {proj.tags.into_iter().map(|tag| {
                                    view! {
                                        <span class="text-[11px] font-mono text-zinc-500 bg-zinc-900/60 border border-zinc-800/50 px-2 py-0.5 rounded">
                                            {tag}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>

                            // Extra interactive note for Athena
                            {if is_athena {
                                view! {
                                    <div class="mt-4 pt-3 border-t border-zinc-800/60 flex items-center justify-between text-xs font-mono text-zinc-400">
                                        <div class="flex items-center gap-2">
                                            <span class="text-base">"💬"</span>
                                            <span>"Athena is also embedded directly in this portfolio! Click the floating widget in the bottom-right corner to start chatting."</span>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
