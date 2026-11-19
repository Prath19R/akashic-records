use leptos::prelude::*;
use crate::data::projects::{get_projects, Project, ProjectCategory};

#[component]
pub fn Projects() -> impl IntoView {
    let all_projects = get_projects();
    let company_projects: Vec<Project> = all_projects
        .iter()
        .filter(|p| p.category == ProjectCategory::Company)
        .cloned()
        .collect();
    let personal_projects: Vec<Project> = all_projects
        .iter()
        .filter(|p| p.category == ProjectCategory::Personal)
        .cloned()
        .collect();

    view! {
        <section id="projects" class="py-20 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto border-t border-zinc-900">
            // Section Intro
            <div class="space-y-4 mb-12">
                <div class="text-xs font-mono text-zinc-500 uppercase tracking-widest">"// 03. Selected Works"</div>
                <h2 class="text-2xl sm:text-3xl font-bold tracking-tight text-zinc-100">"Engineering Projects"</h2>
                <p class="text-sm text-zinc-400 max-w-2xl font-sans leading-relaxed">
                    "Highlighting enterprise distributed platforms engineered in R&D at NICE CXone and YASH Technologies, followed by personal open-source systems built in pure Rust and WebAssembly."
                </p>
            </div>

            // ─────────────────────────────────────────────────────────────────
            // 1. COMPANY & ENTERPRISE PROJECTS (HIGHLIGHTED)
            // ─────────────────────────────────────────────────────────────────
            <div class="mb-16">
                <div class="mb-6 flex items-center justify-between flex-wrap gap-3 pb-3 border-b border-zinc-800/80">
                    <div class="flex items-center gap-2.5">
                        <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse"></span>
                        <h3 class="text-lg sm:text-xl font-bold tracking-tight text-zinc-100 font-sans">
                            "Enterprise & Company Projects"
                        </h3>
                    </div>
                    <span class="px-3 py-1 rounded-full text-xs font-mono font-semibold bg-emerald-950/80 text-emerald-400 border border-emerald-700/60 shadow-sm flex items-center gap-1.5">
                        <span>"🏢"</span>
                        <span>"NICE CXone & YASH Technologies"</span>
                    </span>
                </div>

                <div class="space-y-6">
                    {company_projects.into_iter().map(|proj| {
                        view! {
                            <div class="relative bg-gradient-to-r from-emerald-950/30 via-zinc-900/60 to-zinc-900/40 border-l-4 border-l-emerald-500 border-t border-r border-b border-zinc-800/90 rounded-2xl p-6 sm:p-7 hover:border-emerald-500/50 hover:from-emerald-950/40 transition-all duration-200 shadow-lg shadow-emerald-950/10">
                                <div class="flex flex-col md:flex-row md:items-start justify-between gap-4 mb-3">
                                    <div>
                                        <div class="flex items-center gap-2.5 mb-1.5 flex-wrap">
                                            <h4 class="text-lg font-bold tracking-tight text-white font-sans">
                                                {proj.title}
                                            </h4>
                                            {proj.company.map(|comp| {
                                                view! {
                                                    <span class="px-2.5 py-0.5 rounded-md text-xs font-mono font-bold bg-emerald-900/60 text-emerald-300 border border-emerald-700/70 flex items-center gap-1">
                                                        <span>"🏢"</span>
                                                        <span>{comp}</span>
                                                    </span>
                                                }
                                            })}
                                            {if proj.is_featured {
                                                view! {
                                                    <span class="px-2 py-0.5 rounded text-[10px] font-mono uppercase tracking-wider bg-zinc-800 text-zinc-300 border border-zinc-700/60">
                                                        "Production"
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                        <p class="text-xs font-mono text-emerald-400/90 font-medium">
                                            {proj.tagline}
                                        </p>
                                    </div>
                                </div>

                                <p class="text-xs sm:text-sm text-zinc-300 leading-relaxed font-sans mb-4">
                                    {proj.description}
                                </p>

                                // Enterprise Metrics Badges
                                <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 mb-4">
                                    {proj.metrics.into_iter().map(|m| {
                                        view! {
                                            <div class="bg-zinc-950/80 border border-emerald-900/40 px-2.5 py-1.5 rounded-lg text-[11px] font-mono text-zinc-200 flex items-center gap-1.5 shadow-sm">
                                                <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                                                <span class="font-medium">{m}</span>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>

                                // Technology Tags
                                <div class="flex flex-wrap gap-1.5 pt-2 border-t border-zinc-800/60">
                                    {proj.tags.into_iter().map(|tag| {
                                        view! {
                                            <span class="text-[11px] font-mono text-zinc-400 bg-zinc-900/80 border border-zinc-800 px-2.5 py-0.5 rounded">
                                                {tag}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            // ─────────────────────────────────────────────────────────────────
            // 2. PERSONAL & OPEN-SOURCE SYSTEMS PROJECTS
            // ─────────────────────────────────────────────────────────────────
            <div>
                <div class="mb-6 flex items-center justify-between flex-wrap gap-3 pb-3 border-b border-zinc-800/80">
                    <div class="flex items-center gap-2.5">
                        <span class="w-2.5 h-2.5 rounded-full bg-zinc-400"></span>
                        <h3 class="text-lg sm:text-xl font-bold tracking-tight text-zinc-100 font-sans">
                            "Personal & Open-Source Systems"
                        </h3>
                    </div>
                    <span class="px-3 py-1 rounded-full text-xs font-mono text-zinc-400 bg-zinc-900 border border-zinc-800">
                        "💻 100% Rust WASM & Edge Systems"
                    </span>
                </div>

                <div class="space-y-6">
                    {personal_projects.into_iter().map(|proj| {
                        let is_athena = proj.id == "athena";
                        view! {
                            <div class="bg-zinc-900/40 border border-zinc-800/80 rounded-2xl p-6 sm:p-7 hover:border-zinc-700/80 transition-all duration-200">
                                <div class="flex flex-col md:flex-row md:items-start justify-between gap-4 mb-3">
                                    <div>
                                        <div class="flex items-center gap-2 mb-1">
                                            <h4 class="text-lg font-bold tracking-tight text-white font-sans">
                                                {proj.title}
                                            </h4>
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
                                        <div class="mt-4 pt-3 border-t border-zinc-800/60 flex items-center justify-between text-xs font-mono text-emerald-400/90">
                                            <div class="flex items-center gap-2">
                                                <span class="text-base">"💭"</span>
                                                <span>"Athena is also embedded directly in this portfolio! Click Nahida strolling along the bottom to chat."</span>
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
            </div>
        </section>
    }
}
