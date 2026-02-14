use crate::types::*;
use macroquad::rand::gen_range;
use std::collections::HashSet;

/// The tile map: a 2D grid of Wall/Floor tiles.
pub struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::Wall; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Tile> {
        if x >= 0 && y >= 0 && (x as usize) < GRID_WIDTH && (y as usize) < GRID_HEIGHT {
            Some(self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
        if x >= 0 && y >= 0 && (x as usize) < GRID_WIDTH && (y as usize) < GRID_HEIGHT {
            self.tiles[y as usize][x as usize] = tile;
        }
    }

    pub fn carve_rect(&mut self, rect: Rect) {
        for y in rect.y..rect.y + rect.h {
            for x in rect.x..rect.x + rect.w {
                self.set(x, y, Tile::Floor);
            }
        }
    }

    /// Find the floor tile closest to the map center.
    pub fn find_spawn_point(&self) -> Pos {
        let cx = GRID_WIDTH as i32 / 2;
        let cy = GRID_HEIGHT as i32 / 2;

        // Search outward in expanding rings
        for radius in 0..((GRID_WIDTH + GRID_HEIGHT) as i32) {
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx.abs() + dy.abs() != radius {
                        continue; // Only check the ring perimeter
                    }
                    let x = cx + dx;
                    let y = cy + dy;
                    if self.get(x, y) == Some(Tile::Floor) {
                        return Pos { x, y };
                    }
                }
            }
        }

        // Fallback: first floor tile found
        for y in 0..GRID_HEIGHT as i32 {
            for x in 0..GRID_WIDTH as i32 {
                if self.get(x, y) == Some(Tile::Floor) {
                    return Pos { x, y };
                }
            }
        }

        Pos { x: cx, y: cy }
    }

    fn carve_h_line(&mut self, x1: i32, x2: i32, y: i32) {
        let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start..=end {
            self.set(x, y, Tile::Floor);
        }
    }

    fn carve_v_line(&mut self, y1: i32, y2: i32, x: i32) {
        let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start..=end {
            self.set(x, y, Tile::Floor);
        }
    }
}

/// A room: base rect plus optional mutation chunks.
#[derive(Debug, Clone)]
pub struct Room {
    pub base: Rect,
    pub chunks: Vec<Rect>,
}

impl Room {
    fn new(base: Rect) -> Self {
        Self { base, chunks: Vec::new() }
    }

    /// Bounding box encompassing base + all chunks.
    pub fn bounds(&self) -> Rect {
        let mut min_x = self.base.x;
        let mut min_y = self.base.y;
        let mut max_x = self.base.x + self.base.w;
        let mut max_y = self.base.y + self.base.h;
        for c in &self.chunks {
            min_x = min_x.min(c.x);
            min_y = min_y.min(c.y);
            max_x = max_x.max(c.x + c.w);
            max_y = max_y.max(c.y + c.h);
        }
        Rect { x: min_x, y: min_y, w: max_x - min_x, h: max_y - min_y }
    }

    fn center(&self) -> Pos {
        self.base.center()
    }
}

/// Generation step for the state machine.
#[derive(Debug)]
enum GenStep {
    Init,
    PlacingRooms { placed: usize, target: usize },
    MutatingRooms { mutated: usize },
    SortAndPrepHallways,
    CarvingHallways { carved: usize, total: usize },
    Validating,
    Complete,
}

pub struct DungeonGenerator {
    pub map: TileMap,
    pub rooms: Vec<Room>,
    step: GenStep,
}

impl DungeonGenerator {
    pub fn new() -> Self {
        Self {
            map: TileMap::new(),
            rooms: Vec::new(),
            step: GenStep::Init,
        }
    }

    /// Advance one step. Returns true if more steps are needed.
    pub fn step_generation(&mut self) -> bool {
        match self.step {
            GenStep::Init => {
                let target = gen_range(5, 9); // 5..=8
                self.step = GenStep::PlacingRooms { placed: 0, target };
                true
            }
            GenStep::PlacingRooms { placed, target } => {
                if placed < target {
                    self.try_place_room();
                    self.step = GenStep::PlacingRooms { placed: placed + 1, target };
                } else {
                    self.step = GenStep::MutatingRooms { mutated: 0 };
                }
                true
            }
            GenStep::MutatingRooms { mutated } => {
                if mutated < self.rooms.len() {
                    self.mutate_room(mutated);
                    self.step = GenStep::MutatingRooms { mutated: mutated + 1 };
                } else {
                    self.step = GenStep::SortAndPrepHallways;
                }
                true
            }
            GenStep::SortAndPrepHallways => {
                self.rooms.sort_by_key(|r| r.center().x);
                let base_hallways = if self.rooms.len() > 1 { self.rooms.len() - 1 } else { 0 };
                let extras = gen_range(1, 3); // 1-2 extra
                let total = base_hallways + extras;
                self.step = GenStep::CarvingHallways { carved: 0, total };
                true
            }
            GenStep::CarvingHallways { carved, total } => {
                if carved < total {
                    self.carve_hallway(carved);
                    self.step = GenStep::CarvingHallways { carved: carved + 1, total };
                } else {
                    self.step = GenStep::Validating;
                }
                true
            }
            GenStep::Validating => {
                self.ensure_connectivity();
                self.step = GenStep::Complete;
                false
            }
            GenStep::Complete => false,
        }
    }

    pub fn progress(&self) -> f32 {
        match &self.step {
            GenStep::Init => 0.0,
            GenStep::PlacingRooms { placed, target } => {
                0.1 + (*placed as f32 / *target as f32) * 0.4
            }
            GenStep::MutatingRooms { mutated } => {
                let total = self.rooms.len().max(1) as f32;
                0.5 + (*mutated as f32 / total) * 0.2
            }
            GenStep::SortAndPrepHallways => 0.7,
            GenStep::CarvingHallways { carved, total } => {
                let t = (*total).max(1) as f32;
                0.7 + (*carved as f32 / t) * 0.2
            }
            GenStep::Validating => 0.95,
            GenStep::Complete => 1.0,
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.step, GenStep::Complete)
    }

    fn try_place_room(&mut self) {
        for _ in 0..30 {
            let w = gen_range(4, 11); // 4..=10
            let h = gen_range(3, 9);  // 3..=8
            let x = gen_range(1, GRID_WIDTH as i32 - w - 1);
            let y = gen_range(1, GRID_HEIGHT as i32 - h - 1);
            let rect = Rect { x, y, w, h };

            // Check overlap with existing rooms (2-tile padding)
            let expanded = rect.expand(2);
            let overlaps = self.rooms.iter().any(|r| expanded.intersects(&r.bounds()));
            if overlaps {
                continue;
            }

            self.map.carve_rect(rect);
            self.rooms.push(Room::new(rect));
            return;
        }
    }

    fn mutate_room(&mut self, idx: usize) {
        let mutation_count = gen_range(0, 3); // 0-2 mutations
        for _ in 0..mutation_count {
            let mutation_type = gen_range(0, 3);
            match mutation_type {
                0 => self.carve_chunk(idx),
                1 => self.add_chunk(idx),
                _ => self.add_alcove(idx),
            }
        }
    }

    fn carve_chunk(&mut self, idx: usize) {
        let base = self.rooms[idx].base;
        let side = gen_range(0, 4);
        let chunk = match side {
            0 => { // North: remove top portion
                let ch = gen_range(1, (base.h / 2).max(2));
                let cw = gen_range(2, (base.w - 1).max(3));
                let cx = base.x + gen_range(0, (base.w - cw).max(1));
                Rect { x: cx, y: base.y, w: cw, h: ch }
            }
            1 => { // South
                let ch = gen_range(1, (base.h / 2).max(2));
                let cw = gen_range(2, (base.w - 1).max(3));
                let cx = base.x + gen_range(0, (base.w - cw).max(1));
                Rect { x: cx, y: base.y + base.h - ch, w: cw, h: ch }
            }
            2 => { // West
                let cw = gen_range(1, (base.w / 2).max(2));
                let ch = gen_range(2, (base.h - 1).max(3));
                let cy = base.y + gen_range(0, (base.h - ch).max(1));
                Rect { x: base.x, y: cy, w: cw, h: ch }
            }
            _ => { // East
                let cw = gen_range(1, (base.w / 2).max(2));
                let ch = gen_range(2, (base.h - 1).max(3));
                let cy = base.y + gen_range(0, (base.h - ch).max(1));
                Rect { x: base.x + base.w - cw, y: cy, w: cw, h: ch }
            }
        };

        // Set carved tiles back to wall
        for y in chunk.y..chunk.y + chunk.h {
            for x in chunk.x..chunk.x + chunk.w {
                self.map.set(x, y, Tile::Wall);
            }
        }
    }

    fn add_chunk(&mut self, idx: usize) {
        let base = self.rooms[idx].base;
        let side = gen_range(0, 4);
        let chunk = match side {
            0 => { // North extension
                let cw = gen_range(2, (base.w / 2 + 1).max(3));
                let ch = gen_range(2, 4);
                let cx = base.x + gen_range(0, (base.w - cw).max(1));
                Rect { x: cx, y: base.y - ch, w: cw, h: ch }
            }
            1 => { // South extension
                let cw = gen_range(2, (base.w / 2 + 1).max(3));
                let ch = gen_range(2, 4);
                let cx = base.x + gen_range(0, (base.w - cw).max(1));
                Rect { x: cx, y: base.y + base.h, w: cw, h: ch }
            }
            2 => { // West extension
                let cw = gen_range(2, 4);
                let ch = gen_range(2, (base.h / 2 + 1).max(3));
                let cy = base.y + gen_range(0, (base.h - ch).max(1));
                Rect { x: base.x - cw, y: cy, w: cw, h: ch }
            }
            _ => { // East extension
                let cw = gen_range(2, 4);
                let ch = gen_range(2, (base.h / 2 + 1).max(3));
                let cy = base.y + gen_range(0, (base.h - ch).max(1));
                Rect { x: base.x + base.w, y: cy, w: cw, h: ch }
            }
        };

        // Bounds check
        if chunk.x < 1 || chunk.y < 1
            || chunk.x + chunk.w >= GRID_WIDTH as i32 - 1
            || chunk.y + chunk.h >= GRID_HEIGHT as i32 - 1
        {
            return;
        }

        // Collision check against other rooms
        let expanded = chunk.expand(1);
        let collides = self.rooms.iter().enumerate().any(|(i, r)| {
            i != idx && expanded.intersects(&r.bounds())
        });
        if collides {
            return;
        }

        self.map.carve_rect(chunk);
        self.rooms[idx].chunks.push(chunk);
    }

    fn add_alcove(&mut self, idx: usize) {
        let base = self.rooms[idx].base;
        let side = gen_range(0, 4);
        let alcove = match side {
            0 => { // North
                let cx = base.x + gen_range(0, (base.w - 2).max(1));
                Rect { x: cx, y: base.y - 2, w: 2, h: 2 }
            }
            1 => { // South
                let cx = base.x + gen_range(0, (base.w - 2).max(1));
                Rect { x: cx, y: base.y + base.h, w: 2, h: 2 }
            }
            2 => { // West
                let cy = base.y + gen_range(0, (base.h - 2).max(1));
                Rect { x: base.x - 2, y: cy, w: 2, h: 2 }
            }
            _ => { // East
                let cy = base.y + gen_range(0, (base.h - 2).max(1));
                Rect { x: base.x + base.w, y: cy, w: 2, h: 2 }
            }
        };

        if alcove.x < 1 || alcove.y < 1
            || alcove.x + alcove.w >= GRID_WIDTH as i32 - 1
            || alcove.y + alcove.h >= GRID_HEIGHT as i32 - 1
        {
            return;
        }

        let expanded = alcove.expand(1);
        let collides = self.rooms.iter().enumerate().any(|(i, r)| {
            i != idx && expanded.intersects(&r.bounds())
        });
        if collides {
            return;
        }

        self.map.carve_rect(alcove);
        self.rooms[idx].chunks.push(alcove);
    }

    fn carve_hallway(&mut self, index: usize) {
        if self.rooms.len() < 2 {
            return;
        }

        let (from_idx, to_idx) = if index < self.rooms.len() - 1 {
            // Sequential connection
            (index, index + 1)
        } else {
            // Extra random connection
            let a = gen_range(0, self.rooms.len() as i32) as usize;
            let mut b = gen_range(0, self.rooms.len() as i32) as usize;
            if b == a {
                b = (a + 1) % self.rooms.len();
            }
            (a, b)
        };

        let c1 = self.rooms[from_idx].center();
        let c2 = self.rooms[to_idx].center();

        // L-shaped path: random choice of horizontal-first or vertical-first
        if gen_range(0, 2) == 0 {
            self.map.carve_h_line(c1.x, c2.x, c1.y);
            self.map.carve_v_line(c1.y, c2.y, c2.x);
        } else {
            self.map.carve_v_line(c1.y, c2.y, c1.x);
            self.map.carve_h_line(c1.x, c2.x, c2.y);
        }
    }

    fn ensure_connectivity(&mut self) {
        if self.rooms.is_empty() {
            return;
        }

        let start = self.rooms[0].center();
        let reachable = flood_fill(&self.map, start);

        // Check if all rooms are reachable
        for i in 1..self.rooms.len() {
            let center = self.rooms[i].center();
            if !reachable.contains(&center) {
                // Connect this room to room 0 directly
                let c0 = self.rooms[0].center();
                self.map.carve_h_line(c0.x, center.x, c0.y);
                self.map.carve_v_line(c0.y, center.y, center.x);
            }
        }
    }
}

fn flood_fill(map: &TileMap, start: Pos) -> HashSet<Pos> {
    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        if map.get(pos.x, pos.y) != Some(Tile::Floor) {
            continue;
        }
        visited.insert(pos);
        stack.push(Pos { x: pos.x + 1, y: pos.y });
        stack.push(Pos { x: pos.x - 1, y: pos.y });
        stack.push(Pos { x: pos.x, y: pos.y + 1 });
        stack.push(Pos { x: pos.x, y: pos.y - 1 });
    }

    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_generator(seed: u64) -> DungeonGenerator {
        macroquad::rand::srand(seed);
        let mut gen = DungeonGenerator::new();
        while gen.step_generation() {}
        gen
    }

    #[test]
    fn tilemap_starts_all_walls() {
        let map = TileMap::new();
        assert_eq!(map.get(0, 0), Some(Tile::Wall));
        assert_eq!(map.get(25, 18), Some(Tile::Wall));
        assert_eq!(map.get(49, 36), Some(Tile::Wall));
    }

    #[test]
    fn tilemap_bounds_checking() {
        let map = TileMap::new();
        assert!(map.get(0, 0).is_some());
        assert!(map.get(49, 36).is_some());
        assert!(map.get(-1, 0).is_none());
        assert!(map.get(50, 0).is_none());
        assert!(map.get(0, 37).is_none());
    }

    #[test]
    fn carve_rect_sets_floor() {
        let mut map = TileMap::new();
        map.carve_rect(Rect { x: 10, y: 10, w: 5, h: 5 });
        assert_eq!(map.get(10, 10), Some(Tile::Floor));
        assert_eq!(map.get(14, 14), Some(Tile::Floor));
        assert_eq!(map.get(9, 10), Some(Tile::Wall));
        assert_eq!(map.get(15, 10), Some(Tile::Wall));
    }

    #[test]
    fn generator_produces_5_to_8_rooms() {
        for seed in [100, 200, 300, 400, 500] {
            let gen = run_generator(seed);
            assert!(
                gen.rooms.len() >= 5 && gen.rooms.len() <= 8,
                "seed {} produced {} rooms",
                seed,
                gen.rooms.len()
            );
        }
    }

    #[test]
    fn rooms_within_grid_bounds() {
        let gen = run_generator(12345);
        for (i, room) in gen.rooms.iter().enumerate() {
            let b = room.bounds();
            assert!(b.x >= 0, "room {} x={} out of bounds", i, b.x);
            assert!(b.y >= 0, "room {} y={} out of bounds", i, b.y);
            assert!(
                b.x + b.w <= GRID_WIDTH as i32,
                "room {} right edge {} exceeds grid",
                i,
                b.x + b.w
            );
            assert!(
                b.y + b.h <= GRID_HEIGHT as i32,
                "room {} bottom edge {} exceeds grid",
                i,
                b.y + b.h
            );
        }
    }

    #[test]
    fn all_rooms_connected() {
        for seed in [111, 222, 333, 444, 555] {
            let gen = run_generator(seed);
            if gen.rooms.is_empty() {
                continue;
            }
            let start = gen.rooms[0].center();
            let reachable = flood_fill(&gen.map, start);

            for (i, room) in gen.rooms.iter().enumerate() {
                let c = room.center();
                assert!(
                    reachable.contains(&c),
                    "seed {}: room {} center {:?} not reachable",
                    seed,
                    i,
                    c
                );
            }
        }
    }

    #[test]
    fn progress_increases_monotonically() {
        macroquad::rand::srand(7777);
        let mut gen = DungeonGenerator::new();
        let mut prev = 0.0f32;
        loop {
            let p = gen.progress();
            assert!(p >= prev, "progress decreased: {} -> {}", prev, p);
            prev = p;
            if !gen.step_generation() {
                break;
            }
        }
        assert_eq!(gen.progress(), 1.0);
    }

    #[test]
    fn dungeon_has_floor_tiles() {
        let gen = run_generator(9999);
        let floor_count = (0..GRID_HEIGHT)
            .flat_map(|y| (0..GRID_WIDTH).map(move |x| (x, y)))
            .filter(|&(x, y)| gen.map.get(x as i32, y as i32) == Some(Tile::Floor))
            .count();
        assert!(floor_count > 50, "only {} floor tiles", floor_count);
    }
}
