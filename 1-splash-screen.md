# Version 1: Splash Screen

## Goal

Create the project skeleton and a polished splash screen — the first thing players see when the game loads.

## What This Version Delivers

- Rust + macroquad project compiling to WASM
- A splash screen that runs in the browser via GitHub Pages
- Pixel art scene with a warrior, monster, and treasure
- An original 8-bit intro song playing on loop
- "Press any key to continue" prompt (leads nowhere yet — future versions add menus)

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
│        ~ Press any key to start ~        │
│          (pulsing/fading text)           │
│                                          │
└──────────────────────────────────────────┘
```

### Title

- "DUNGEON CREATOR" in large pixel-art styled font
- Subtle glow or color cycle effect on the letters

### Pixel Art (Drawn Programmatically)

All art will be rendered using macroquad drawing primitives (rectangles, lines) to create a chunky pixel-art look. No external image assets needed for v1.

- **Warrior** — Blocky humanoid figure, sword in hand, simple 2-3 frame idle animation
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

## Audio: 8-Bit Intro Song (Pure Rust PCM + macroquad audio)

### Approach

No audio files. No JavaScript. No `wasm-bindgen` or `web-sys`. The intro song is defined as note data in Rust, rendered into a **raw PCM WAV byte buffer** at startup using pure math, and played through **macroquad's built-in audio system** (`load_sound_from_bytes`). macroquad already handles the WebAudio bridge internally for WASM — we just hand it a WAV.

### Architecture

```
Song Data (Rust)       PCM Renderer (Rust)        macroquad audio
┌──────────────┐      ┌──────────────────┐      ┌──────────────────┐
│ Note structs │─────▶│ render_to_wav()   │─────▶│ load_sound_from  │
│ freq, dur,   │      │                  │      │ _bytes()         │
│ vol, wave    │      │ For each sample: │      │                  │
│              │      │  square wave     │      │ play_sound()     │
│ MELODY[]     │      │  triangle wave   │      │ (looped)         │
│ BASS[]       │      │  noise channel   │      │                  │
│ DRUMS[]      │      │  → mix to i16    │      │ Handles WebAudio │
└──────────────┘      │  → WAV bytes     │      │ internally       │
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

// Three channels, rendered in parallel into one mixed buffer:
const MELODY: &[Note] = &[...];  // square wave — main theme
const BASS: &[Note] = &[...];    // triangle wave — bass line
const DRUMS: &[Note] = &[...];   // noise — percussion hits
```

### PCM Rendering

A `render_to_wav()` function in `audio.rs` does the synthesis:

1. Allocate a sample buffer (sample rate: 44100 Hz, 16-bit mono)
2. For each channel, walk the note array and generate samples:
   - **Melody** — square wave: `if (phase % period) < half_period { +amp } else { -amp }`
   - **Bass** — triangle wave: sawtooth-based triangle formula
   - **Drums** — white noise bursts: random values scaled by amplitude
3. Mix all three channels by summing samples (with clipping to i16 range)
4. Prepend a valid WAV header (44 bytes: RIFF, fmt chunk, data chunk)
5. Return the complete WAV as `Vec<u8>`

### Music Specs

- **Style:** 8-bit chiptune, heroic/adventurous feel
- **Tempo:** ~140 BPM
- **Length:** 15-30 second loop
- **Sample rate:** 44100 Hz, 16-bit mono
- **Channels:**
  - Melody — square wave
  - Bass — triangle wave
  - Percussion — white noise bursts
- **Playback:** `macroquad::audio::play_sound()` with `looped: true`. Starts on first user interaction to satisfy browser autoplay policy.

### Dependencies

**None beyond macroquad.** No `wasm-bindgen`, no `web-sys`, no `js-sys`. The only dependency is `macroquad = "0.4"` which already provides `macroquad::audio`.

### Browser Autoplay Handling

macroquad's audio plays through the browser's WebAudio internally. Audio starts on first user keypress/click. Before interaction, the splash screen shows "~ Press any key to start ~".

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
- [ ] Add pulsing "Press any key" text
- [ ] Define intro song as note data (melody, bass, drums) in `song.rs`
- [ ] Implement `render_to_wav()` PCM synthesizer in `audio.rs` (square, triangle, noise)
- [ ] Load generated WAV via `macroquad::audio::load_sound_from_bytes()`
- [ ] Play looped audio on first user interaction (browser autoplay policy)
- [ ] Create `web/index.html` wrapper
- [ ] Test in browser via WASM
- [ ] Deploy to GitHub Pages

## Files to Create/Modify

| File | Purpose |
|------|---------|
| `Cargo.toml` | Project config, macroquad dependency |
| `src/main.rs` | Entry point, game loop, splash screen state |
| `src/splash.rs` | Splash screen rendering and animation logic |
| `src/audio.rs` | PCM WAV renderer, macroquad audio playback |
| `src/song.rs` | Intro song note data (melody, bass, drums arrays) |
| `web/index.html` | HTML wrapper for WASM |

## Success Criteria

The splash screen is done when:
1. `cargo build --target wasm32-unknown-unknown` compiles cleanly
2. Opening `index.html` in a browser shows the splash screen
3. Pixel art warrior, monster, and treasure are visible with animations
4. 8-bit music plays after first user interaction
5. "Press any key" text pulses on screen
