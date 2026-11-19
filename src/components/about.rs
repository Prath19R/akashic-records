use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-20 px-4 sm:px-6 lg:px-8 max-w-6xl mx-auto border-t border-zinc-900">
            <div class="space-y-4 mb-10">
                <div class="text-xs font-mono text-zinc-500 uppercase tracking-widest">"// 01. Engineering Profile"</div>
                <h2 class="text-2xl sm:text-3xl font-bold tracking-tight text-zinc-100">"About & Systems Focus"</h2>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                <div class="md:col-span-2 space-y-4 text-sm sm:text-base text-zinc-400 leading-relaxed font-sans">
                    <p>
                        "I am a Backend and Distributed Systems Engineer based in Pune, India. In my current role in enterprise R&D at "
                        <span class="text-zinc-200 font-medium">"NICE CXone"</span>
                        ", I focus on the "
                        <span class="text-zinc-200 font-medium">"ACDHub Gateway Platform"</span>
                        " — a greenfield real-time ingestion engine aggregating high-frequency events from Amazon Connect and Cisco UCCE."
                    </p>
                    <p>
                        "My core competencies lie in low-level systems engineering with "
                        <span class="text-zinc-200 font-mono text-xs sm:text-sm">"Rust & WASM"</span>
                        ", distributed message brokers ("
                        <span class="text-zinc-200 font-mono text-xs sm:text-sm">"Kafka/Amazon MSK"</span>
                        "), proxy customization ("
                        <span class="text-zinc-200 font-mono text-xs sm:text-sm">"Envoy"</span>
                        "), and high-performance in-memory state layers ("
                        <span class="text-zinc-200 font-mono text-xs sm:text-sm">"Redis Cluster & Valkey"</span>
                        ")."
                    </p>
                    <p>
                        "Whether optimizing cleanup jobs from 2 hours to 28 minutes or shaving latency down to sub-99ms p99 at enterprise scale, I believe in mechanical sympathy, rigorous profiling (Gatling, X-Ray), deterministic architectures, and clean code."
                    </p>

                    <div class="pt-4 grid grid-cols-2 gap-4 text-xs font-mono">
                        <div class="bg-zinc-900/60 border border-zinc-800/80 p-3.5 rounded-xl">
                            <span class="text-zinc-500 uppercase tracking-wider block mb-1 text-[10px]">"Location"</span>
                            <span class="text-zinc-200 font-medium">"Pune, India"</span>
                        </div>
                        <div class="bg-zinc-900/60 border border-zinc-800/80 p-3.5 rounded-xl">
                            <span class="text-zinc-500 uppercase tracking-wider block mb-1 text-[10px]">"Education"</span>
                            <span class="text-zinc-200 font-medium">"B.Tech CS · 88% Grade"</span>
                        </div>
                        <div class="bg-zinc-900/60 border border-zinc-800/80 p-3.5 rounded-xl">
                            <span class="text-zinc-500 uppercase tracking-wider block mb-1 text-[10px]">"Primary Focus"</span>
                            <span class="text-zinc-200 font-medium">"Distributed Streams & WASM"</span>
                        </div>
                        <div class="bg-zinc-900/60 border border-zinc-800/80 p-3.5 rounded-xl">
                            <span class="text-zinc-500 uppercase tracking-wider block mb-1 text-[10px]">"Contact"</span>
                            <span class="text-zinc-200 font-medium">"prathmeshranadive@gmail.com"</span>
                        </div>
                    </div>
                </div>

                // Right sidebar / principles
                <div class="space-y-4">
                    <div class="bg-zinc-900/40 border border-zinc-800/80 rounded-2xl p-5 space-y-4">
                        <h3 class="text-xs font-mono uppercase tracking-wider text-zinc-300 flex items-center gap-2">
                            <span class="w-1.5 h-1.5 rounded-full bg-zinc-400"></span>
                            "Core Architectural Principles"
                        </h3>
                        <ul class="space-y-3 text-xs text-zinc-400 font-sans">
                            <li class="flex items-start gap-2">
                                <span class="text-zinc-500 font-mono">"01."</span>
                                <div>
                                    <strong class="text-zinc-200 block">"Sub-100ms p99 Budgets"</strong>
                                    "Zero-allocation hot paths, asynchronous stream processing, and non-blocking I/O."
                                </div>
                            </li>
                            <li class="flex items-start gap-2">
                                <span class="text-zinc-500 font-mono">"02."</span>
                                <div>
                                    <strong class="text-zinc-200 block">"Zero Client Secrets"</strong>
                                    "Edge proxy validation, KV rate limiting, and server-side secret injection."
                                </div>
                            </li>
                            <li class="flex items-start gap-2">
                                <span class="text-zinc-500 font-mono">"03."</span>
                                <div>
                                    <strong class="text-zinc-200 block">"Deterministic Schema"</strong>
                                    "Universal event contracts, UUIDv5 mapping, and strict JSON Schema guards."
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}
