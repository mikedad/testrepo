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

## Audio: 8-Bit Intro Song (WebAudio Synthesizer)

### Approach

No audio files. The intro song is defined as note data in Rust and synthesized at runtime using the browser's **WebAudio API** via `web-sys` and `wasm-bindgen`. The browser's built-in `OscillatorNode` generates square, triangle, and noise waveforms — authentic 8-bit sound with zero file overhead.

### Architecture

```
Song Data (Rust)          WebAudio API (Browser)
┌──────────────┐         ┌─────────────────────┐
│ Note structs │───JS───▶│ AudioContext         │
│ freq, dur,   │  FFI    │ ├─ OscillatorNode    │
│ vol, wave    │         │ │  (square/triangle)  │
│              │         │ ├─ GainNode (volume)  │
│ MELODY[]     │         │ └─ destination        │
│ BASS[]       │         │    (speakers)         │
│ DRUMS[]      │         └─────────────────────┘
└──────────────┘
```

### Song Definition Format

Notes are defined as Rust data — arrays of `Note` structs:

```rust
struct Note {
    freq: f32,      // frequency in Hz (e.g. 440.0 = A4), 0.0 = rest
    duration: f32,  // in seconds
    volume: f32,    // 0.0 to 1.0
}

// Three channels, scheduled in parallel:
const MELODY: &[Note] = &[...];  // square wave — main theme
const BASS: &[Note] = &[...];    // triangle wave — bass line
const DRUMS: &[Note] = &[...];   // noise — percussion hits
```

### Music Specs

- **Style:** 8-bit chiptune, heroic/adventurous feel
- **Tempo:** ~120 BPM
- **Length:** 15-30 second loop
- **Channels:**
  - Melody — `OscillatorNode` with `"square"` waveform
  - Bass — `OscillatorNode` with `"triangle"` waveform
  - Percussion — Short burst noise via `OscillatorNode` or buffer noise
- **Playback:** Starts on first user interaction (browser autoplay policy), loops seamlessly by re-scheduling notes when the loop completes

### Dependencies

```toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "AudioContext", "OscillatorNode", "OscillatorType",
    "GainNode", "AudioDestinationNode", "AudioParam",
] }
```

### Browser Autoplay Handling

The `AudioContext` is created on first user keypress/click. This satisfies the browser's autoplay policy. Before interaction, the splash screen shows "~ Press any key to start ~".

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
- [ ] Create or source 8-bit intro music
- [ ] Integrate audio playback with browser autoplay handling
- [ ] Create `web/index.html` wrapper
- [ ] Test in browser via WASM
- [ ] Deploy to GitHub Pages

## Files to Create/Modify

| File | Purpose |
|------|---------|
| `Cargo.toml` | Project config, macroquad dependency |
| `src/main.rs` | Entry point, game loop, splash screen state |
| `src/splash.rs` | Splash screen rendering and animation logic |
| `src/audio.rs` | Audio loading and playback |
| `assets/audio/intro.ogg` | 8-bit intro song |
| `web/index.html` | HTML wrapper for WASM |

## Success Criteria

The splash screen is done when:
1. `cargo build --target wasm32-unknown-unknown` compiles cleanly
2. Opening `index.html` in a browser shows the splash screen
3. Pixel art warrior, monster, and treasure are visible with animations
4. 8-bit music plays after first user interaction
5. "Press any key" text pulses on screen
