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

### Workflow

1. **Spec** — Create a numbered spec file (e.g. `1-splash-screen.md`, `2-dungeon-gen.md`). Discuss requirements in natural language until the spec is complete. The spec is the source of truth.
2. **Code** — Implement from the spec. All coding follows directly from what the spec describes.
3. **Unit Tests** — Write tests for the new code.
4. **Compile & Test Locally** — `cargo build` and `cargo test` must both pass.
5. **Build for Web** — `cargo build --release --target wasm32-unknown-unknown`, copy WASM to `web/`, verify in browser.

### Spec Files

- Numbered sequentially: `1-splash-screen.md`, `2-dungeon-gen.md`, etc.
- Each spec captures all details for that feature — requirements, design, architecture, checklist.
- Specs are cumulative — later specs build on earlier ones. Together they form the full project history.
- No code is written until the spec is agreed upon.

## Conventions

- **Be concise but complete** — Code, comments, and documentation should be brief and to the point while covering everything necessary. No fluff, no gaps.
- Use macroquad's built-in game loop (`#[macroquad::main]` attribute)
- Tile-based rendering — all game objects align to a grid
- Real-time gameplay (not turn-based) with smooth movement
- Procedural dungeon generation each run
- Keep WASM binary size small — avoid heavy dependencies
