# Spec 3: Move Hero

## Overview

Render a 16x16 pixel-art adventurer on the dungeon map. The player taps/touches the screen and the hero continuously walks toward that point, tile by tile, avoiding walls.

## Hero Sprite (16x16)

A cool-looking scaled-down adventurer drawn with macroquad primitives within a single 16x16 tile:

- **Helmet:** 2-3px tall, silver/gray, pointed or flat top
- **Head:** 2px tall, skin color
- **Body/armor:** 4-5px tall, silver with a colored tunic/detail
- **Arms:** 1px wide on each side of body
- **Legs:** 3px tall, dark pants/boots — animated
- **Sword:** 1px wide accent on one side (optional, if it fits)

### Walking Animation

Simple 2-frame animation that alternates legs:
- **Frame 0 (idle/step-left):** Left leg forward, right leg back
- **Frame 1 (step-right):** Right leg forward, left leg back
- Frames alternate each tile step during movement
- When idle (not moving), show frame 0

## Spawning

- Find the floor tile closest to the center of the map (GRID_WIDTH/2, GRID_HEIGHT/2)
- If the center tile is a wall, search outward in a spiral or check room centers
- Hero occupies exactly one tile

## Movement

### Input
- **Touch/tap:** Touch anywhere on the screen sets the target tile
- **Mouse click:** Same behavior (desktop support)
- Target tile = touched screen position converted to grid coordinates (divide by TILE_SIZE)
- New tap overrides the previous target immediately

### Walking Behavior
- Hero walks toward the target tile, **one tile per step**
- Step interval: ~150ms (adjustable constant)
- Each step, pick the adjacent tile (up/down/left/right — 4 directions, no diagonal) that:
  1. Is a floor tile (not a wall)
  2. Is closest to the target (Manhattan distance or Euclidean)
- If no adjacent floor tile gets closer to the target, stop (blocked)
- If hero reaches the target tile, stop
- Movement looks continuous because of the steady step interval

### Pathfinding
Simple greedy approach (no A* needed for now):
- Each step, evaluate 4 neighbors
- Pick the one that minimizes distance to target AND is a floor tile
- Break ties by preferring the axis with the larger delta
- This won't navigate around complex obstacles perfectly, but works well for open dungeons

## Rendering

- Hero is drawn ON TOP of the floor tile at their current grid position
- Screen position: `(hero.x * TILE_SIZE, hero.y * TILE_SIZE)`
- The hero tile replaces nothing — floor is drawn first, hero on top
- No camera movement — map fills the screen, hero moves within it

## Implementation Checklist

- [ ] Create `src/player.rs` with `Player` struct (grid position, target, step timer, anim frame)
- [ ] Draw 16x16 adventurer sprite with macroquad primitives
- [ ] Implement 2-frame walking animation
- [ ] Find spawn point near map center
- [ ] Convert touch/click screen position to grid coordinates
- [ ] Implement greedy step-toward-target movement
- [ ] Integrate into `GameState::Playing` in main.rs
- [ ] Draw hero on top of dungeon tiles
- [ ] Unit tests (spawn on floor, movement stays on floor, blocked by walls)
- [ ] Compile and test locally
- [ ] Build for WASM and deploy

## Files to Create/Modify

| File | Purpose |
|------|---------|
| `src/player.rs` | **Create** — Player struct, sprite drawing, movement logic |
| `src/main.rs` | **Modify** — Add Player to Playing state, handle touch input, update/draw player |
| `src/dungeon.rs` | **Modify** — Add `find_spawn_point()` helper |

## Success Criteria

1. Hero appears on a floor tile near the center of the dungeon
2. Tapping the screen makes the hero walk toward that point
3. Hero walks tile-by-tile at a steady pace (~150ms per step)
4. Hero cannot walk through walls
5. Legs animate while walking
6. New tap overrides previous destination
7. Works on iPad via touch
