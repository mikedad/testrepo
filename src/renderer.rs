use crate::dungeon::TileMap;
use crate::types::*;
use macroquad::prelude::*;

/// Simple hash for deterministic per-tile variation.
fn tile_hash(x: i32, y: i32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374761393)
        .wrapping_add((y as u32).wrapping_mul(668265263));
    h = (h ^ (h >> 15)).wrapping_mul(2246822519);
    h = (h ^ (h >> 13)).wrapping_mul(3266489917);
    h ^ (h >> 16)
}

fn tile_variant(x: i32, y: i32, variants: u32) -> u32 {
    tile_hash(x, y) % variants
}

/// Draw the entire dungeon map.
pub fn draw_dungeon(map: &TileMap) {
    clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

    for y in 0..GRID_HEIGHT as i32 {
        for x in 0..GRID_WIDTH as i32 {
            match map.get(x, y) {
                Some(Tile::Wall) => draw_wall(x, y),
                Some(Tile::Floor) => draw_floor(x, y, map),
                None => {}
            }
        }
    }
}

fn draw_wall(x: i32, y: i32) {
    let sx = x as f32 * TILE_SIZE;
    let sy = y as f32 * TILE_SIZE;
    let ts = TILE_SIZE;

    // Base dark stone
    let base = Color::new(0.227, 0.227, 0.29, 1.0); // ~#3a3a4a
    draw_rectangle(sx, sy, ts, ts, base);

    match tile_variant(x, y, 3) {
        0 => draw_wall_bricks_a(sx, sy, ts),
        1 => draw_wall_bricks_b(sx, sy, ts),
        _ => draw_wall_bricks_c(sx, sy, ts),
    }
}

/// Brick pattern A: 3 horizontal bands, centered vertical mortar
fn draw_wall_bricks_a(sx: f32, sy: f32, ts: f32) {
    let mortar = Color::new(0.157, 0.157, 0.22, 1.0); // #282838
    let light = Color::new(0.259, 0.259, 0.322, 1.0);  // #424252
    let dark = Color::new(0.196, 0.196, 0.259, 1.0);   // #323242

    let bh = ts / 3.0;
    // Top brick (lighter)
    draw_rectangle(sx, sy, ts, bh, light);
    // Mortar line
    draw_rectangle(sx, sy + bh - 0.5, ts, 1.0, mortar);
    // Middle brick
    draw_rectangle(sx, sy + bh, ts, bh, Color::new(0.227, 0.227, 0.29, 1.0));
    // Mortar line
    draw_rectangle(sx, sy + bh * 2.0 - 0.5, ts, 1.0, mortar);
    // Bottom brick (darker)
    draw_rectangle(sx, sy + bh * 2.0, ts, bh, dark);
    // Vertical mortar (offset)
    draw_rectangle(sx + ts * 0.5, sy, 1.0, bh, mortar);
    draw_rectangle(sx + ts * 0.25, sy + bh, 1.0, bh, mortar);
    draw_rectangle(sx + ts * 0.75, sy + bh * 2.0, 1.0, bh, mortar);
}

/// Brick pattern B: 2 large blocks with cross mortar
fn draw_wall_bricks_b(sx: f32, sy: f32, ts: f32) {
    let mortar = Color::new(0.157, 0.157, 0.22, 1.0);
    let shade1 = Color::new(0.243, 0.243, 0.306, 1.0); // #3e3e4e
    let shade2 = Color::new(0.212, 0.212, 0.275, 1.0); // #363646

    // Top-left block
    draw_rectangle(sx, sy, ts * 0.5, ts * 0.5, shade1);
    // Top-right block
    draw_rectangle(sx + ts * 0.5, sy, ts * 0.5, ts * 0.5, shade2);
    // Bottom-left block
    draw_rectangle(sx, sy + ts * 0.5, ts * 0.5, ts * 0.5, shade2);
    // Bottom-right block
    draw_rectangle(sx + ts * 0.5, sy + ts * 0.5, ts * 0.5, ts * 0.5, shade1);
    // Cross mortar
    draw_rectangle(sx + ts * 0.5 - 0.5, sy, 1.0, ts, mortar);
    draw_rectangle(sx, sy + ts * 0.5 - 0.5, ts, 1.0, mortar);
}

/// Brick pattern C: rough stone with cracks
fn draw_wall_bricks_c(sx: f32, sy: f32, ts: f32) {
    let mortar = Color::new(0.157, 0.157, 0.22, 1.0);
    let highlight = Color::new(0.275, 0.275, 0.337, 1.0); // #464656

    // Diagonal crack
    draw_rectangle(sx + 3.0, sy + 2.0, 1.0, 4.0, mortar);
    draw_rectangle(sx + 4.0, sy + 5.0, 1.0, 3.0, mortar);
    // Stone highlight
    draw_rectangle(sx + 7.0, sy + 2.0, 5.0, 3.0, highlight);
    draw_rectangle(sx + 2.0, sy + 9.0, 6.0, 4.0, highlight);
    // Corner shadow
    draw_rectangle(sx + ts - 2.0, sy + ts - 2.0, 2.0, 2.0, mortar);
}

fn draw_floor(x: i32, y: i32, map: &TileMap) {
    let sx = x as f32 * TILE_SIZE;
    let sy = y as f32 * TILE_SIZE;
    let ts = TILE_SIZE;

    // Base warm gray-brown
    let base = Color::new(0.416, 0.416, 0.353, 1.0); // ~#6a6a5a
    draw_rectangle(sx, sy, ts, ts, base);

    // Deterministic speckles
    let hash = tile_hash(x, y);
    for i in 0..8u32 {
        let sh = hash.wrapping_mul(i.wrapping_add(1).wrapping_mul(2654435761));
        let px = (sh % 14) as f32 + 1.0;
        let py = ((sh >> 8) % 14) as f32 + 1.0;

        let brightness = (sh >> 16) % 3;
        let color = match brightness {
            0 => Color::new(0.447, 0.447, 0.384, 1.0), // lighter
            1 => Color::new(0.384, 0.384, 0.322, 1.0), // darker
            _ => base,
        };
        draw_rectangle(sx + px, sy + py, 1.0, 1.0, color);
    }

    // Edge shadows next to walls
    let shadow = Color::new(0.29, 0.29, 0.227, 1.0); // #4a4a3a

    if map.get(x, y - 1) != Some(Tile::Floor) {
        draw_rectangle(sx, sy, ts, 2.0, shadow);
    }
    if map.get(x, y + 1) != Some(Tile::Floor) {
        draw_rectangle(sx, sy + ts - 2.0, ts, 2.0, shadow);
    }
    if map.get(x - 1, y) != Some(Tile::Floor) {
        draw_rectangle(sx, sy, 2.0, ts, shadow);
    }
    if map.get(x + 1, y) != Some(Tile::Floor) {
        draw_rectangle(sx + ts - 2.0, sy, 2.0, ts, shadow);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        assert_eq!(tile_hash(10, 20), tile_hash(10, 20));
    }

    #[test]
    fn hash_varies_by_position() {
        let h1 = tile_hash(10, 20);
        let h2 = tile_hash(11, 20);
        let h3 = tile_hash(10, 21);
        assert_ne!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[test]
    fn variant_in_range() {
        for x in 0..50 {
            for y in 0..37 {
                assert!(tile_variant(x, y, 3) < 3);
            }
        }
    }

    #[test]
    fn variant_distribution_roughly_even() {
        let mut counts = [0u32; 3];
        for x in 0..50 {
            for y in 0..37 {
                counts[tile_variant(x, y, 3) as usize] += 1;
            }
        }
        let total = 50 * 37;
        for count in counts {
            assert!(count > total / 5, "uneven distribution: {:?}", counts);
        }
    }
}
