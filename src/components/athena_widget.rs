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

    let nahida_img_src = "assets/nahida.png";

    view! {
        <div class="font-sans">
            // Expanded Chat Modal / Slide-up Box
            <div class=move || format!(
                "fixed bottom-24 right-4 sm:right-6 z-50 transition-all duration-300 ease-out origin-bottom-right shadow-2xl rounded-2xl overflow-hidden border border-zinc-800 bg-zinc-950 {}",
                if is_open.get() {
                    "w-[90vw] sm:w-[420px] h-[580px] opacity-100 scale-100 pointer-events-auto"
                } else {
                    "w-[90vw] sm:w-[420px] h-0 opacity-0 scale-95 pointer-events-none"
                }
            )>
                <div class="h-11 bg-zinc-900 border-b border-zinc-800 px-3.5 flex items-center justify-between text-xs font-mono">
                    <div class="flex items-center gap-2">
                        <img
                            src=nahida_img_src
                            alt="Athena Chibi"
                            class="w-5 h-5 rounded-full object-cover border border-emerald-500/40 bg-zinc-800"
                        />
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

            // Interactive Anime Chibi Avatar (Nahida)
            {move || {
                if !is_open.get() {
                    // ROAMING MODE: Walks across the screen, bonks into walls, falls down, gets up, turns around!
                    view! {
                        <div
                            class="nahida-mover group"
                            on:click=move |_| set_is_open.set(true)
                            title="Click to chat with Athena AI Guide!"
                        >
                            // Speech Bubble floating above her head (stays upright)
                            <div class="relative flex justify-center mb-1">
                                // Normal walking bubble
                                <div class="nahida-bubble-walking absolute bottom-0 px-2.5 py-1 rounded-xl bg-zinc-900/95 border border-zinc-700/80 text-zinc-100 text-[11px] font-mono shadow-lg whitespace-nowrap flex items-center gap-1.5 transition-transform group-hover:scale-105 group-hover:border-zinc-500">
                                    <span class="text-xs">"💬"</span>
                                    <span>"Ask Athena!"</span>
                                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                                </div>
                                // Bonk bubble when she hits the wall and falls down
                                <div class="nahida-bubble-bonk absolute bottom-0 px-2 py-0.5 rounded-lg bg-amber-950/90 border border-amber-500/80 text-amber-200 text-[11px] font-mono font-bold shadow-lg whitespace-nowrap pointer-events-none">
                                    "💫 bonk!"
                                </div>
                            </div>

                            // Character Sprite with run & tumble animations
                            <div class="nahida-sprite">
                                <img
                                    src=nahida_img_src
                                    alt="Athena Nahida Chibi"
                                    class="w-16 h-auto drop-shadow-[0_8px_16px_rgba(0,0,0,0.6)] select-none pointer-events-none transition-transform group-hover:brightness-110"
                                />
                            </div>
                        </div>
                    }.into_any()
                } else {
                    // OPEN / DOCKED MODE: Sits happily next to the chat modal
                    view! {
                        <div
                            class="fixed bottom-3 right-5 z-50 flex items-center gap-2 cursor-pointer group"
                            on:click=move |_| set_is_open.set(false)
                            title="Click to close Athena"
                        >
                            <div class="px-2.5 py-1 rounded-xl bg-zinc-900/90 border border-zinc-700 text-zinc-200 text-[11px] font-mono shadow-md flex items-center gap-1.5 group-hover:bg-zinc-800 transition-colors">
                                <span class="text-xs">"✕"</span>
                                <span>"Close Athena"</span>
                            </div>
                            <img
                                src=nahida_img_src
                                alt="Athena Nahida Chibi"
                                class="w-14 h-auto drop-shadow-lg select-none group-hover:scale-105 transition-transform animate-bounce"
                                style="animation-duration: 2s;"
                            />
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
