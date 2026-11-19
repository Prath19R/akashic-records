use leptos::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;

const SPRITES_RUN: [&str; 5] = [
    "assets/chibi/01_movement/nahida_run_01.png?v=3",
    "assets/chibi/01_movement/nahida_run_02.png?v=3",
    "assets/chibi/01_movement/nahida_run_03.png?v=3",
    "assets/chibi/01_movement/nahida_run_04.png?v=3",
    "assets/chibi/01_movement/nahida_run_05.png?v=3",
];

const SPRITES_WALK_LEFT: [&str; 3] = [
    "assets/chibi/01_movement/nahida_walk_left_01.png?v=3",
    "assets/chibi/01_movement/nahida_walk_left_02.png?v=3",
    "assets/chibi/01_movement/nahida_walk_left_03.png?v=3",
];

const SPRITE_IMPACT: &str = "assets/chibi/02_physics_events/nahida_impact_01.png?v=3";
const SPRITE_RECOVER: &str = "assets/chibi/02_physics_events/nahida_recover_01.png?v=3";

const SPRITE_THINK: &str = "assets/chibi/03_chat_interaction/nahida_think_pose.png?v=3";
const SPRITE_PONDER: &str = "assets/chibi/03_chat_interaction/nahida_ponder_pose.png?v=3";
const SPRITE_ASK_INVITE: &str = "assets/chibi/03_chat_interaction/nahida_ask_invite.png?v=3";
const SPRITE_LOADING: &str = "assets/chibi/03_chat_interaction/nahida_loading_01.png?v=3";
const SPRITE_EXCITED: &str = "assets/chibi/03_chat_interaction/nahida_excited_01.png?v=3";
const SPRITE_ANSWERING: &str = "assets/chibi/03_chat_interaction/nahida_answering_01.png?v=3";

#[derive(Clone, Copy, Debug, PartialEq)]
enum CompanionState {
    Spawning,          // 0: Spawns at bottom-left
    PatrolRight,       // 1: Runs forward (facing right) along bottom towards right edge
    EdgeContact,       // 2: Hits right edge, recoils slightly
    Recovery,          // 3: Recovers, shakes head, turns left
    PatrolLeft,        // 4: Walks forward (facing left) along bottom towards left edge
    LeftTurn,          // 5: Reaches left edge, turns around to face right
    InterruptedChat,   // 6: Docked beside open chatbox
    Thinking,          // 7: Athena AI generating response
    Answering,         // 8: Athena AI completed answer
}

#[component]
pub fn AthenaWidget() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Chibi Reactive State Signals (100% Rust)
    let (x, set_x) = signal(24.0);
    let (bottom, set_bottom) = signal(20.0);
    let (scale, set_scale) = signal(0.0);
    let (opacity, set_opacity) = signal(1.0);
    let (flip, set_flip) = signal(-1.0); // -1.0 flips native-left sprites to face right
    let (current_sprite, set_current_sprite) = signal(SPRITES_RUN[0]);
    let (bubble_text, set_bubble_text) = signal("Awakening...");
    let (bubble_icon, set_bubble_icon) = signal("🌱");
    let (bubble_dot, set_bubble_dot) = signal("#34d399");

    // Communication bridge from Athena iframe
    let (companion_event, set_companion_event) = signal(Option::<String>::None);

    let athena_url = move || {
        #[cfg(debug_assertions)]
        { "http://localhost:8081/?embed=true&v=1.1.0" }
        #[cfg(not(debug_assertions))]
        { "https://prath19r.github.io/athena/?embed=true&v=1.1.0" }
    };

    // Initialize 60fps Rust physics game-loop using gloo-timers Interval
    Effect::new(move |_| {
        let mut state = CompanionState::Spawning;
        let mut pos_x: f64 = 24.0;
        let pos_bottom: f64 = 20.0;
        let left_edge: f64 = 24.0;
        let mut state_time: f64 = 0.0;
        let mut anim_tick: usize = 0;
        let mut was_open = false;

        let interval = Interval::new(16, move || {
            let win = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            let win_w = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1200.0);
            let right_edge = (win_w - 76.0).max(140.0);

            state_time += 16.6;
            anim_tick = anim_tick.wrapping_add(1);

            let open = is_open.get();

            // Transition into Chat Dock when opened
            if open && !was_open {
                state = CompanionState::InterruptedChat;
                state_time = 0.0;
                set_bubble_text.set("Ask me anything!");
                set_bubble_icon.set("✨");
                set_bubble_dot.set("#34d399");
                set_current_sprite.set(SPRITE_ASK_INVITE);
                set_scale.set(1.0);
                set_opacity.set(1.0);
                set_flip.set(1.0);
            } else if !open && was_open {
                // When chat is closed, turn and resume bottom patrol left
                state = CompanionState::PatrolLeft;
                state_time = 0.0;
                set_flip.set(1.0);
                set_bubble_text.set("Exploring...");
                set_bubble_icon.set("🌿");
            }
            was_open = open;

            // Handle iframe events during chat mode
            if open {
                if let Some(ev) = companion_event.get() {
                    match ev.as_str() {
                        "THINKING" => {
                            state = CompanionState::Thinking;
                            state_time = 0.0;
                        }
                        "ANSWERING" => {
                            state = CompanionState::Answering;
                            state_time = 0.0;
                        }
                        "WAIT" => {
                            state = CompanionState::InterruptedChat;
                            state_time = 0.0;
                        }
                        _ => {}
                    }
                    set_companion_event.set(None);
                }
            }

            match state {
                // State 0: Spawning at Bottom-Left
                CompanionState::Spawning => {
                    pos_x = left_edge;
                    let sc = (state_time / 250.0).min(1.0);
                    set_scale.set(sc);
                    set_opacity.set(1.0);
                    set_flip.set(-1.0); // Face right
                    set_current_sprite.set(SPRITE_THINK);
                    set_bubble_text.set("Awakening...");
                    set_bubble_icon.set("🌱");
                    set_bubble_dot.set("#34d399");

                    if state_time >= 250.0 {
                        state = CompanionState::PatrolRight;
                        state_time = 0.0;
                        set_bubble_text.set("Ask Athena!");
                        set_bubble_icon.set("💬");
                        set_bubble_dot.set("#34d399");
                    }
                }

                // State 1: Running Left-to-Right along Bottom (Facing Right)
                CompanionState::PatrolRight => {
                    pos_x += 2.6; // Steady running speed
                    set_scale.set(1.0);
                    set_opacity.set(1.0);
                    set_flip.set(-1.0); // -1.0 makes native-left sprites face forward to the right

                    // 12fps running cycle
                    let frame = (anim_tick / 5) % SPRITES_RUN.len();
                    set_current_sprite.set(SPRITES_RUN[frame]);

                    if pos_x >= right_edge {
                        pos_x = right_edge;
                        state = CompanionState::EdgeContact;
                        state_time = 0.0;
                        set_current_sprite.set(SPRITE_IMPACT);
                        set_bubble_text.set("ouch!");
                        set_bubble_icon.set("💫");
                        set_bubble_dot.set("#f59e0b");
                    }
                }

                // State 2: Edge Impact Bounce at Right Edge
                CompanionState::EdgeContact => {
                    let bounce = (state_time / 300.0).min(1.0) * std::f64::consts::PI;
                    pos_x = right_edge - bounce.sin() * 7.0;

                    if state_time >= 300.0 {
                        state = CompanionState::Recovery;
                        state_time = 0.0;
                        set_current_sprite.set(SPRITE_RECOVER);
                        set_bubble_text.set("dizzy...");
                        set_bubble_icon.set("🌀");
                    }
                }

                // State 3: Recovery & Turn Around
                CompanionState::Recovery => {
                    if state_time < 350.0 {
                        set_current_sprite.set(SPRITE_RECOVER);
                        set_flip.set(-1.0);
                    } else if state_time < 750.0 {
                        set_current_sprite.set(SPRITE_THINK);
                        set_bubble_text.set("I'm okay! ✨");
                        set_bubble_icon.set("✨");
                        set_flip.set(1.0); // Turn around to face left
                    } else {
                        state = CompanionState::PatrolLeft;
                        state_time = 0.0;
                        set_flip.set(1.0); // 1.0 makes native-left sprites face forward to the left
                        set_bubble_text.set("Exploring...");
                        set_bubble_icon.set("🌿");
                        set_bubble_dot.set("#10b981");
                    }
                }

                // State 4: Walking Right-to-Left along Bottom (Facing Left)
                CompanionState::PatrolLeft => {
                    pos_x -= 1.8; // Steady, calm walk speed
                    set_flip.set(1.0); // 1.0 faces forward to the left

                    // 10fps walk cycle
                    let frame = (anim_tick / 6) % SPRITES_WALK_LEFT.len();
                    set_current_sprite.set(SPRITES_WALK_LEFT[frame]);

                    if pos_x <= left_edge {
                        pos_x = left_edge;
                        state = CompanionState::LeftTurn;
                        state_time = 0.0;
                        set_bubble_text.set("Turning... 🍃");
                        set_bubble_icon.set("🍃");
                    }
                }

                // State 5: Turn Around at Left Edge
                CompanionState::LeftTurn => {
                    pos_x = left_edge;
                    if state_time < 500.0 {
                        set_current_sprite.set(SPRITE_PONDER);
                        set_flip.set(1.0);
                    } else if state_time < 800.0 {
                        set_current_sprite.set(SPRITE_THINK);
                        set_flip.set(-1.0); // Turn to face right
                        set_bubble_text.set("Let's go! ✨");
                        set_bubble_icon.set("✨");
                    } else {
                        state = CompanionState::PatrolRight;
                        state_time = 0.0;
                        set_flip.set(-1.0);
                        set_bubble_text.set("Ask Athena!");
                        set_bubble_icon.set("💬");
                    }
                }

                // State 6: Interrupted Chat Dock (Waiting beside chat modal at bottom)
                CompanionState::InterruptedChat => {
                    let target_x = (win_w - 490.0).max(20.0);
                    pos_x += (target_x - pos_x) * 0.18;
                    set_flip.set(1.0);
                    set_opacity.set(1.0);

                    let cycle = (state_time / 3000.0) as usize % 2;
                    set_current_sprite.set(if cycle == 0 { SPRITE_ASK_INVITE } else { SPRITE_PONDER });
                }

                // State 7: Thinking & Loading
                CompanionState::Thinking => {
                    let target_x = (win_w - 490.0).max(20.0);
                    pos_x += (target_x - pos_x) * 0.18;

                    let frame = (anim_tick / 8) % 2;
                    set_current_sprite.set(if frame == 0 { SPRITE_LOADING } else { SPRITE_THINK });
                    set_bubble_text.set("Thinking...");
                    set_bubble_icon.set("💭");
                    set_bubble_dot.set("#60a5fa");
                }

                // State 8: Answering
                CompanionState::Answering => {
                    let target_x = (win_w - 490.0).max(20.0);
                    pos_x += (target_x - pos_x) * 0.18;

                    if state_time < 1000.0 {
                        set_current_sprite.set(SPRITE_EXCITED);
                        set_bubble_text.set("Got it!");
                        set_bubble_icon.set("🌟");
                        set_bubble_dot.set("#fbbf24");
                    } else {
                        let frame = (anim_tick / 8) % 2;
                        set_current_sprite.set(if frame == 0 { SPRITE_ANSWERING } else { SPRITE_ASK_INVITE });
                        set_bubble_text.set("Explaining...");
                        set_bubble_icon.set("✨");
                    }
                }
            }

            set_x.set(pos_x);
            set_bottom.set(pos_bottom);
        });

        interval.forget();
    });

    // Handle cross-frame messages from Athena chat in pure Rust
    let _ = window_event_listener_untyped("message", move |ev| {
        if let Ok(msg_ev) = ev.dyn_into::<web_sys::MessageEvent>() {
            if let Ok(val) = msg_ev.data().dyn_into::<js_sys::Object>() {
                if let Ok(ty) = js_sys::Reflect::get(&val, &"type".into()) {
                    if ty.as_string().as_deref() == Some("ATHENA_STATE") {
                        if let Ok(status) = js_sys::Reflect::get(&val, &"status".into()) {
                            if let Some(s) = status.as_string() {
                                set_companion_event.set(Some(s));
                            }
                        }
                    }
                }
            }
        }
    });

    let on_nahida_click = move |_| {
        set_is_open.update(|v| *v = !*v);
    };

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

            // Pure Rust Nahida Companion (Singleton: exactly one DOM instance)
            <div
                id="nahida_companion"
                on:click=on_nahida_click
                title="Click to talk with Athena AI Guide!"
                style=move || format!(
                    "position: fixed; left: {}px; bottom: {}px; opacity: {}; z-index: 9990; cursor: pointer; user-select: none; will-change: left, bottom; display: flex; flex-direction: column; align-items: center;",
                    x.get(),
                    bottom.get(),
                    opacity.get()
                )
            >
                // Floating Speech Bubble
                <div
                    class="relative mb-1 px-2.5 py-1 rounded-xl bg-zinc-950/90 border border-zinc-700/80 text-zinc-100 text-[11px] font-mono shadow-xl flex items-center gap-1.5 whitespace-nowrap"
                >
                    <span class="text-xs">{move || bubble_icon.get()}</span>
                    <span>{move || bubble_text.get()}</span>
                    <span
                        class="w-1.5 h-1.5 rounded-full animate-pulse"
                        style=move || format!("background-color: {};", bubble_dot.get())
                    ></span>
                </div>

                // Animated Nahida Sprite Frame (100% visible on bottom, no CSS transition lag)
                <img
                    src=move || current_sprite.get()
                    alt="Nahida Chibi Companion"
                    class="w-16 h-auto drop-shadow-[0_8px_16px_rgba(0,0,0,0.6)] select-none pointer-events-none"
                    style=move || format!(
                        "transform: scale({}) scaleX({}); transform-origin: bottom center;",
                        scale.get(),
                        flip.get()
                    )
                />
            </div>
        </div>
    }
}
