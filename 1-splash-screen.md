# Version 1: Splash Screen

## Goal

Create the project skeleton and a polished splash screen — the first thing players see when the game loads.

## What This Version Delivers

- Rust + macroquad project compiling to WASM
- A splash screen that runs in the browser via GitHub Pages
- Pixel art scene with a warrior, monster, and treasure
- An original 8-bit intro song that plays automatically on load
- "Tap to continue" prompt (leads nowhere yet — future versions add menus)
- Works on iPad — touch input, no keyboard required

## Splash Screen Design

### Visual Layout

```
┌──────────────────────────────────────────┐
│                                          │
│          ╔══════════════════╗             │
│          ║  DUNGEON CREATOR ║             │
│          ╚══════════════════╝             │
│                                          │
│     🗡️ Warrior    💀 Monster    💰 Chest  │
│                                          │
│   [ pixel art scene across the center ]  │
│   - Left: warrior holding sword, idle    │
│     animation (breathing / sword glint)  │
│   - Center: monster with glowing eyes,   │
│     subtle idle sway                     │
│   - Right: treasure chest with gold      │
│     shimmer effect                       │
│                                          │
│          ~ Tap to continue ~             │
│          (pulsing/fading text)           │
│                                          │
└──────────────────────────────────────────┘
```

### Title

- "DUNGEON CREATOR" in large pixel-art styled font
- Subtle glow or color cycle effect on the letters

### Pixel Art (Drawn Programmatically)

All art will be rendered using macroquad drawing primitives (rectangles, lines) to create a chunky pixel-art look. No external image assets needed for v1.

- **Warrior** — Blocky humanoid figure, sword in hand, simple 2-3 frame idle animation. **On tap/click: sword swing animation** (sword rotates forward and back over ~0.5s). This serves as visual confirmation that touch input is being detected.
- **Monster** — Horned/fanged creature, glowing red eyes, idle sway animation
- **Treasure** — Open chest with gold coins, shimmer sparkle effect

### Color Palette

Keep it retro. Suggested base palette (adjustable):
- Background: deep dungeon purple/black (`#1a1a2e`)
- Title text: gold (`#ffd700`)
- Warrior: silver armor (`#c0c0c0`), blue cape (`#4169e1`)
- Monster: dark green (`#2d5a27`), red eyes (`#ff0000`)
- Treasure: gold (`#ffd700`), brown chest (`#8b4513`)
- Prompt text: white with alpha fade

## Audio: 8-Bit Intro Song (In-Memory WAV + macroquad audio)

### Approach

No audio files on disk. No JavaScript. No `wasm-bindgen`, `web-sys`, or `cpal`. The intro song is defined as note data in Rust and **synthesized into a WAV byte buffer in memory** at startup using pure math. The WAV is then loaded and played through **macroquad's built-in audio system** (`load_sound_from_bytes` / `play_sound`). macroquad's `quad-snd` backend already handles WebAudio internally via `mq_js_bundle.js` — no additional JS or WASM bindings needed.

### Architecture

```
Song Data (Rust)       WAV Renderer (Rust)        macroquad audio
┌──────────────┐      ┌──────────────────┐      ┌──────────────────┐
│ Note structs │─────▶│ render_to_wav()   │─────▶│ load_sound_from  │
│ freq, dur,   │      │                  │      │ _bytes()         │
│ vol, wave    │      │ For each sample: │      │                  │
│              │      │  square wave     │      │ play_sound()     │
│ MELODY[]     │      │  triangle wave   │      │ (looped: true)   │
│ BASS[]       │      │  noise channel   │      │                  │
│ DRUMS[]      │      │  → mix to i16    │      │ quad-snd handles │
└──────────────┘      │  → WAV header    │      │ WebAudio via     │
                      │  → Vec<u8>       │      │ mq_js_bundle.js  │
                      └──────────────────┘      └──────────────────┘
```

### Song Definition Format

Notes are defined as Rust data — arrays of `Note` structs (unchanged from current `song.rs`):

```rust
struct Note {
    freq: f32,      // frequency in Hz (e.g. 440.0 = A4), 0.0 = rest
    duration: f32,  // in seconds
    volume: f32,    // 0.0 to 1.0
}

// Three channels, mixed into one WAV buffer:
const MELODY: &[Note] = &[...];  // square wave — main theme
const BASS: &[Note] = &[...];    // triangle wave — bass line
const DRUMS: &[Note] = &[...];   // noise — percussion hits
```

### WAV Rendering

A `render_to_wav()` function in `audio.rs`:

1. Calculate total duration from the melody channel
2. Allocate a float sample buffer (44100 Hz, mono)
3. For each channel, walk the note array and generate samples:
   - **Melody** — square wave: `if phase < 0.5 { +amp } else { -amp }`
   - **Bass** — triangle wave: linear ramp up/down per period
   - **Drums** — white noise bursts via LFSR
4. Apply per-note envelope (10ms attack, 20ms release) to avoid clicks
5. Mix all three channels by summing into the buffer
6. Convert to i16, clamp to range
7. Prepend a valid 44-byte WAV header (RIFF/fmt/data chunks)
8. Return `Vec<u8>` — a complete, valid WAV file in memory

### Music Specs

- **Style:** 8-bit chiptune, heroic/adventurous feel
- **Tempo:** ~140 BPM
- **Length:** 15-30 second loop
- **Sample rate:** 44100 Hz, 16-bit mono
- **Channels:**
  - Melody — square wave
  - Bass — triangle wave
  - Percussion — white noise bursts
- **Playback:** `play_sound()` with `looped: true`. Attempts to play immediately on load. Retries on first touch/click if browser blocked autoplay.

### Dependencies

```toml
[dependencies]
macroquad = "0.4"
```

**macroquad only.** No `cpal`, no `wasm-bindgen`, no `web-sys`, no `js-sys`. Zero additional dependencies.

### Autoplay & iPad Support

Audio playback strategy (handles browser autoplay policy):

1. On startup: generate WAV, load via `load_sound_from_bytes`, call `play_sound` immediately
2. If browser blocks autoplay (silent failure), the sound is already loaded and ready
3. On first touch/click/tap: call `play_sound` again as a retry
4. macroquad's internal WebAudio context gets resumed by user interaction with the canvas

The splash screen uses touch-friendly input — "Tap to continue" instead of "Press any key". All interaction works via touch (mouse clicks also work on desktop).

## Implementation Checklist

- [ ] Initialize Cargo project with macroquad dependency
- [ ] Set up WASM build target
- [ ] Create the game window with correct resolution
- [ ] Draw the background gradient/color
- [ ] Render "DUNGEON CREATOR" title with pixel-art style
- [ ] Draw warrior sprite using primitives
- [ ] Draw monster sprite using primitives
- [ ] Draw treasure chest sprite using primitives
- [ ] Add idle animations (breathing, sway, shimmer)
- [ ] Add pulsing "Tap to continue" text
- [ ] Add `build.rs` to capture compile timestamp
- [ ] Display build timestamp in small text at bottom-right of screen
- [ ] Define intro song as note data (melody, bass, drums) in `song.rs`
- [ ] Implement `render_to_wav()` in `audio.rs` — synthesize WAV in memory from note data
- [ ] Generate square, triangle, and noise waveforms with per-note envelopes
- [ ] Load WAV via `macroquad::audio::load_sound_from_bytes()`
- [ ] Play immediately on load + retry on first touch/click (autoplay handling)
- [ ] Use touch input — "Tap to continue" (iPad compatible)
- [ ] Create `web/index.html` wrapper
- [ ] Test in browser via WASM
- [ ] Deploy to GitHub Pages

## Files to Create/Modify

| File | Purpose |
|------|---------|
| `build.rs` | Captures compile timestamp as env var |
| `Cargo.toml` | Project config, macroquad dependency |
| `src/main.rs` | Entry point, game loop, splash screen state |
| `src/splash.rs` | Splash screen rendering and animation logic |
| `src/audio.rs` | WAV renderer (in-memory), macroquad audio playback with autoplay retry |
| `src/song.rs` | Intro song note data (melody, bass, drums arrays) |
| `web/index.html` | HTML wrapper for WASM |

## Success Criteria

The splash screen is done when:
1. `cargo build --target wasm32-unknown-unknown` compiles cleanly
2. Opening `index.html` in a browser shows the splash screen
3. Pixel art warrior, monster, and treasure are visible with animations
4. 8-bit music plays (immediately or after first tap)
5. "Tap to continue" text pulses on screen
6. Works on iPad via touch
7. Build timestamp visible at bottom-right corner
