# Project: Dungeon Creator — Rust WASM Roguelike Dungeon Crawler

## Overview

A real-time roguelike dungeon crawler built in Rust, compiled to WebAssembly, and hosted on GitHub Pages. The game runs entirely in the browser with real-time graphics, keyboard controls, and audio.

## Tech Stack

### Language & Compilation Target

- **Rust** — Primary language
- **WebAssembly (wasm32-unknown-unknown)** — Compilation target for browser execution

### Game Library

- **macroquad** — Lightweight 2D game framework with first-class WASM support
  - Provides: rendering, input handling, audio, game loop
  - Minimal boilerplate and small WASM binary size
  - Built-in JS/HTML loader for WASM deployment
  - Crate: `macroquad = "0.4"`

### Key macroquad Modules

- **macroquad::prelude** — Core types, math, colors, drawing primitives
- **macroquad::audio** — Sound effects and music via WebAudio backend
- **macroquad::texture** — Sprite/tileset loading and rendering
- **macroquad::input** — Real-time keyboard and mouse input (no frame delay)
- **macroquad::rand** — Random number generation for procedural content

### Build & Deploy Tools

- **cargo** — Rust package manager and build system
- **rustup target add wasm32-unknown-unknown** — WASM compilation target
- **macroquad WASM loader** — Minimal JS/HTML wrapper provided by macroquad for browser embedding
- **GitHub Pages** — Free static hosting from the `gh-pages` branch

### Optional/Future Dependencies

- **serde + serde_json** — Save/load game state (serialization)
- **noise** — Procedural noise for dungeon generation algorithms

## Build Commands

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Build for WASM
cargo build --release --target wasm32-unknown-unknown

# Output will be at: target/wasm32-unknown-unknown/release/<name>.wasm
```

## Project Structure (Planned)

```
finn/
├── CLAUDE.md            # This file — project conventions and tools
├── README.md            # Project overview and game design
├── Cargo.toml           # Rust dependencies
├── src/
│   ├── main.rs          # Entry point and game loop
│   ├── player.rs        # Player state, movement, combat
│   ├── dungeon.rs       # Procedural dungeon generation
│   ├── enemies.rs       # Enemy types, AI, behavior
│   ├── items.rs         # Loot, potions, weapons
│   ├── audio.rs         # Sound effects and music management
│   ├── renderer.rs      # Tile rendering, camera, UI overlay
│   └── types.rs         # Shared types and constants
├── assets/
│   ├── sprites/         # Tilesets and character sprites
│   └── audio/           # Sound effects and background music
└── web/
    ├── index.html       # HTML wrapper for WASM
    └── mq_js_bundle.js  # macroquad JS loader
```

## Development Process: Spec-Driven Development

This project follows **spec-driven development**. All work flows through natural language conversations with Claude to produce numbered spec files before any code is written.

### CRITICAL: Follow the process. No exceptions.

**Never skip steps. Never write code before the spec is discussed and agreed upon.** If a problem is discovered (e.g. a deployment issue, a broken build), the response is to talk it through first, update the spec, then code the fix. Jumping straight to code — even for "quick fixes" — violates the process.

### Workflow

1. **Discuss** — Talk through the feature or problem in natural language. Ask questions. Explore options.
2. **Spec** — Create or update a numbered spec file (e.g. `1-splash-screen.md`, `2-dungeon-gen.md`). The spec is the source of truth.
3. **Code** — Implement from the spec. All coding follows directly from what the spec describes.
4. **Unit Tests** — Write tests for the new code.
5. **Compile & Test Locally** — `cargo build` and `cargo test` must both pass.
6. **Build for Web** — `cargo build --release --target wasm32-unknown-unknown`, copy WASM to `web/`, verify in browser.

### Spec Files

- Numbered sequentially: `1-splash-screen.md`, `2-dungeon-gen.md`, etc.
- Each spec captures all details for that feature — requirements, design, architecture, checklist.
- Specs are cumulative — later specs build on earlier ones. Together they form the full project history.
- No code is written until the spec is discussed and agreed upon.

## Conventions

- **Be concise but complete** — Code, comments, and documentation should be brief and to the point while covering everything necessary. No fluff, no gaps.
- **No JavaScript. No exceptions.** — This is a pure WASM project. All logic lives in Rust. The only JS allowed is the minimal macroquad loader (`mq_js_bundle.js`) which is a third-party dependency. Never write custom JavaScript. Never use `wasm-bindgen`, `web-sys`, or `js-sys` as dependencies — directly or transitively. Any crate that depends on these (e.g. `cpal`) is incompatible with macroquad's WASM loader and must not be used. All proposed solutions must work with macroquad's pure WASM pipeline.
- **iPad compatible** — The game must work on iPad via touch input. No keyboard required for core interactions. Use touch/tap instead of key presses where possible.
- Use macroquad's built-in game loop (`#[macroquad::main]` attribute)
- Tile-based rendering — all game objects align to a grid
- Real-time gameplay (not turn-based) with smooth movement
- Procedural dungeon generation each run
- Keep WASM binary size small — avoid heavy dependencies
