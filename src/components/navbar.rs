use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-40 w-full border-b border-zinc-800/80 bg-[#09090b]/80 backdrop-blur-md transition-all">
            <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
                <a href="#hero" class="flex items-center gap-2.5 group">
                    <span class="w-8 h-8 rounded-lg bg-zinc-900 border border-zinc-800 flex items-center justify-center font-mono text-sm font-semibold text-zinc-100 group-hover:border-zinc-700 transition-colors">
                        "PR"
                    </span>
                    <div class="flex flex-col">
                        <span class="text-sm font-semibold tracking-tight text-zinc-100 group-hover:text-white transition-colors">
                            "Prathmesh Ranadive"
                        </span>
                        <span class="text-[10px] font-mono text-zinc-500 uppercase tracking-widest">
                            "Systems Engineer"
                        </span>
                    </div>
                </a>

                <div class="hidden md:flex items-center gap-6 text-xs font-mono">
                    <a href="#about" class="text-zinc-400 hover:text-zinc-100 transition-colors">"// about"</a>
                    <a href="#skills" class="text-zinc-400 hover:text-zinc-100 transition-colors">"// skills"</a>
                    <a href="#projects" class="text-zinc-400 hover:text-zinc-100 transition-colors">"// projects"</a>
                    <a href="#experience" class="text-zinc-400 hover:text-zinc-100 transition-colors">"// experience"</a>
                    <a href="#contact" class="text-zinc-400 hover:text-zinc-100 transition-colors">"// contact"</a>
                </div>

                <div class="flex items-center gap-3">
                    <a
                        href="assets/resume.pdf"
                        target="_blank"
                        rel="noreferrer"
                        class="text-xs font-mono px-3 py-1.5 rounded-lg border border-zinc-700 bg-zinc-900 text-zinc-200 hover:bg-zinc-800 hover:text-white transition-all flex items-center gap-1.5 shadow-sm"
                    >
                        <span>"Resume"</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                            <polyline points="15 3 21 3 21 9"></polyline>
                            <line x1="10" y1="14" x2="21" y2="3"></line>
                        </svg>
                    </a>
                </div>
            </div>
        </nav>
    }
}
