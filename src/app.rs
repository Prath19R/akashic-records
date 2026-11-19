use leptos::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::hero::Hero;
use crate::components::about::About;
use crate::components::skills::Skills;
use crate::components::projects::Projects;
use crate::components::experience::Experience;
use crate::components::contact::Contact;
use crate::components::athena_widget::AthenaWidget;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-[#09090b] text-zinc-100 flex flex-col font-sans selection:bg-zinc-800 selection:text-white">
            <Navbar />

            <main class="flex-1">
                <Hero />
                <About />
                <Skills />
                <Projects />
                <Experience />
                <Contact />
            </main>

            <footer class="border-t border-zinc-900 py-10 px-4 text-center text-xs font-mono text-zinc-500">
                <div class="max-w-6xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-4">
                    <div>
                        "© 2026 Prathmesh Ranadive · Built with Leptos & Rust WebAssembly"
                    </div>
                    <div class="flex items-center gap-4 text-zinc-400">
                        <a href="https://github.com/Prath19R" target="_blank" rel="noreferrer" class="hover:text-zinc-100 transition-colors">"GitHub"</a>
                        <span>"·"</span>
                        <a href="https://linkedin.com/in/prathmesh-ranadive" target="_blank" rel="noreferrer" class="hover:text-zinc-100 transition-colors">"LinkedIn"</a>
                        <span>"·"</span>
                        <a href="mailto:prathmeshranadive@gmail.com" class="hover:text-zinc-100 transition-colors">"Email"</a>
                    </div>
                </div>
            </footer>

            // Bottom-right floating interactive Athena guide
            <AthenaWidget />
        </div>
    }
}
