use leptos::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;

const SPRITES_WALK: [&str; 4] = [
    "assets/chibi/01_movement/nahida_walk_left_01.png?v=5",
    "assets/chibi/01_movement/nahida_walk_left_02.png?v=5",
    "assets/chibi/01_movement/nahida_walk_left_03.png?v=5",
    "assets/chibi/01_movement/nahida_walk_left_02.png?v=5",
];

const SPRITE_IMPACT: &str = "assets/chibi/02_physics_events/nahida_impact_01.png?v=5";
const SPRITE_RECOVER: &str = "assets/chibi/02_physics_events/nahida_recover_01.png?v=5";

const SPRITE_THINK: &str = "assets/chibi/03_chat_interaction/nahida_think_pose.png?v=5";
const SPRITE_PONDER: &str = "assets/chibi/03_chat_interaction/nahida_ponder_pose.png?v=5";
const SPRITE_ASK_INVITE: &str = "assets/chibi/03_chat_interaction/nahida_ask_invite.png?v=5";
const SPRITE_LOADING: &str = "assets/chibi/03_chat_interaction/nahida_loading_01.png?v=5";
const SPRITE_EXCITED: &str = "assets/chibi/03_chat_interaction/nahida_excited_01.png?v=5";
const SPRITE_ANSWERING: &str = "assets/chibi/03_chat_interaction/nahida_answering_01.png?v=5";

#[derive(Clone, Copy, Debug, PartialEq)]
enum CompanionState {
    Spawning,          // 0: Spawns at bottom-left
    PatrolRight,       // 1: Walks forward (facing right) along bottom towards right edge
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
    let (current_sprite, set_current_sprite) = signal(SPRITES_WALK[0]);
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
        let mut moving_right = true;

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

            // Transition into Chat Dock when opened (Stand and serve answer WHERE CLICKED ON)
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
                // When chat is closed, resume from current spot in same direction
                if moving_right {
                    state = CompanionState::PatrolRight;
                    set_flip.set(-1.0);
                    set_bubble_text.set("Ask Athena!");
                    set_bubble_icon.set("💬");
                } else {
                    state = CompanionState::PatrolLeft;
                    set_flip.set(1.0);
                    set_bubble_text.set("Exploring...");
                    set_bubble_icon.set("🌿");
                }
                state_time = 0.0;
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
                        moving_right = true;
                        set_bubble_text.set("Ask Athena!");
                        set_bubble_icon.set("💬");
                        set_bubble_dot.set("#34d399");
                    }
                }

                // State 1: Walking Left-to-Right along Bottom (Facing Right, calm slow stroll)
                CompanionState::PatrolRight => {
                    moving_right = true;
                    pos_x += 1.1; // Slower, relaxed walking speed
                    set_scale.set(1.0);
                    set_opacity.set(1.0);
                    set_flip.set(-1.0); // -1.0 faces forward to the right

                    // Gentle walk cycle: 01 -> 02 -> 03 -> 02
                    let frame_idx = (anim_tick / 8) % SPRITES_WALK.len();
                    set_current_sprite.set(SPRITES_WALK[frame_idx]);

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
                        moving_right = false;
                        set_flip.set(1.0); // 1.0 faces forward to the left
                        set_bubble_text.set("Exploring...");
                        set_bubble_icon.set("🌿");
                        set_bubble_dot.set("#10b981");
                    }
                }

                // State 4: Walking Right-to-Left along Bottom (Facing Left, calm slow stroll)
                CompanionState::PatrolLeft => {
                    moving_right = false;
                    pos_x -= 1.1; // Slower, relaxed walking speed
                    set_flip.set(1.0); // 1.0 faces forward to the left

                    // Gentle walk cycle: 01 -> 02 -> 03 -> 02
                    let frame_idx = (anim_tick / 8) % SPRITES_WALK.len();
                    set_current_sprite.set(SPRITES_WALK[frame_idx]);

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
                        moving_right = true;
                        set_flip.set(-1.0);
                        set_bubble_text.set("Ask Athena!");
                        set_bubble_icon.set("💬");
                    }
                }

                // State 6: Interrupted Chat (STAYS WHERE CLICKED ON, zero teleportation)
                CompanionState::InterruptedChat => {
                    set_flip.set(1.0);
                    set_opacity.set(1.0);

                    let cycle = (state_time / 3000.0) as usize % 2;
                    set_current_sprite.set(if cycle == 0 { SPRITE_ASK_INVITE } else { SPRITE_PONDER });
                }

                // State 7: Thinking & Loading (STAYS WHERE CLICKED ON)
                CompanionState::Thinking => {
                    let frame = (anim_tick / 8) % 2;
                    set_current_sprite.set(if frame == 0 { SPRITE_LOADING } else { SPRITE_THINK });
                    set_bubble_text.set("Thinking...");
                    set_bubble_icon.set("💭");
                    set_bubble_dot.set("#60a5fa");
                }

                // State 8: Answering (STAYS WHERE CLICKED ON)
                CompanionState::Answering => {
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

    let modal_layout = move || {
        let (win_w, win_h) = if let Some(w) = web_sys::window() {
            let width = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1200.0);
            let height = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
            (width, height)
        } else {
            (1200.0, 800.0)
        };

        // Thought bubble size: 460px wide, 540px high on desktop (readable size, not too big, not too small)
        let modal_width: f64 = 460.0_f64.min(win_w - 24.0);
        let modal_height: f64 = 540.0_f64.min(win_h - 165.0).max(380.0);

        // Center thought bubble horizontally over Athena's current X position
        let athena_center_x = x.get() + 40.0;
        let ideal_left = athena_center_x - (modal_width / 2.0);
        let modal_left = ideal_left.clamp(12.0, (win_w - modal_width - 12.0).max(12.0));

        // Position bottom of thought bubble directly above Athena (Athena is at bottom.get() + 96px height)
        let modal_bottom = bottom.get() + 116.0;

        // Tail connector offset (points directly down towards Athena)
        let tail_x = (athena_center_x - modal_left).clamp(36.0, modal_width - 36.0);

        (modal_left, modal_bottom, modal_width, modal_height, tail_x)
    };

    view! {
        <div class="font-sans">
            // Thought Bubble Chat Modal (Anchored directly above Athena)
            {move || {
                let (m_left, m_bottom, m_width, m_height, tail_x) = modal_layout();
                let open = is_open.get();

                view! {
                    <div
                        style=format!(
                            "position: fixed; left: {}px; bottom: {}px; width: {}px; height: {}px; z-index: 9995; transform-origin: {}px bottom; transition: transform 0.28s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.25s ease-out; opacity: {}; transform: scale({}); pointer-events: {};",
                            m_left,
                            m_bottom,
                            m_width,
                            m_height,
                            tail_x,
                            if open { "1" } else { "0" },
                            if open { "1" } else { "0.85" },
                            if open { "auto" } else { "none" },
                        )
                        class="flex flex-col"
                    >
                        // Main Thought Bubble Box
                        <div class="w-full h-full flex flex-col rounded-[32px] border-2 border-zinc-700/80 bg-zinc-950/98 shadow-[0_25px_60px_-15px_rgba(0,0,0,0.85),0_0_35px_rgba(52,211,153,0.18)] overflow-hidden">
                            // Thought Bubble Header
                            <div class="h-12 bg-zinc-900/90 border-b border-zinc-800/80 px-4 flex items-center justify-between text-xs font-mono select-none shrink-0">
                                <div class="flex items-center gap-2.5">
                                    <span class="text-base">"💭"</span>
                                    <div>
                                        <span class="font-bold text-zinc-100 text-sm">"Athena's Mind"</span>
                                        <span class="ml-2 text-[11px] text-emerald-400 font-mono">"AI Guide"</span>
                                    </div>
                                </div>
                                <button
                                    type="button"
                                    on:click=move |_| set_is_open.set(false)
                                    class="p-1.5 rounded-lg text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800 transition-colors"
                                    title="Close Chat"
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                        <line x1="18" y1="6" x2="6" y2="18"></line>
                                        <line x1="6" y1="6" x2="18" y2="18"></line>
                                    </svg>
                                </button>
                            </div>

                            // Chat Iframe
                            <iframe
                                src=athena_url()
                                class="w-full flex-1 border-0 bg-[#09090b]"
                                title="Athena AI Portfolio Assistant"
                            ></iframe>
                        </div>

                        // Thought Bubble Trail (Two descending circles pointing down towards Athena)
                        <div
                            style=format!(
                                "position: absolute; left: {}px; top: 100%; transform: translateX(-50%); pointer-events: none; margin-top: 4px;",
                                tail_x
                            )
                            class="flex flex-col items-center gap-1.5"
                        >
                            <div class="w-4 h-4 rounded-full bg-zinc-950 border-2 border-zinc-700 shadow-md"></div>
                            <div class="w-2.5 h-2.5 rounded-full bg-zinc-950 border-2 border-zinc-700 shadow-sm"></div>
                        </div>
                    </div>
                }
            }}

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
                // Floating Speech Bubble (Fades when thought bubble opens)
                <div
                    class=move || format!(
                        "relative mb-1 px-2.5 py-1 rounded-xl bg-zinc-950/90 border border-zinc-700/80 text-zinc-100 text-[11px] font-mono shadow-xl flex items-center gap-1.5 whitespace-nowrap transition-opacity duration-200 {}",
                        if is_open.get() { "opacity-0 pointer-events-none" } else { "opacity-100" }
                    )
                >
                    <span class="text-xs">{move || bubble_icon.get()}</span>
                    <span>{move || bubble_text.get()}</span>
                    <span
                        class="w-1.5 h-1.5 rounded-full animate-pulse"
                        style=move || format!("background-color: {};", bubble_dot.get())
                    ></span>
                </div>

                // Animated Nahida Sprite Frame (Bigger & clearly visible on bottom, fixed height to prevent growing/shrinking)
                <img
                    src=move || current_sprite.get()
                    alt="Nahida Chibi Companion"
                    class="h-24 w-auto drop-shadow-[0_8px_16px_rgba(0,0,0,0.6)] select-none pointer-events-none"
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
