use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section id="hero" class="relative pt-20 pb-28 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto overflow-hidden">
            // Subtle ambient background glow
            <div class="absolute top-10 left-1/2 -translate-x-1/2 w-[550px] h-[250px] bg-zinc-800/20 rounded-full blur-[120px] pointer-events-none"></div>

            <div class="relative z-10 space-y-6">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-mono bg-zinc-900/90 border border-zinc-800 text-zinc-300 backdrop-blur-sm">
                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                    <span>"R&D Engineer @ NICE CXone · Distributed Systems"</span>
                </div>

                <div class="space-y-3">
                    <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-white font-sans">
                        "High-throughput systems."
                        <br />
                        <span class="text-zinc-500 font-mono text-3xl sm:text-4xl lg:text-5xl">
                            "Rust · WASM · Envoy · Kafka"
                        </span>
                    </h1>
                    <p class="max-w-2xl text-base sm:text-lg text-zinc-400 leading-relaxed font-sans">
                        "I engineer low-latency gateways, custom Envoy WebAssembly plugins, and distributed data pipelines. Reduced real-time streaming latency by 50%+ (p99 < 99ms) on enterprise contact-center platforms."
                    </p>
                </div>

                <div class="pt-2 flex flex-wrap items-center gap-3">
                    <a
                        href="#projects"
                        class="px-5 py-2.5 rounded-xl bg-zinc-100 text-zinc-950 hover:bg-white text-xs sm:text-sm font-semibold tracking-tight transition-all shadow-md hover:shadow-zinc-800/50 flex items-center gap-2"
                    >
                        <span>"View Engineering Projects"</span>
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </a>
                    <a
                        href="https://prath19r.github.io/athena/"
                        target="_blank"
                        rel="noreferrer"
                        class="px-4 py-2.5 rounded-xl border border-zinc-800 bg-zinc-900/80 hover:bg-zinc-800 text-zinc-200 text-xs sm:text-sm font-mono transition-colors flex items-center gap-2"
                    >
                        <span>"Launch Athena AI"</span>
                        <span class="text-xs">"🏛️"</span>
                    </a>
                    <a
                        href="assets/resume.pdf"
                        target="_blank"
                        rel="noreferrer"
                        class="px-4 py-2.5 rounded-xl border border-zinc-800/80 hover:border-zinc-700 bg-transparent text-zinc-400 hover:text-zinc-100 text-xs sm:text-sm font-mono transition-colors"
                    >
                        "Download Resume"
                    </a>
                </div>

                <div class="pt-8 border-t border-zinc-900 grid grid-cols-2 sm:grid-cols-4 gap-4 text-xs font-mono">
                    <div class="border-l border-zinc-800 pl-3">
                        <div class="text-lg font-bold text-zinc-200">"< 99 ms"</div>
                        <div class="text-zinc-500 uppercase tracking-wider text-[11px]">"p99 Ingestion Latency"</div>
                    </div>
                    <div class="border-l border-zinc-800 pl-3">
                        <div class="text-lg font-bold text-zinc-200">"15+ Services"</div>
                        <div class="text-zinc-500 uppercase tracking-wider text-[11px]">"Compliant & Modernized"</div>
                    </div>
                    <div class="border-l border-zinc-800 pl-3">
                        <div class="text-lg font-bold text-zinc-200">"100% Rust"</div>
                        <div class="text-zinc-500 uppercase tracking-wider text-[11px]">"WASM & Envoy Plugins"</div>
                    </div>
                    <div class="border-l border-zinc-800 pl-3">
                        <div class="text-lg font-bold text-zinc-200">"Zero Secrets"</div>
                        <div class="text-zinc-500 uppercase tracking-wider text-[11px]">"Edge Gateway Security"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
