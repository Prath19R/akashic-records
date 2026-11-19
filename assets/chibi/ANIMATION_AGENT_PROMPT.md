# Coding Agent Brief — Implement a Smooth 60 Hz Nahida Chibi Animator

You are working in the user's existing application/repository. Implement Nahida as a polished, interactive 2D character companion using the supplied individual PNG assets. **First inspect the repository and follow its existing framework, conventions, routing, styling, and test setup. Do not introduce a new framework or replace working architecture.** If the repo already has a character/assistant component, extend it rather than creating a competing system.

The objective is a stable **60 Hz presentation target** (one display update every ~16.67 ms on a 60 Hz display), smooth movement, predictable animation transitions, and low CPU/GPU cost. The source sheet contains a limited set of hand-drawn poses. A 60 Hz renderer does **not** mean inventing 60 unique drawings per second: hold each PNG for multiple display refreshes and use smooth position/rotation/opacity interpolation between pose changes.

---

## 1. Inspect and prepare the assets

1. Read `manifest.json`; do not hard-code guessed paths or dimensions.
2. Assets are under:
   - `01_movement/`
   - `02_physics_events/`
   - `03_chat_interaction/`
   - `04_emotions/`
   - `05_playful/`
3. Preserve the transparent PNGs. Do not use the original sprite sheet as a runtime background and do not re-add a checkerboard.
4. The PNGs have different tight-cropped dimensions. Create a fixed-size stage for each animation family:
   - Standing poses: bottom-center anchor.
   - Running / dragging: bottom-center or foot anchor, adjusted per frame if required.
   - Fall / parachute: center anchor.
   - Peek: bottom-center, while retaining the translucent panel.
5. Calculate a common render height for every multi-pose sequence (or use per-frame anchor metadata) so switching PNGs does not cause visible scale jumps. Keep `width: auto`, preserve aspect ratio, and never stretch the character.
6. Preload the current state and its likely next state. Decode PNGs before a transition when practical (`HTMLImageElement.decode()` or the equivalent for the existing stack). Avoid loading every asset repeatedly.

### Source gaps — do not fabricate files

The supplied sheet intentionally has gaps. Handle only the assets that exist:
- Running: left 01–06 and right 01–06.
- Dragged: left 01–03; right 01 and 03 only (`right_02` is absent).
- Dropped: one left and one right pose.
- Recovery: `ending_left_01`, `getting_02`, `rising_03`, `rising_04`.
- Angry: left 01–04 and right 01–04.
- Playful: one pose each for hop, twirl, and peek in left/right directions.
- Thinking: left 01–02 and right 01–02.
- Excited: left 01 and 03; right 01 and 03 only.
- Pout: three poses. The source repeats the label `R01` for poses 2 and 3; use the normalized file names `pout_01`–`pout_03`.
- Shock: four poses. The source repeats the L01/R01 labels for the second pair; use `shock_01`–`shock_04`.

There is no dedicated idle, walk, leave, or dissolve-out drawing. Do not silently create or reference those files. For a neutral standby, use `rising_04` as a temporary standing pose, with only a subtle procedural idle motion; document this fallback in code. A close/leave action may fade the companion with CSS, but it must not be presented as a source-authored exit animation.

---

## 2. Animation engine and 60 Hz timing

Implement a small reusable animator/controller, adapted to the repository's language and UI framework. Separate:
- **Character state** (idle, running, dragging, falling, recovering, thinking, answering, excited, angry, pout, shocked, playful, hidden).
- **Animation definition** (frame paths, frame duration, loop/one-shot, direction, anchor).
- **Playback clock** (elapsed time, current frame, completion callback).
- **Presentation transform** (x/y position, scale, rotation, opacity).

### Timing rules

- Prefer `requestAnimationFrame` for browser animation, or the framework's equivalent frame scheduler. Use the callback timestamp / elapsed time, not a counter that assumes every callback arrived exactly on time.
- Use an accumulator or elapsed-time calculation to advance source poses at their configured duration. The character may keep the same PNG visible across several 60 Hz updates.
- Never use `setInterval(16)` as the animation clock. It drifts and is throttled in background tabs.
- Keep transforms on the compositor where possible (`transform` and `opacity`). Avoid layout-triggering properties (`top`, `left`, `width`, `height`) on every tick.
- Clamp unusually large deltas after tab suspension; pause or reset playback when `document.visibilityState` changes. On return, do not fast-forward through a long queue of frames.
- Stop the render loop when the character is hidden or no transform is changing. Restart it on interaction.
- Use CSS keyframes / Web Animations for simple bobbing, hover, fades, and rotations when those can run independently of application state.
- Keep the update loop deterministic. Do not generate random motion each frame; use a bounded, seeded or precomputed idle pattern.

### Suggested starting timings

Treat these as tunable defaults and validate visually:

| Animation | Source poses | Suggested pose hold | Playback |
|---|---:|---:|---|
| Run left/right | 6 per direction | 75–95 ms | Loop |
| Dragged left | 3 | 100–130 ms | Loop while held |
| Dragged right | 2 available | 120–150 ms | Gentle two-pose loop; do not invent R02 |
| Dropped/parachute | 1 per direction | Static pose | Procedural float / descent |
| Soft fall | 1 | 350–500 ms | One-shot transform |
| Ending → Getting → Rising 03 → Rising 04 | 4 different poses | 160–240 ms each | One-shot recovery |
| Angry left/right | 4 per direction | 90–120 ms | Short loop while angry, then hold pose 04 |
| Hop / Twirl / Peek | 1 per direction each | Static pose | Procedural CSS movement |
| Think left/right | 2 per direction | 450–650 ms | Slow loop |
| Excited left/right | 2 per direction | 180–260 ms | Short burst, then hold pose 03 |
| Pout | 3 | 180–260 ms | One-shot, then hold pose 03 |
| Shock | 4 | 100–160 ms | One-shot, then hold pose 04 |

The values above are source-pose durations, not the display refresh rate. On a 60 Hz display, a 90 ms pose is held for roughly five or six refreshes while CSS transforms continue smoothly between positions.

---

## 3. Build an explicit state machine

Use a single source of truth for the active state. Every transition must define:
- the animation sequence to play;
- whether it loops or plays once;
- the final pose / fallback;
- the allowed next states;
- whether movement is enabled;
- whether the character should capture pointer input.

Recommended behavior:

### Standby / idle fallback
- Use `rising_04` as the temporary neutral standing pose because no idle frame is supplied.
- Add a very subtle vertical float (about 2–3 px over 2.5–3.5 s) and optional tiny scale/rotation variation. Keep it restrained; it should not look like walking or breathing if the artwork cannot support it.
- Stop unnecessary JS frame work while standby is static; CSS can handle this motion.

### Run
- Choose the correct left or right sequence from movement direction.
- Loop 01→06 with a duration-based clock.
- Use position interpolation for travel; never move by a fixed number of pixels per frame.
- If direction changes, switch to the opposite direction without resetting the character's world position.
- Do not mirror a left-facing frame unless inspection confirms it is safe; the sheet provides dedicated left and right assets.

### Drag
- Use pointer/touch events and pointer capture so the character remains attached to the pointer when the pointer leaves the image bounds.
- During drag, follow pointer coordinates with a small smoothing factor (or a critically damped spring); do not make the character lag excessively.
- Apply `touch-action: none` only to the draggable surface, not the whole page.
- Choose the direction set based on pointer movement. Use left 01–03 or right 01/03 only.
- On release, compute velocity from timestamped pointer samples, then transition to dropped/fall behavior.

### Drop / fall / recovery
- For parachute poses, use a slow downward drift and gentle sway; keep the pose static rather than rapidly cycling it.
- Use `soft_fall_01` for the impact/fall accent with a one-shot rotation/translation/scale animation.
- Then play `ending_left_01` → `getting_02` → `rising_03` → `rising_04` as a one-shot recovery.
- Add a short settle/bounce with easing, but do not overshoot far enough to look elastic or cartoony unless the product design calls for it.
- After recovery, return to standby or the prior interaction state.

### Thinking / answering
- While waiting for an answer, loop the left or right Think 01/02 pair slowly.
- When a response becomes available, transition to an Excited pose for a brief acknowledgement or use the excited pair as an answer emphasis.
- Return to the relevant standby pose when speaking/answering is complete.
- Do not animate continuously just because text is streaming; keep the thinking loop calm.

### Emotions
- Angry: use the four directional frames. Trigger only from an explicit app event or user-selected mood, not from inferred sentiment without product approval.
- Pout: play 01→02→03 once, then hold 03.
- Shock: play 01→02→03→04 once, then hold 04.
- Transitions should crossfade briefly only when a hard pose change is visually jarring. Avoid blending two semi-transparent characters for long periods.

### Playful
- Hop: small vertical arc and squash/stretch (subtle; preserve proportions).
- Twirl: rotate around the character's center with easing and return to zero rotation.
- Peek: slide in from behind the relevant panel edge and settle. Keep the translucent panel in the source image.
- Respect reduced-motion settings by replacing these with a simple pose swap or short fade.

---

## 4. Transitions and compositing

- Use a single character stage with a stable bounding box and a single active image element where possible. Avoid mounting a new image on every animation tick.
- Preload the next frame, then update the `src` at pose boundaries. Keep the stage's computed dimensions fixed during a sequence.
- Use `transform: translate3d(x, y, 0) rotate(...) scale(...)` for motion. Prefer one transform composition rather than stacking conflicting CSS transforms from multiple components.
- Use transform origins appropriate to the pose:
  - standing/run: near the feet;
  - fall: near the torso/center;
  - twirl: center of mass.
- Use easing (`ease-out`, cubic-bezier, spring with bounded damping) for movement between positions. The frame clock itself should remain time-based and stable.
- Avoid filters (large blur/drop-shadow) on every animation frame. If a shadow is needed, keep it subtle and static or animate only opacity.
- Do not scale beyond the visual resolution of the PNGs. On high-DPI screens, cap the rendered size to avoid blurry enlargement.

---

## 5. Input, responsiveness, and accessibility

- Support mouse, touch, and pen via Pointer Events.
- Keep hit targets larger than the visible character if appropriate, but do not stretch the artwork.
- Clamp character movement to the viewport / relevant container and account for scroll offsets.
- Make the companion responsive: preserve aspect ratio, set reasonable min/max rendered size, and avoid covering important content on narrow screens.
- Provide a clear keyboard-accessible way to open, close, or focus the companion if it is interactive.
- Use `prefers-reduced-motion: reduce`: disable continuous bobbing and playful motion, shorten transitions, and prefer static poses.
- Provide meaningful accessible text for the character interaction; mark purely decorative duplicate images as `aria-hidden`.
- Do not steal focus when the character animates or when a chat response arrives.

---

## 6. Suggested implementation API

Adapt names/types to the current project, but keep responsibilities similarly separated:

```ts
type Direction = "left" | "right";

type AnimationDefinition = {
  id: string;
  frames: string[];
  frameDurationMs: number;
  loop: boolean;
  anchor: "bottom-center" | "center";
  renderHeight?: number;
  finalFrame?: number;
};

type CharacterState =
  | "idle"
  | "running"
  | "dragging"
  | "dropped"
  | "falling"
  | "recovering"
  | "thinking"
  | "answering"
  | "excited"
  | "angry"
  | "pout"
  | "shocked"
  | "playful"
  | "hidden";

type AnimatorController = {
  play(animationId: string, options?: { loop?: boolean }): void;
  setDirection(direction: Direction): void;
  moveTo(x: number, y: number, durationMs: number): void;
  setState(state: CharacterState): void;
  pause(): void;
  resume(): void;
  destroy(): void;
};
```

The sample is a design guide, not a mandate to use TypeScript if the project uses another language. Make cleanup explicit: cancel animation frames, remove event listeners, release pointer capture, and clear pending callbacks on unmount/destroy.

---

## 7. Testing and performance acceptance criteria

Test at the actual target viewport sizes and at 60 Hz where available.

### Visual checks
- Every PNG renders with true alpha transparency; no checkerboard is visible around the character.
- Character scale and foot/center anchor do not jump when frames change.
- Running loops without a noticeable hitch at the 06→01 boundary.
- Left/right direction changes preserve position and do not flash the wrong direction.
- Dragging remains pointer-attached and release velocity is stable.
- Drop → soft fall → recovery plays once and returns to standby.
- Think → excited → answering transitions do not flicker.
- Pout and shock stop on their intended final poses.
- Peek panel remains intentional and does not look like a leftover background artifact.
- No missing-file 404s. Missing numbered frames are not referenced.

### Performance checks
- Aim for a steady 60 Hz presentation on a typical desktop; each frame budget is ~16.67 ms.
- The character update does not force synchronous layout or trigger React/Vue/Svelte state updates 60 times per second unnecessarily.
- Use compositor-friendly transforms and opacity.
- No duplicate RAF loops after state changes or component remounts.
- Pause work in background tabs and when hidden.
- No memory leaks after repeated drag, open, close, and state transitions.
- Confirm reduced-motion mode is usable.

### Deliverables
1. A reusable character component/controller integrated into the existing application.
2. A data-driven animation manifest/config referencing only the supplied PNGs.
3. A clear state-transition table or diagram in the repository.
4. Tests for frame sequencing, loop/one-shot behavior, state transitions, pointer cleanup, and missing-asset handling.
5. A short README section explaining how to add a new animation sequence and where its PNG files belong.

**Before finishing:** run the project's relevant lint, type-check, tests, and build commands. Report what was implemented, which commands passed, any failing checks, and any visual limitations caused by the source sheet. Do not claim a measured 60 FPS unless it was actually profiled on the target runtime.
