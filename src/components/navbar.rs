use leptos::prelude::*;

struct NavTab {
    label: &'static str,
    href: &'static str,
}

const TABS: &[NavTab] = &[
    NavTab { label: "about", href: "#about" },
    NavTab { label: "skills", href: "#skills" },
    NavTab { label: "projects", href: "#projects" },
    NavTab { label: "experience", href: "#experience" },
    NavTab { label: "contact", href: "#contact" },
];

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-40 w-full border-b border-zinc-800/80 bg-[#09090b]/85 backdrop-blur-md transition-all">
            <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between gap-4">
                // Logo & Identity
                <a href="#hero" class="flex items-center gap-2.5 group shrink-0">
                    <span class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 flex items-center justify-center font-mono text-sm font-semibold text-zinc-100 group-hover:border-emerald-500/50 group-hover:text-emerald-400 transition-all shadow-inner">
                        "PR"
                    </span>
                    <div class="flex flex-col">
                        <span class="text-sm font-semibold tracking-tight text-zinc-100 group-hover:text-white transition-colors">
                            "Prathmesh Ranadive"
                        </span>
                        <span class="text-[10px] font-mono text-zinc-500 uppercase tracking-widest flex items-center gap-1">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                            "Systems Engineer"
                        </span>
                    </div>
                </a>

                // Desktop Segmented Pill Tabs
                <nav class="hidden md:flex items-center gap-1 bg-zinc-900/80 border border-zinc-800 rounded-full p-1 backdrop-blur-md shadow-inner">
                    {TABS.iter().map(|tab| {
                        view! {
                            <a
                                href=tab.href
                                class="px-3.5 py-1.5 rounded-full text-xs font-mono font-medium text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800/90 hover:shadow-sm transition-all duration-150 flex items-center gap-1 group"
                            >
                                <span class="text-emerald-400/70 group-hover:text-emerald-400 font-bold text-[11px] transition-colors">
                                    "//"
                                </span>
                                <span class="tracking-wide">{tab.label}</span>
                            </a>
                        }
                    }).collect_view()}
                </nav>

                // Actions (Resume Download)
                <div class="flex items-center gap-2.5 shrink-0">
                    <a
                        href="assets/resume.pdf"
                        target="_blank"
                        rel="noreferrer"
                        class="text-xs font-mono font-semibold px-3.5 py-1.5 rounded-full border border-emerald-700/60 bg-emerald-950/40 text-emerald-300 hover:bg-emerald-900/50 hover:text-emerald-200 hover:border-emerald-500/80 transition-all flex items-center gap-1.5 shadow-sm"
                        title="Download Latest Resume"
                    >
                        <span>"Resume"</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                            <polyline points="15 3 21 3 21 9"></polyline>
                            <line x1="10" y1="14" x2="21" y2="3"></line>
                        </svg>
                    </a>
                </div>
            </div>

            // Mobile Segmented Pill Bar (Smooth Horizontal Scroll)
            <div class="md:hidden flex items-center gap-1.5 overflow-x-auto px-4 py-2 border-t border-zinc-800/50 bg-[#09090b]/95 backdrop-blur-md scrollbar-none">
                {TABS.iter().map(|tab| {
                    view! {
                        <a
                            href=tab.href
                            class="shrink-0 px-3 py-1 rounded-full text-xs font-mono text-zinc-400 hover:text-zinc-100 bg-zinc-900/70 border border-zinc-800 hover:border-zinc-700 hover:bg-zinc-800 flex items-center gap-1 transition-colors"
                        >
                            <span class="text-emerald-400 font-bold text-[10px]">"//"</span>
                            <span>{tab.label}</span>
                        </a>
                    }
                }).collect_view()}
            </div>
        </header>
    }
}
