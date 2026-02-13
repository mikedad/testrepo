# Dungeon Creator

A real-time roguelike dungeon crawler built in Rust and WebAssembly. Play it in your browser — no install required.

## About

Dungeon Creator is a procedurally generated dungeon crawler where you explore randomly created levels, fight enemies, collect loot, and descend deeper into increasingly dangerous floors. Each run is unique. Death is permanent.

The game is built entirely in Rust, compiled to WebAssembly, and hosted for free on GitHub Pages.

## Gameplay

### Core Loop

1. Enter a procedurally generated dungeon floor
2. Explore rooms and corridors in real-time
3. Fight enemies using melee and ranged attacks
4. Collect items — weapons, potions, keys
5. Find the stairs to descend to the next floor
6. Repeat until you die or conquer the dungeon

### Controls

| Key | Action |
|-----|--------|
| W / Up Arrow | Move up |
| A / Left Arrow | Move left |
| S / Down Arrow | Move down |
| D / Right Arrow | Move right |
| Space | Attack |
| E | Use/pick up item |
| I | Open inventory |
| Esc | Pause menu |

### Features

- **Procedural Dungeon Generation** — Every run produces a unique layout of rooms, corridors, traps, and secrets
- **Real-Time Combat** — Enemies move and attack in real-time; timing and positioning matter
- **Permadeath** — One life per run; death sends you back to the beginning
- **Item System** — Weapons with varying damage/speed, health potions, armor, keys for locked doors
- **Enemy Variety** — Different enemy types with distinct behaviors (patrol, chase, ranged, boss)
- **Audio** — Background music that shifts with dungeon depth, sound effects for combat and exploration
- **Progressive Difficulty** — Deeper floors have tougher enemies, better loot, and more complex layouts

### Dungeon Generation

Floors are generated using a room-and-corridor algorithm:
- Random rectangular rooms placed without overlap
- Corridors connect rooms via nearest-neighbor paths
- Special rooms: treasure rooms, trap rooms, boss rooms
- Stairs down placed in the furthest room from the spawn point

### Enemy Types (Planned)

| Enemy | Behavior | First Appears |
|-------|----------|---------------|
| Rat | Wanders randomly, attacks if adjacent | Floor 1 |
| Skeleton | Patrols a set path, chases on sight | Floor 2 |
| Bat | Fast, erratic movement | Floor 3 |
| Archer | Ranged attacks, keeps distance | Floor 4 |
| Slime | Slow, splits into smaller slimes on death | Floor 5 |
| Boss | Unique per floor set, guards the stairs | Every 5th floor |

## Technical Details

- **Language:** Rust
- **Game Framework:** macroquad 0.4
- **Render Target:** WebAssembly (wasm32-unknown-unknown)
- **Hosting:** GitHub Pages
- **Graphics:** Tile-based 2D rendering with sprite sheets
- **Audio:** WebAudio via macroquad's audio module
- **Input:** Real-time keyboard handling via macroquad's input module

## Building

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown

# Build for web
cargo build --release --target wasm32-unknown-unknown

# The WASM binary will be at:
# target/wasm32-unknown-unknown/release/dungeon_of_rust.wasm
```

## Playing

Once deployed to GitHub Pages, just open the URL in any modern browser. No plugins or downloads needed.

## License

MIT
