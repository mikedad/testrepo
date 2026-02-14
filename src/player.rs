use crate::dungeon::TileMap;
use crate::types::*;
use macroquad::prelude::*;

const STEP_INTERVAL: f32 = 0.15; // seconds between tile steps

// Hero colors
const HELMET: Color = Color::new(0.7, 0.7, 0.75, 1.0);
const HELMET_DARK: Color = Color::new(0.5, 0.5, 0.55, 1.0);
const SKIN_COLOR: Color = Color::new(0.87, 0.72, 0.53, 1.0);
const ARMOR: Color = Color::new(0.6, 0.6, 0.65, 1.0);
const TUNIC: Color = Color::new(0.2, 0.35, 0.7, 1.0);
const PANTS: Color = Color::new(0.25, 0.25, 0.35, 1.0);
const BOOTS: Color = Color::new(0.4, 0.22, 0.08, 1.0);
const SWORD_BLADE: Color = Color::new(0.85, 0.85, 0.9, 1.0);
const SWORD_HILT: Color = Color::new(0.9, 0.75, 0.1, 1.0);

pub struct Player {
    pub pos: Pos,
    target: Option<Pos>,
    step_timer: f32,
    anim_frame: u8, // 0 or 1
    moving: bool,
}

impl Player {
    pub fn new(spawn: Pos) -> Self {
        Self {
            pos: spawn,
            target: None,
            step_timer: 0.0,
            anim_frame: 0,
            moving: false,
        }
    }

    pub fn set_target(&mut self, target: Pos) {
        self.target = Some(target);
    }

    pub fn update(&mut self, dt: f32, map: &TileMap) {
        let target = match self.target {
            Some(t) => t,
            None => {
                self.moving = false;
                return;
            }
        };

        // Already at target
        if self.pos == target {
            self.moving = false;
            self.target = None;
            return;
        }

        self.step_timer += dt;
        if self.step_timer < STEP_INTERVAL {
            return;
        }
        self.step_timer -= STEP_INTERVAL;

        // Greedy step: pick best adjacent floor tile
        if let Some(next) = best_step(self.pos, target, map) {
            self.pos = next;
            self.moving = true;
            self.anim_frame = 1 - self.anim_frame;
        } else {
            // Blocked, stop
            self.moving = false;
            self.target = None;
        }

        // Check if we arrived
        if self.pos == target {
            self.moving = false;
            self.target = None;
        }
    }

    pub fn draw(&self) {
        let sx = self.pos.x as f32 * TILE_SIZE;
        let sy = self.pos.y as f32 * TILE_SIZE;
        draw_hero(sx, sy, self.anim_frame, self.moving);
    }
}

/// Pick the adjacent floor tile (4-dir) closest to target.
/// Returns None if no neighbor gets closer than current position.
fn best_step(pos: Pos, target: Pos, map: &TileMap) -> Option<Pos> {
    let current_dist = manhattan(pos, target);
    let neighbors = [
        Pos { x: pos.x, y: pos.y - 1 },
        Pos { x: pos.x, y: pos.y + 1 },
        Pos { x: pos.x - 1, y: pos.y },
        Pos { x: pos.x + 1, y: pos.y },
    ];

    let mut best: Option<Pos> = None;
    let mut best_dist = current_dist;

    for &n in &neighbors {
        if map.get(n.x, n.y) != Some(Tile::Floor) {
            continue;
        }
        let d = manhattan(n, target);
        if d < best_dist {
            best_dist = d;
            best = Some(n);
        }
    }

    best
}

fn manhattan(a: Pos, b: Pos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// Draw a 16x16 pixel-art adventurer at screen position (sx, sy).
fn draw_hero(sx: f32, sy: f32, frame: u8, moving: bool) {
    // Helmet (top)
    draw_rectangle(sx + 4.0, sy, 8.0, 3.0, HELMET);
    draw_rectangle(sx + 5.0, sy - 1.0, 6.0, 1.0, HELMET_DARK); // helmet crest
    draw_rectangle(sx + 4.0, sy + 2.0, 8.0, 1.0, HELMET_DARK); // helmet brim

    // Face
    draw_rectangle(sx + 5.0, sy + 3.0, 6.0, 2.0, SKIN_COLOR);
    // Eyes
    draw_rectangle(sx + 6.0, sy + 3.0, 1.0, 1.0, Color::new(0.15, 0.15, 0.2, 1.0));
    draw_rectangle(sx + 9.0, sy + 3.0, 1.0, 1.0, Color::new(0.15, 0.15, 0.2, 1.0));

    // Body / armor
    draw_rectangle(sx + 4.0, sy + 5.0, 8.0, 5.0, ARMOR);
    // Tunic detail (center stripe)
    draw_rectangle(sx + 7.0, sy + 5.0, 2.0, 5.0, TUNIC);
    // Belt
    draw_rectangle(sx + 4.0, sy + 9.0, 8.0, 1.0, BOOTS);

    // Arms
    draw_rectangle(sx + 2.0, sy + 5.0, 2.0, 4.0, ARMOR);
    draw_rectangle(sx + 12.0, sy + 5.0, 2.0, 4.0, ARMOR);
    // Hands
    draw_rectangle(sx + 2.0, sy + 9.0, 2.0, 1.0, SKIN_COLOR);
    draw_rectangle(sx + 12.0, sy + 9.0, 2.0, 1.0, SKIN_COLOR);

    // Sword (right side)
    draw_rectangle(sx + 14.0, sy + 3.0, 1.0, 6.0, SWORD_BLADE);
    draw_rectangle(sx + 13.0, sy + 8.0, 3.0, 1.0, SWORD_HILT);

    // Legs (animated)
    if moving && frame == 1 {
        // Step right: left leg back, right leg forward
        draw_rectangle(sx + 5.0, sy + 10.0, 2.0, 3.0, PANTS);
        draw_rectangle(sx + 9.0, sy + 10.0, 2.0, 4.0, PANTS);
        draw_rectangle(sx + 5.0, sy + 13.0, 2.0, 2.0, BOOTS);
        draw_rectangle(sx + 9.0, sy + 14.0, 3.0, 2.0, BOOTS);
    } else {
        // Idle / step left: right leg back, left leg forward
        draw_rectangle(sx + 5.0, sy + 10.0, 2.0, 4.0, PANTS);
        draw_rectangle(sx + 9.0, sy + 10.0, 2.0, 3.0, PANTS);
        draw_rectangle(sx + 4.0, sy + 14.0, 3.0, 2.0, BOOTS);
        draw_rectangle(sx + 9.0, sy + 13.0, 2.0, 2.0, BOOTS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_map() -> TileMap {
        let mut map = TileMap::new();
        // Carve a 5x5 room at (10,10)
        map.carve_rect(Rect { x: 10, y: 10, w: 5, h: 5 });
        map
    }

    #[test]
    fn player_spawns_at_given_pos() {
        let p = Player::new(Pos { x: 12, y: 12 });
        assert_eq!(p.pos, Pos { x: 12, y: 12 });
    }

    #[test]
    fn player_moves_toward_target() {
        let map = make_test_map();
        let mut p = Player::new(Pos { x: 10, y: 12 });
        p.set_target(Pos { x: 14, y: 12 });
        // Step enough times to move
        for _ in 0..10 {
            p.update(STEP_INTERVAL + 0.01, &map);
        }
        assert!(p.pos.x > 10, "player should have moved right");
    }

    #[test]
    fn player_stops_at_wall() {
        let map = make_test_map();
        let mut p = Player::new(Pos { x: 10, y: 10 });
        // Target is outside the room (wall)
        p.set_target(Pos { x: 5, y: 10 });
        for _ in 0..20 {
            p.update(STEP_INTERVAL + 0.01, &map);
        }
        // Player should be at the edge of the room, not at (5,10)
        assert!(p.pos.x >= 10, "player should not walk through walls");
    }

    #[test]
    fn player_reaches_target_and_stops() {
        let map = make_test_map();
        let mut p = Player::new(Pos { x: 10, y: 12 });
        p.set_target(Pos { x: 12, y: 12 });
        for _ in 0..20 {
            p.update(STEP_INTERVAL + 0.01, &map);
        }
        assert_eq!(p.pos, Pos { x: 12, y: 12 });
        assert!(!p.moving);
    }

    #[test]
    fn best_step_picks_closer_tile() {
        let map = make_test_map();
        let step = best_step(Pos { x: 11, y: 12 }, Pos { x: 14, y: 12 }, &map);
        assert_eq!(step, Some(Pos { x: 12, y: 12 }));
    }

    #[test]
    fn best_step_returns_none_when_blocked() {
        let map = make_test_map();
        // At corner (10,10), trying to go to (5,5) — all directions away are walls
        // Left (9,10) = wall, up (10,9) = wall
        // Only right/down are floor but they go further from target
        // Actually (11,10) and (10,11) are floor but further from (5,5)
        // manhattan(10,10 -> 5,5) = 10
        // manhattan(11,10 -> 5,5) = 11 (worse)
        // manhattan(10,11 -> 5,5) = 11 (worse)
        let step = best_step(Pos { x: 10, y: 10 }, Pos { x: 5, y: 5 }, &map);
        assert_eq!(step, None);
    }

    #[test]
    fn manhattan_distance_correct() {
        assert_eq!(manhattan(Pos { x: 0, y: 0 }, Pos { x: 3, y: 4 }), 7);
        assert_eq!(manhattan(Pos { x: 5, y: 5 }, Pos { x: 5, y: 5 }), 0);
    }

    #[test]
    fn anim_frame_toggles_during_movement() {
        let map = make_test_map();
        let mut p = Player::new(Pos { x: 10, y: 12 });
        p.set_target(Pos { x: 14, y: 12 });
        let f0 = p.anim_frame;
        p.update(STEP_INTERVAL + 0.01, &map);
        let f1 = p.anim_frame;
        assert_ne!(f0, f1, "anim frame should toggle on step");
    }
}
