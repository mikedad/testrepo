# Spec 2: Dungeon Generation & Display

## Overview

After the splash screen art displays, a progress bar fills while a random dungeon is generated in the background. Once complete, a "Play!" button appears. Tapping it switches to a new screen displaying the dungeon map.

## Flow

1. Splash screen shows pixel art (warrior, monster, treasure) — no "Tap to continue" anymore
2. Progress bar appears: "Generating Level..." with 0–100% fill
3. Dungeon generates in the background while bar animates
4. Bar hits 100%, "Play!" button appears
5. Tap "Play!" → screen switches to dungeon view
6. Dungeon map fills the screen

## Tile Grid

- **Tile size:** 16x16 pixels
- **Grid:** 50 columns x 37 rows (800x592 pixels, fitting within the 800x600 window)
- **Tile types:** Wall, Floor
- Remaining 8px at bottom can hold the build timestamp

## Dungeon Generation Algorithm

### Rooms

- **Count:** 5–8 rooms per dungeon
- **Base shape:** Start with a random rectangle (4–10 tiles wide, 3–8 tiles tall)
- **Shape variation:** After placing the base rectangle, randomly mutate:
  - **Carve chunks:** Remove a rectangular section from one side (creates L-shapes, U-shapes)
  - **Add chunks:** Extend a rectangular section from one side (creates T-shapes, plus-shapes)
  - **Alcoves:** Small 2x2 or 2x3 extensions on random edges
- Each room must fit within the grid with at least 1 tile of wall padding from the grid edge
- Rooms must not overlap (minimum 2 tiles between any two rooms)

### Hallways

- **Width:** 1 tile (floor bordered by wall on both sides)
- **Connection:** Every room connects to at least one other room
- Use a simple approach:
  1. Sort rooms by position (e.g., by center x)
  2. Connect each room to the next in sequence
  3. Optionally add 1–2 extra connections for loops
- Hallway routing: L-shaped paths (horizontal then vertical, or vice versa — random choice)
- Hallways carve through walls, turning wall tiles into floor tiles

### Generation Steps (mapped to progress bar)

| Progress | Step |
|----------|------|
| 0–10% | Initialize grid (all walls) |
| 10–50% | Place rooms one by one |
| 50–70% | Mutate room shapes |
| 70–90% | Carve hallways between rooms |
| 90–100% | Final validation (ensure connectivity) |

The generation runs across multiple frames so the progress bar animates smoothly. Each step yields back to the game loop to update the display.

## Rendering Style — Old School Pixel Art

### Wall Tiles (16x16)

Dark stone/brick appearance drawn with macroquad primitives:
- **Base color:** Dark gray-brown (#3a3a4a)
- **Brick pattern:** 2–3 horizontal bands of slightly different shades
- **Mortar lines:** 1px lines in darker shade between bricks
- **Variation:** Each wall tile picks from 2–3 pre-defined brick patterns (based on tile position hash) so walls don't look uniform

### Floor Tiles (16x16)

Lighter stone/dirt appearance:
- **Base color:** Warm gray-brown (#6a6a5a)
- **Texture:** Subtle speckles — a few pixels in slightly lighter/darker shades
- **Variation:** Each floor tile has a slightly different speckle pattern (based on position hash)
- **Edge detail:** Floor tiles adjacent to walls get a subtle shadow on the wall side (1–2px darker strip)

### Overall Aesthetic

- Classic dungeon crawler look (Rogue/Nethack era but with pixel art instead of ASCII)
- Dark, moody atmosphere
- No smoothing or anti-aliasing — crisp pixel edges
- The dungeon should look like carved stone corridors connecting rough-hewn chambers

## Progress Bar

- **Position:** Centered horizontally, below the splash art characters
- **Size:** ~400px wide, 24px tall
- **Style:** Dark border, fills left-to-right with a warm color (gold/amber)
- **Label:** "Generating Level..." text centered above the bar
- **Percentage:** "XX%" text centered inside the bar

## Play Button

- Appears after progress bar reaches 100%
- **Position:** Replaces the progress bar area
- **Style:** Large, prominent pixel-art button
  - Dark border, lighter interior
  - "PLAY!" text centered, bold
  - Subtle pulse/glow animation to draw attention
- **Interaction:** Tap/click triggers screen transition

## Screen Transition

- Simple instant cut from splash to dungeon view (no fancy transition needed)
- Dungeon view shows the full tile map filling the screen
- Build timestamp remains visible (bottom area, 8px strip)

## Implementation Checklist

- [ ] Add `GameState` enum: `Splash`, `Playing`
- [ ] Add `DungeonGenerator` struct with step-by-step generation across frames
- [ ] Implement room placement (random rectangles, non-overlapping)
- [ ] Implement room mutation (carve/add chunks for varied shapes)
- [ ] Implement hallway carving (L-shaped paths connecting rooms)
- [ ] Implement connectivity validation
- [ ] Add progress bar rendering on splash screen
- [ ] Add "Play!" button rendering and tap detection
- [ ] Implement `TileMap` struct (50x37 grid of Wall/Floor)
- [ ] Implement wall tile rendering (brick pattern with variation)
- [ ] Implement floor tile rendering (speckled stone with edge shadows)
- [ ] Add screen transition from Splash to Playing
- [ ] Remove "Tap to continue" from splash screen
- [ ] Keep build timestamp on both screens
- [ ] Write unit tests for generation (room placement, connectivity, bounds)
- [ ] Compile and test locally
- [ ] Build for WASM and deploy

## Files to Create/Modify

| File | Purpose |
|------|---------|
| `src/main.rs` | Add GameState enum, orchestrate splash → playing transition |
| `src/splash.rs` | Add progress bar, "Play!" button, remove "Tap to continue" |
| `src/dungeon.rs` | DungeonGenerator, TileMap, room/hallway algorithms |
| `src/renderer.rs` | Tile rendering (wall/floor pixel art patterns) |
| `src/types.rs` | Shared types: Tile enum, grid constants |

## Success Criteria

1. Progress bar fills 0–100% while dungeon generates
2. "Play!" button appears after generation
3. Tapping "Play!" shows the dungeon map
4. Dungeon has 5–8 rooms of varying shapes
5. Rooms connected by 1-tile-wide hallways
6. No unreachable areas
7. Wall and floor tiles have distinct pixel-art textures
8. Looks like a classic dungeon crawler map
9. Build timestamp visible on both screens
10. Works on iPad via touch
