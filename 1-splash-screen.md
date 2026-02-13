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

1. On startup: generate WAV bytes in memory (`render_to_wav()`). Do NOT call `load_sound_from_bytes` yet — this avoids creating macroquad's internal AudioContext before user interaction.
2. On first touch/click/tap: call `load_sound_from_bytes` (creates AudioContext after user gesture) then immediately call `play_sound`. Both happen in the same frame, after interaction.
3. If either step fails, capture the error for display.

**Why delay loading:** macroquad's `quad-snd` creates a browser AudioContext during `load_sound_from_bytes`. If this happens before any user interaction, the AudioContext starts in a "suspended" state and may never resume. By deferring both load and play to after the first tap, the AudioContext is created in an "allowed" state.

The splash screen uses touch-friendly input — "Tap to continue" instead of "Press any key". All interaction works via touch (mouse clicks also work on desktop).

### Audio Debug Display

Any audio errors are displayed on screen above the build timestamp. This helps diagnose issues in deployed builds where we can't see a console. The `AudioManager` exposes a `status()` method that returns a string like:
- `"audio: wav ready, waiting for tap"` — WAV bytes generated, not yet loaded into audio system
- `"audio: loading..."` — `load_sound_from_bytes` in progress (after tap)
- `"audio: playing"` — loaded and playing successfully
- `"audio: load error: <message>"` — `load_sound_from_bytes` failed
- `"audio: render error"` — WAV generation failed

This text is shown in small faint text, same style as the build timestamp.

### Audio Debugging: Test Tone

To isolate audio issues on Safari/iPad, temporarily replace the full song with a minimal test tone: a 1-second 440Hz sine wave. This is the simplest possible WAV — if Safari's `decodeAudioData` can't handle it, nothing will work and the issue is in macroquad's audio pipeline on Safari.

The `render_to_wav()` function generates the test tone instead of the full song. The status display shows `"audio: TEST TONE wav ready"` so it's clear the test build is deployed. The WAV is still 44100 Hz, 16-bit, mono PCM — same format, just simpler content.

Additionally, the `AudioManager` tracks whether a second tap occurred after loading. macroquad's JS audio plugin creates the AudioContext during `load_sound_from_bytes`, but since this happens inside an `await` (not directly in the tap handler), the AudioContext may start "suspended". The JS plugin sets up touch/mousedown/keydown listeners to call `audioContext.resume()`, but these only fire on **subsequent** interactions. The status display shows:
- `"audio: playing (tap again to resume)"` — after first tap loads and plays
- `"audio: playing (resumed)"` — after a second tap fires the resume listeners

This helps determine if the AudioContext suspension is the root cause.

### Audio Debugging: Static File Loading

If `load_sound_from_bytes` doesn't produce audio on Safari even with a simple test tone, try loading audio as a static file instead. Generate `web/test_tone.wav` at build time (via a Rust helper), and use `macroquad::audio::load_sound("test_tone.wav")` instead of `load_sound_from_bytes`. This uses macroquad's XHR-based file loading path, which is more commonly tested. If this works but `load_sound_from_bytes` doesn't, the issue is specific to the in-memory loading path.

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
- [ ] Warrior sword swing animation on tap/click (visual touch confirmation)
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
