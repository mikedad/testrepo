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

## Audio: 8-Bit Intro Song

### Approach

Generate the intro music programmatically at build time or embed a small `.ogg` file. The song should feel like an NES/Game Boy title screen.

### Music Specs

- **Style:** 8-bit chiptune, heroic/adventurous feel
- **Tempo:** ~120 BPM
- **Length:** 15-30 second loop
- **Channels:** Melody (square wave), bass (triangle wave), percussion (noise)
- **Format:** `.ogg` (small file size, good browser support)
- **Playback:** Starts automatically, loops seamlessly

### Audio Note

Browsers block autoplay audio until a user interaction. The splash screen must handle this — either start audio on the first keypress/click, or show a "Click to start" prompt first.

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
