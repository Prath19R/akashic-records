use leptos::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::PointerEvent;

// ─────────────────────────────────────────────────────────────────────────────
// 53 ASSET DEFINITIONS (manifest.json - nahida_chibi_assets_1)
// ─────────────────────────────────────────────────────────────────────────────

// 01 Movement (Dedicated left and right sequences - no artificial mirroring needed)
const RUN_LEFT: [&str; 6] = [
    "assets/chibi/01_movement/run_left_01.png",
    "assets/chibi/01_movement/run_left_02.png",
    "assets/chibi/01_movement/run_left_03.png",
    "assets/chibi/01_movement/run_left_04.png",
    "assets/chibi/01_movement/run_left_05.png",
    "assets/chibi/01_movement/run_left_06.png",
];

const RUN_RIGHT: [&str; 6] = [
    "assets/chibi/01_movement/run_right_01.png",
    "assets/chibi/01_movement/run_right_02.png",
    "assets/chibi/01_movement/run_right_03.png",
    "assets/chibi/01_movement/run_right_04.png",
    "assets/chibi/01_movement/run_right_05.png",
    "assets/chibi/01_movement/run_right_06.png",
];

// 02 Physics Events
const DRAGGED_LEFT: [&str; 3] = [
    "assets/chibi/02_physics_events/dragged_left_01.png",
    "assets/chibi/02_physics_events/dragged_left_02.png",
    "assets/chibi/02_physics_events/dragged_left_03.png",
];

// Right 02 is absent from source sheet; gently loop 01 and 03 as specified in brief
const DRAGGED_RIGHT: [&str; 2] = [
    "assets/chibi/02_physics_events/dragged_right_01.png",
    "assets/chibi/02_physics_events/dragged_right_03.png",
];

const DROPPED_LEFT: &str = "assets/chibi/02_physics_events/dropped_left_01.png";
const DROPPED_RIGHT: &str = "assets/chibi/02_physics_events/dropped_right_01.png";
const SOFT_FALL: &str = "assets/chibi/02_physics_events/soft_fall_01.png";

// One-shot recovery sequence: ending -> getting -> rising 03 -> rising 04
const RECOVERY_FRAMES: [&str; 4] = [
    "assets/chibi/02_physics_events/ending_left_01.png",
    "assets/chibi/02_physics_events/getting_02.png",
    "assets/chibi/02_physics_events/rising_03.png",
    "assets/chibi/02_physics_events/rising_04.png",
];

// Rising 04 is the designated neutral standing pose fallback (per prompt brief)
const POSE_STANDBY: &str = "assets/chibi/02_physics_events/rising_04.png";

// 03 Chat Interaction
const THINK_LEFT: [&str; 2] = [
    "assets/chibi/03_chat_interaction/think_left_01.png",
    "assets/chibi/03_chat_interaction/think_left_02.png",
];

const THINK_RIGHT: [&str; 2] = [
    "assets/chibi/03_chat_interaction/think_right_01.png",
    "assets/chibi/03_chat_interaction/think_right_02.png",
];

const EXCITED_LEFT: [&str; 2] = [
    "assets/chibi/03_chat_interaction/excited_left_01.png",
    "assets/chibi/03_chat_interaction/excited_left_03.png",
];

const EXCITED_RIGHT: [&str; 2] = [
    "assets/chibi/03_chat_interaction/excited_right_01.png",
    "assets/chibi/03_chat_interaction/excited_right_03.png",
];

// 05 Playful Actions
const HOP_LEFT: &str = "assets/chibi/05_playful/hop_left_01.png";
const HOP_RIGHT: &str = "assets/chibi/05_playful/hop_right_01.png";

// ─────────────────────────────────────────────────────────────────────────────
// CHARACTER STATE MACHINE
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharacterState {
    Spawning,          // Scale up pop-in on startup
    Standby,           // Neutral standing (rising_04) with subtle procedural float
    Running,           // 60 Hz patrol along bottom edge (85ms per frame)
    Dragging,          // Interactive pointer drag (follows pointer, plays dragged poses)
    Dropped,           // Parachute descent with gentle sine-wave drift
    SoftFall,          // Impact / recoil (soft_fall_01)
    Recovery,          // One-shot 4-pose rising sequence
    Thinking,          // AI thinking pose loop
    Answering,         // AI excited burst & answering pose
    ChatStandby,       // Idle docked beside open thought bubble
    PlayfulHop,        // Spontaneous joyful hop
}

#[component]
pub fn AthenaWidget() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Presentation Transforms & Pose Signals (Smooth 60 Hz Render Target)
    let (x, set_x) = signal(28.0);
    let (bottom, set_bottom) = signal(20.0);
    let (scale, set_scale) = signal(0.0);
    let (rotation, set_rotation) = signal(0.0);
    let (opacity, set_opacity) = signal(1.0);
    let (current_sprite, set_current_sprite) = signal(POSE_STANDBY);

    // Floating Speech Bubble
    let (bubble_text, set_bubble_text) = signal("Awakening...");
    let (bubble_icon, set_bubble_icon) = signal("🌱");
    let (bubble_dot, set_bubble_dot) = signal("#34d399");

    // Drag Interaction State (Signals for reactive event handlers)
    let (is_dragging_active, set_is_dragging_active) = signal(false);
    let (pointer_drag_pos, set_pointer_drag_pos) = signal(Option::<(f64, f64)>::None);
    let (pointer_just_dragged, set_pointer_just_dragged) = signal(false);

    // Cross-frame event bridge from Athena iframe
    let (companion_event, set_companion_event) = signal(Option::<String>::None);

    let athena_url = move || {
        #[cfg(debug_assertions)]
        { "http://localhost:8081/?embed=true&v=1.2.0" }
        #[cfg(not(debug_assertions))]
        { "https://prath19r.github.io/athena/?embed=true&v=1.2.0" }
    };

    // ─────────────────────────────────────────────────────────────────────────
    // 60 HZ RUST WASM ANIMATION & PHYSICS ENGINE
    // ─────────────────────────────────────────────────────────────────────────
    Effect::new(move |_| {
        let mut state = CharacterState::Spawning;
        let mut direction = Direction::Right;

        let mut pos_x: f64 = 28.0;
        let mut pos_bottom: f64 = 20.0;
        let floor_bottom: f64 = 20.0;
        let left_boundary: f64 = 24.0;
        let mut chat_anchor_bottom: f64 = 20.0;

        let mut state_time: f64 = 0.0;
        let mut anim_accum_ms: f64 = 0.0;
        let mut current_frame_idx: usize = 0;
        let mut was_open = false;

        let perf = web_sys::window().and_then(|w| w.performance());
        let mut last_now = perf.as_ref().map(|p| p.now()).unwrap_or(0.0);

        let interval = Interval::new(16, move || {
            let win = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            let win_w = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1200.0);
            let _win_h = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
            let right_boundary = (win_w - 90.0).max(120.0);

            // Compute delta time with tab-suspension clamping
            let current_now = perf.as_ref().map(|p| p.now()).unwrap_or(last_now + 16.67);
            let mut dt = current_now - last_now;
            last_now = current_now;
            if dt > 100.0 || dt <= 0.0 {
                dt = 16.67;
            }

            state_time += dt;
            anim_accum_ms += dt;

            // Handle Pointer Drag Override
            if is_dragging_active.get() {
                if let Some((target_x, target_bottom)) = pointer_drag_pos.get() {
                    let prev_x = pos_x;
                    pos_x = target_x.clamp(left_boundary, right_boundary);
                    pos_bottom = target_bottom.max(floor_bottom);

                    if pos_x < prev_x - 0.5 {
                        direction = Direction::Left;
                    } else if pos_x > prev_x + 0.5 {
                        direction = Direction::Right;
                    }

                    state = CharacterState::Dragging;
                    set_rotation.set(0.0);
                    set_scale.set(1.0);
                    set_opacity.set(1.0);

                    // Advance drag frame
                    if anim_accum_ms >= 120.0 {
                        anim_accum_ms = 0.0;
                        current_frame_idx = current_frame_idx.wrapping_add(1);
                    }

                    match direction {
                        Direction::Left => {
                            let f = current_frame_idx % DRAGGED_LEFT.len();
                            set_current_sprite.set(DRAGGED_LEFT[f]);
                        }
                        Direction::Right => {
                            let f = current_frame_idx % DRAGGED_RIGHT.len();
                            set_current_sprite.set(DRAGGED_RIGHT[f]);
                        }
                    }

                    set_bubble_text.set("Whoa! 🍃");
                    set_bubble_icon.set("💨");
                    set_bubble_dot.set("#60a5fa");

                    set_x.set(pos_x);
                    set_bottom.set(pos_bottom);
                    return;
                }
            } else if state == CharacterState::Dragging {
                // Released from drag: if released above ground, float down with parachute!
                if pos_bottom > floor_bottom + 15.0 {
                    state = CharacterState::Dropped;
                    state_time = 0.0;
                    anim_accum_ms = 0.0;
                    set_current_sprite.set(if direction == Direction::Left { DROPPED_LEFT } else { DROPPED_RIGHT });
                    set_bubble_text.set("Floating... ✨");
                    set_bubble_icon.set("🪂");
                    set_bubble_dot.set("#34d399");
                } else {
                    pos_bottom = floor_bottom;
                    state = CharacterState::Standby;
                    state_time = 0.0;
                }
            }

            // Sync with Chat Open / Close state
            let open = is_open.get();
            if open && !was_open {
                state = CharacterState::ChatStandby;
                state_time = 0.0;
                anim_accum_ms = 0.0;
                // Preserve floating altitude if clicked in mid-air (stays at current height)
                chat_anchor_bottom = pos_bottom.max(floor_bottom);
                pos_bottom = chat_anchor_bottom;

                set_current_sprite.set(POSE_STANDBY);
                set_bubble_text.set("Ask me anything!");
                set_bubble_icon.set("✨");
                set_bubble_dot.set("#34d399");
                set_rotation.set(0.0);
                set_scale.set(1.0);
            } else if !open && was_open {
                // If closed while elevated, continue floating down with parachute
                if pos_bottom > floor_bottom + 15.0 {
                    state = CharacterState::Dropped;
                    set_current_sprite.set(if direction == Direction::Left { DROPPED_LEFT } else { DROPPED_RIGHT });
                    set_bubble_text.set("Floating... ✨");
                    set_bubble_icon.set("🪂");
                    set_bubble_dot.set("#34d399");
                } else {
                    pos_bottom = floor_bottom;
                    state = CharacterState::Running;
                    set_bubble_text.set("Exploring...");
                    set_bubble_icon.set("🌿");
                }
                state_time = 0.0;
                anim_accum_ms = 0.0;
                set_rotation.set(0.0);
            }
            was_open = open;

            // Handle iframe event communication during chat
            if open {
                if let Some(ev) = companion_event.get() {
                    match ev.as_str() {
                        "THINKING" => {
                            state = CharacterState::Thinking;
                            state_time = 0.0;
                            anim_accum_ms = 0.0;
                            current_frame_idx = 0;
                        }
                        "ANSWERING" => {
                            state = CharacterState::Answering;
                            state_time = 0.0;
                            anim_accum_ms = 0.0;
                            current_frame_idx = 0;
                        }
                        "WAIT" => {
                            state = CharacterState::ChatStandby;
                            state_time = 0.0;
                            anim_accum_ms = 0.0;
                            set_current_sprite.set(POSE_STANDBY);
                        }
                        _ => {}
                    }
                    set_companion_event.set(None);
                }
            }

            // ─────────────────────────────────────────────────────────────────
            // STATE EXECUTION
            // ─────────────────────────────────────────────────────────────────
            match state {
                // 1. Spawning Pop-in
                CharacterState::Spawning => {
                    pos_bottom = floor_bottom;
                    let sc = (state_time / 250.0).min(1.0);
                    set_scale.set(sc);
                    set_opacity.set(1.0);
                    set_current_sprite.set(POSE_STANDBY);
                    set_rotation.set(0.0);

                    if state_time >= 250.0 {
                        state = CharacterState::Running;
                        direction = Direction::Right;
                        state_time = 0.0;
                        anim_accum_ms = 0.0;
                        set_bubble_text.set("Ask Athena!");
                        set_bubble_icon.set("💬");
                    }
                }

                // 2. Running Patrol (Smooth 60 Hz Translation + 85ms Pose Hold)
                CharacterState::Running => {
                    pos_bottom = floor_bottom;
                    set_scale.set(1.0);
                    set_opacity.set(1.0);
                    set_rotation.set(0.0);

                    // Speed: 1.1 px per frame for calm, steady walk
                    let speed = 1.1;

                    // Advance running pose every 85ms
                    if anim_accum_ms >= 85.0 {
                        anim_accum_ms = 0.0;
                        current_frame_idx = (current_frame_idx + 1) % 6;
                    }

                    match direction {
                        Direction::Right => {
                            pos_x += speed;
                            set_current_sprite.set(RUN_RIGHT[current_frame_idx]);

                            if pos_x >= right_boundary {
                                pos_x = right_boundary;
                                state = CharacterState::SoftFall;
                                state_time = 0.0;
                                anim_accum_ms = 0.0;
                                set_current_sprite.set(SOFT_FALL);
                                set_bubble_text.set("ouch!");
                                set_bubble_icon.set("💫");
                                set_bubble_dot.set("#f59e0b");
                            }
                        }
                        Direction::Left => {
                            pos_x -= speed;
                            set_current_sprite.set(RUN_LEFT[current_frame_idx]);

                            if pos_x <= left_boundary {
                                pos_x = left_boundary;
                                state = CharacterState::SoftFall;
                                state_time = 0.0;
                                anim_accum_ms = 0.0;
                                set_current_sprite.set(SOFT_FALL);
                                set_bubble_text.set("Turning... 🍃");
                                set_bubble_icon.set("🍃");
                                set_bubble_dot.set("#f59e0b");
                            }
                        }
                    }
                }

                // 3. Dropped / Parachute Float (Downwards drift with gentle sine sway)
                CharacterState::Dropped => {
                    pos_bottom -= 1.8; // Gentle downward speed
                    let sway = (state_time / 300.0).sin() * 1.5;
                    pos_x = (pos_x + sway).clamp(left_boundary, right_boundary);

                    set_current_sprite.set(if direction == Direction::Left { DROPPED_LEFT } else { DROPPED_RIGHT });
                    set_rotation.set(sway * 3.0);

                    if pos_bottom <= floor_bottom {
                        pos_bottom = floor_bottom;
                        state = CharacterState::SoftFall;
                        state_time = 0.0;
                        anim_accum_ms = 0.0;
                        set_rotation.set(0.0);
                        set_current_sprite.set(SOFT_FALL);
                        set_bubble_text.set("Landed softly! ✨");
                        set_bubble_icon.set("✨");
                    }
                }

                // 4. Soft Fall Impact / Recoil (350ms one-shot)
                CharacterState::SoftFall => {
                    set_current_sprite.set(SOFT_FALL);
                    let bounce = (state_time / 350.0).min(1.0) * std::f64::consts::PI;
                    set_rotation.set(if direction == Direction::Right { bounce.sin() * 6.0 } else { -bounce.sin() * 6.0 });

                    if state_time >= 350.0 {
                        state = CharacterState::Recovery;
                        state_time = 0.0;
                        anim_accum_ms = 0.0;
                        current_frame_idx = 0;
                        set_rotation.set(0.0);
                        set_bubble_text.set("Getting up...");
                        set_bubble_icon.set("🌱");
                    }
                }

                // 5. One-Shot Recovery (ending -> getting -> rising 03 -> rising 04)
                CharacterState::Recovery => {
                    set_rotation.set(0.0);
                    // 180ms per recovery frame
                    if anim_accum_ms >= 180.0 {
                        anim_accum_ms = 0.0;
                        current_frame_idx += 1;
                    }

                    if current_frame_idx < RECOVERY_FRAMES.len() {
                        set_current_sprite.set(RECOVERY_FRAMES[current_frame_idx]);
                    } else {
                        // Completed recovery: reverse direction and resume walking
                        direction = if direction == Direction::Right { Direction::Left } else { Direction::Right };
                        state = CharacterState::Running;
                        state_time = 0.0;
                        anim_accum_ms = 0.0;
                        current_frame_idx = 0;
                        set_bubble_text.set("Ask Athena!");
                        set_bubble_icon.set("💬");
                        set_bubble_dot.set("#34d399");
                    }
                }

                // 6. Playful Hop (450ms vertical arc)
                CharacterState::PlayfulHop => {
                    let progress = (state_time / 450.0).min(1.0);
                    let hop_y = (progress * std::f64::consts::PI).sin() * 24.0;
                    pos_bottom = floor_bottom + hop_y;
                    set_current_sprite.set(if direction == Direction::Left { HOP_LEFT } else { HOP_RIGHT });

                    if state_time >= 450.0 {
                        pos_bottom = floor_bottom;
                        state = CharacterState::Running;
                        state_time = 0.0;
                        anim_accum_ms = 0.0;
                    }
                }

                // 7. Thinking (Slow 500ms pose loop)
                CharacterState::Thinking => {
                    let float_y = (state_time / 1200.0).sin() * 1.5;
                    pos_bottom = chat_anchor_bottom + float_y;
                    set_rotation.set(0.0);
                    if anim_accum_ms >= 500.0 {
                        anim_accum_ms = 0.0;
                        current_frame_idx = (current_frame_idx + 1) % 2;
                    }

                    match direction {
                        Direction::Left => set_current_sprite.set(THINK_LEFT[current_frame_idx]),
                        Direction::Right => set_current_sprite.set(THINK_RIGHT[current_frame_idx]),
                    }
                    set_bubble_text.set("Thinking...");
                    set_bubble_icon.set("💭");
                    set_bubble_dot.set("#60a5fa");
                }

                // 8. Answering / Excited (220ms pose loop)
                CharacterState::Answering => {
                    let float_y = (state_time / 1200.0).sin() * 1.5;
                    pos_bottom = chat_anchor_bottom + float_y;
                    set_rotation.set(0.0);
                    if anim_accum_ms >= 220.0 {
                        anim_accum_ms = 0.0;
                        current_frame_idx = (current_frame_idx + 1) % 2;
                    }

                    match direction {
                        Direction::Left => set_current_sprite.set(EXCITED_LEFT[current_frame_idx]),
                        Direction::Right => set_current_sprite.set(EXCITED_RIGHT[current_frame_idx]),
                    }
                    set_bubble_text.set("Explaining...");
                    set_bubble_icon.set("✨");
                    set_bubble_dot.set("#fbbf24");
                }

                // 9. Standby / Neutral Standing (rising_04 with subtle procedural float)
                CharacterState::Standby => {
                    set_current_sprite.set(POSE_STANDBY);
                    set_rotation.set(0.0);
                    let float_y = (state_time / 1200.0).sin() * 1.5;
                    pos_bottom = floor_bottom + float_y;
                }

                CharacterState::ChatStandby => {
                    set_current_sprite.set(POSE_STANDBY);
                    set_rotation.set(0.0);
                    let float_y = (state_time / 1200.0).sin() * 1.5;
                    pos_bottom = chat_anchor_bottom + float_y;
                }

                CharacterState::Dragging => {}
            }

            set_x.set(pos_x);
            set_bottom.set(pos_bottom);
        });

        interval.forget();
    });

    // ─────────────────────────────────────────────────────────────────────────
    // CROSS-FRAME ATHENA STATE LISTENER
    // ─────────────────────────────────────────────────────────────────────────
    let _ = window_event_listener_untyped("message", move |ev| {
        if let Ok(msg_ev) = ev.dyn_into::<web_sys::MessageEvent>() {
            if let Ok(val) = msg_ev.data().dyn_into::<js_sys::Object>() {
                if let Ok(ty) = js_sys::Reflect::get(&val, &"type".into()) {
                    if ty.as_string().as_deref() == Some("ATHENA_STATE") {
                        if let Ok(status) = js_sys::Reflect::get(&val, &"state".into()) {
                            if let Some(s) = status.as_string() {
                                set_companion_event.set(Some(s));
                            }
                        }
                    }
                }
            }
        }
    });

    // ─────────────────────────────────────────────────────────────────────────
    // POINTER DRAG & CLICK HANDLERS
    // ─────────────────────────────────────────────────────────────────────────
    let drag_start = StoredValue::new((0.0, 0.0, 0.0, 0.0)); // (start_x, start_y, init_pos_x, init_pos_bottom)

    let on_pointer_down = move |ev: PointerEvent| {
        if let Some(target) = ev.current_target() {
            if let Ok(elem) = target.dyn_into::<web_sys::Element>() {
                let _ = elem.set_pointer_capture(ev.pointer_id());
            }
        }

        let sx = ev.client_x() as f64;
        let sy = ev.client_y() as f64;
        drag_start.set_value((sx, sy, x.get(), bottom.get()));
        set_is_dragging_active.set(false);
        set_pointer_just_dragged.set(false);
    };

    let on_pointer_move = move |ev: PointerEvent| {
        let (sx, sy, init_x, init_bottom) = drag_start.get_value();
        if sx == 0.0 && sy == 0.0 {
            return;
        }

        let cx = ev.client_x() as f64;
        let cy = ev.client_y() as f64;
        let dx = cx - sx;
        let dy = cy - sy;

        // If moved more than 6px, initiate drag mode
        if (dx * dx + dy * dy).sqrt() > 6.0 {
            set_is_dragging_active.set(true);
            set_pointer_just_dragged.set(true);

            let new_x = init_x + dx;
            let new_bottom = init_bottom - dy;
            set_pointer_drag_pos.set(Some((new_x, new_bottom)));
        }
    };

    let on_pointer_up = move |ev: PointerEvent| {
        if let Some(target) = ev.current_target() {
            if let Ok(elem) = target.dyn_into::<web_sys::Element>() {
                let _ = elem.release_pointer_capture(ev.pointer_id());
            }
        }

        let was_drag = pointer_just_dragged.get();
        drag_start.set_value((0.0, 0.0, 0.0, 0.0));
        set_is_dragging_active.set(false);
        set_pointer_drag_pos.set(None);

        // If it was a simple click without dragging, toggle chat modal
        if !was_drag {
            set_is_open.update(|v| *v = !*v);
        }
    };

    // ─────────────────────────────────────────────────────────────────────────
    // DYNAMIC THOUGHT BUBBLE LAYOUT (ANCHORED DIRECTLY ABOVE NAHIDA)
    // ─────────────────────────────────────────────────────────────────────────
    let modal_layout = move || {
        let (win_w, win_h) = if let Some(w) = web_sys::window() {
            let width = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1200.0);
            let height = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
            (width, height)
        } else {
            (1200.0, 800.0)
        };

        // Thought bubble size: 460px max width, 540px max height (readable, not too big or small)
        let modal_width: f64 = 460.0_f64.min(win_w - 24.0);
        let modal_height: f64 = 540.0_f64.min(win_h - 165.0).max(380.0);

        // Center thought bubble horizontally over Nahida's current X position
        let athena_center_x = x.get() + 48.0;
        let ideal_left = athena_center_x - (modal_width / 2.0);
        let modal_left = ideal_left.clamp(12.0, (win_w - modal_width - 12.0).max(12.0));

        // Position bottom of thought bubble directly above Nahida's head (+ gap for connector dots)
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

            // ─────────────────────────────────────────────────────────────────
            // 60 HZ INTERACTIVE NAHIDA CHIBI (STABLE BASELINE STAGE ANCHOR)
            // ─────────────────────────────────────────────────────────────────
            <div
                id="nahida_companion"
                on:pointerdown=on_pointer_down
                on:pointermove=on_pointer_move
                on:pointerup=on_pointer_up
                on:pointercancel=on_pointer_up
                title="Click to chat with Athena AI Guide! (Drag to move or parachute)"
                style=move || format!(
                    "position: fixed; left: {}px; bottom: {}px; opacity: {}; z-index: 9990; cursor: grab; user-select: none; touch-action: none; will-change: transform; transform: translate3d(0, 0, 0) rotate({}deg) scale({}); display: flex; flex-direction: column; align-items: center;",
                    x.get(),
                    bottom.get(),
                    opacity.get(),
                    rotation.get(),
                    scale.get()
                )
            >
                // Floating Speech Bubble (Fades when thought bubble opens)
                <div
                    class=move || format!(
                        "relative mb-3.5 px-2.5 py-1 rounded-xl bg-zinc-950/90 border border-zinc-700/80 text-zinc-100 text-[11px] font-mono shadow-xl flex items-center gap-1.5 whitespace-nowrap transition-opacity duration-200 {}",
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

                // Fixed-Height Stage Box (96px height, bottom-center foot anchor prevents all jumping)
                <div class="h-24 w-32 flex items-end justify-center pointer-events-none select-none">
                    <img
                        src=move || current_sprite.get()
                        alt="Nahida Chibi Companion"
                        class="max-h-24 w-auto object-contain object-bottom drop-shadow-[0_8px_16px_rgba(0,0,0,0.6)] select-none pointer-events-none"
                    />
                </div>
            </div>
        </div>
    }
}
