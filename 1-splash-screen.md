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

## Audio: 8-Bit Intro Song (Real-Time Synthesis via `cpal`)

### Approach

No audio files. No JavaScript. No pre-rendered WAV buffers. The intro song is defined as note data in Rust and **synthesized in real time** using the `cpal` crate. `cpal` provides a callback-based audio stream — the browser's WebAudio system asks for samples and our synth generates them on the fly. This is true real-time audio synthesis, pure Rust.

### Why `cpal` instead of macroquad audio

macroquad's audio (`macroquad::audio`) is designed for loading and playing audio files. It doesn't expose a real-time sample callback. `cpal` is the standard Rust crate for low-level audio I/O and has a WASM/WebAudio backend. It gives us a callback where we fill audio buffers sample-by-sample — exactly what a synthesizer needs.

### Architecture

```
Song Data (Rust)       Real-Time Synth (Rust)     cpal (WASM backend)
┌──────────────┐      ┌──────────────────┐      ┌──────────────────┐
│ Note structs │─────▶│ Audio callback:   │─────▶│ WebAudio via     │
│ freq, dur,   │      │                  │      │ cpal's WASM      │
│ vol, wave    │      │ "Give me next N  │      │ backend           │
│              │      │  samples"        │      │                  │
│ MELODY[]     │      │                  │      │ AudioWorklet /   │
│ BASS[]       │      │ Walk note arrays │      │ ScriptProcessor  │
│ DRUMS[]      │      │ Generate square, │      │ → speakers       │
└──────────────┘      │ triangle, noise  │      └──────────────────┘
                      │ Mix & output     │
                      └──────────────────┘
```

### Song Definition Format

Notes are defined as Rust data — arrays of `Note` structs (unchanged from current `song.rs`):

```rust
struct Note {
    freq: f32,      // frequency in Hz (e.g. 440.0 = A4), 0.0 = rest
    duration: f32,  // in seconds
    volume: f32,    // 0.0 to 1.0
}

// Three channels, synthesized in parallel in the audio callback:
const MELODY: &[Note] = &[...];  // square wave — main theme
const BASS: &[Note] = &[...];    // triangle wave — bass line
const DRUMS: &[Note] = &[...];   // noise — percussion hits
```

### Real-Time Synthesis

The audio callback in `audio.rs`:

1. `cpal` opens a default output stream with a sample callback
2. Each call asks for N samples to fill a buffer
3. For each sample, the synth tracks playback position across all three channels:
   - **Melody** — square wave: `if phase < 0.5 { +amp } else { -amp }`
   - **Bass** — triangle wave: linear ramp up/down per period
   - **Drums** — white noise bursts via LFSR
4. Apply per-note envelope (10ms attack, 20ms release) to avoid clicks
5. Mix channels by summing, clamp to output range
6. When all notes in a channel are exhausted, loop back to the start

### Music Specs

- **Style:** 8-bit chiptune, heroic/adventurous feel
- **Tempo:** ~140 BPM
- **Length:** 15-30 second loop
- **Sample rate:** determined by `cpal` default device (typically 44100 or 48000 Hz)
- **Channels:**
  - Melody — square wave
  - Bass — triangle wave
  - Percussion — white noise bursts
- **Playback:** Stream starts automatically when the splash screen loads. Loops seamlessly. Note: some browsers may block autoplay — if so, audio begins on first touch/click.

### Dependencies

```toml
[dependencies]
macroquad = "0.4"
cpal = "0.15"
```

No `wasm-bindgen`, no `web-sys`, no `js-sys`. `cpal` handles the WASM/WebAudio bridge internally.

### Browser Autoplay Handling

The `cpal` output stream is created on first user keypress/click. Before interaction, the splash screen shows "~ Press any key to start ~".

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
- [ ] Implement real-time synth via `cpal` audio callback in `audio.rs`
- [ ] Generate square, triangle, and noise waveforms sample-by-sample
- [ ] Loop song seamlessly when end is reached
- [ ] Start audio stream on first user interaction (browser autoplay policy)
- [ ] Create `web/index.html` wrapper
- [ ] Test in browser via WASM
- [ ] Deploy to GitHub Pages

## Files to Create/Modify

| File | Purpose |
|------|---------|
| `Cargo.toml` | Project config, macroquad dependency |
| `src/main.rs` | Entry point, game loop, splash screen state |
| `src/splash.rs` | Splash screen rendering and animation logic |
| `src/audio.rs` | Real-time synth engine via `cpal`, waveform generation |
| `src/song.rs` | Intro song note data (melody, bass, drums arrays) |
| `web/index.html` | HTML wrapper for WASM |

## Success Criteria

The splash screen is done when:
1. `cargo build --target wasm32-unknown-unknown` compiles cleanly
2. Opening `index.html` in a browser shows the splash screen
3. Pixel art warrior, monster, and treasure are visible with animations
4. 8-bit music plays after first user interaction
5. "Press any key" text pulses on screen
