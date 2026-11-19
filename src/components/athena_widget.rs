use leptos::prelude::*;

#[component]
pub fn AthenaWidget() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let athena_url = move || {
        #[cfg(debug_assertions)]
        { "http://localhost:8081/?embed=true" }
        #[cfg(not(debug_assertions))]
        { "https://prath19r.github.io/athena/?embed=true" }
    };

    view! {
        <div class="fixed bottom-5 right-5 z-50 font-sans">
            // Expanded Chat Modal / Slide-up Box
            <div class=move || format!(
                "transition-all duration-300 ease-out origin-bottom-right mb-3 shadow-2xl rounded-2xl overflow-hidden border border-zinc-800 bg-zinc-950 {}",
                if is_open.get() {
                    "w-[90vw] sm:w-[420px] h-[580px] opacity-100 scale-100 pointer-events-auto"
                } else {
                    "w-[90vw] sm:w-[420px] h-0 opacity-0 scale-95 pointer-events-none"
                }
            )>
                <div class="h-11 bg-zinc-900 border-b border-zinc-800 px-3.5 flex items-center justify-between text-xs font-mono">
                    <div class="flex items-center gap-2">
                        <span class="text-sm">"🏛️"</span>
                        <span class="font-semibold text-zinc-200">"Athena AI Guide"</span>
                        <span class="text-[10px] text-zinc-500 font-mono">"(Rust WASM)"</span>
                    </div>
                    <button
                        type="button"
                        on:click=move |_| set_is_open.set(false)
                        class="p-1 rounded text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800 transition-colors"
                        title="Close Chat"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    </button>
                </div>
                <iframe
                    src=athena_url()
                    class="w-full h-[calc(100%-44px)] border-0 bg-[#09090b]"
                    title="Athena AI Portfolio Assistant"
                ></iframe>
            </div>

            // Floating Toggle Button
            <div class="flex justify-end">
                <button
                    type="button"
                    on:click=move |_| set_is_open.update(|v| *v = !*v)
                    class="group flex items-center gap-2.5 px-4 py-3 rounded-full bg-zinc-900 hover:bg-zinc-800 border border-zinc-700/80 hover:border-zinc-500 text-zinc-100 shadow-xl transition-all duration-200 active:scale-95"
                    title="Chat with Athena AI Assistant"
                >
                    <span class="text-lg group-hover:scale-110 transition-transform">"🏛️"</span>
                    <span class="text-xs font-mono font-medium hidden sm:inline-block">
                        {move || if is_open.get() { "Close Athena" } else { "Ask Athena AI" }}
                    </span>
                    <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                </button>
            </div>
        </div>
    }
}
