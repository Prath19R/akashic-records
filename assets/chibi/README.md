# Nahida Chibi Assets

This archive contains **53 individual transparent PNG cutouts** extracted from the supplied Nahida sprite sheet.

## Folder structure

```text
nahida_chibi_assets/
├── 01_movement/
│   ├── run_left_01.png ... run_left_06.png
│   └── run_right_01.png ... run_right_06.png
├── 02_physics_events/
│   ├── dragged_left_01.png ... dragged_left_03.png
│   ├── dragged_right_01.png
│   ├── dragged_right_03.png
│   ├── dropped_left_01.png
│   ├── dropped_right_01.png
│   ├── soft_fall_01.png
│   ├── ending_left_01.png
│   ├── getting_02.png
│   ├── rising_03.png
│   └── rising_04.png
├── 03_chat_interaction/
│   ├── think_left_01.png, think_left_02.png
│   ├── think_right_01.png, think_right_02.png
│   ├── excited_left_01.png, excited_left_03.png
│   └── excited_right_01.png, excited_right_03.png
├── 04_emotions/
│   ├── angry_left_01.png ... angry_left_04.png
│   ├── angry_right_01.png ... angry_right_04.png
│   ├── pout_01.png ... pout_03.png
│   └── shock_01.png ... shock_04.png
├── 05_playful/
│   ├── hop_left_01.png, hop_right_01.png
│   ├── twirl_left_01.png, twirl_right_01.png
│   └── peek_left_01.png, peek_right_01.png
├── manifest.json
└── ANIMATION_AGENT_PROMPT.md
```

## Integration notes

- Use `manifest.json` as the source of truth for file paths and native dimensions.
- PNGs are tightly cropped with a small transparent safety margin. Dimensions vary; normalize each animation sequence in the UI using a shared render height and a consistent anchor (usually bottom-center for standing poses).
- The first row of running characters is packed closely together. A few cape/leaf extents approach adjacent frame boundaries, so inspect the running cycle at runtime and apply tiny per-frame horizontal offsets only if needed.
- The source sheet labels contain missing frame numbers. They are intentionally preserved:
  - `dragged_right_02` is not present.
  - Excited has only `left_01`, `left_03`, `right_01`, and `right_03`.
  - Pout has three poses; the source labels the second and third as `R01`.
  - Shock has four poses; the source repeats the `L01` / `R01` labels for the second pair.
  - There are no dedicated idle, walking, leaving, or dissolve-out frames in this sheet.
- The Peek sprites include their tinted translucent panel/effect as it appears in the source art.

## Transparency / extraction quality

The source used a checkerboard that was part of the flattened image, not a true alpha channel. The checkerboard was removed with a grayscale color-key pass and exported with alpha transparency. This is a best-effort extraction: some neutral-gray shading close to the checkerboard values may be slightly reduced, and a few tiny edge details may benefit from manual cleanup. The original source image is not modified.

## Animation

Read `ANIMATION_AGENT_PROMPT.md` for the detailed implementation brief. It targets a 60 Hz display/update loop while respecting the lower native drawing cadence of the supplied poses. Do not fabricate missing drawings to claim 60 unique frames per second.
