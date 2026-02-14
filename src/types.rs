#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
}

pub const GRID_WIDTH: usize = 50;
pub const GRID_HEIGHT: usize = 37;
pub const TILE_SIZE: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn center(&self) -> Pos {
        Pos {
            x: self.x + self.w / 2,
            y: self.y + self.h / 2,
        }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }

    pub fn expand(&self, amount: i32) -> Rect {
        Rect {
            x: self.x - amount,
            y: self.y - amount,
            w: self.w + amount * 2,
            h: self.h + amount * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_center() {
        let r = Rect { x: 10, y: 20, w: 6, h: 4 };
        assert_eq!(r.center(), Pos { x: 13, y: 22 });
    }

    #[test]
    fn rect_intersects_overlapping() {
        let r1 = Rect { x: 0, y: 0, w: 10, h: 10 };
        let r2 = Rect { x: 5, y: 5, w: 10, h: 10 };
        assert!(r1.intersects(&r2));
    }

    #[test]
    fn rect_no_intersect_separated() {
        let r1 = Rect { x: 0, y: 0, w: 5, h: 5 };
        let r2 = Rect { x: 20, y: 20, w: 5, h: 5 };
        assert!(!r1.intersects(&r2));
    }

    #[test]
    fn rect_no_intersect_adjacent() {
        let r1 = Rect { x: 0, y: 0, w: 5, h: 5 };
        let r2 = Rect { x: 5, y: 0, w: 5, h: 5 };
        assert!(!r1.intersects(&r2));
    }

    #[test]
    fn rect_expand() {
        let r = Rect { x: 10, y: 10, w: 5, h: 5 };
        let e = r.expand(2);
        assert_eq!(e.x, 8);
        assert_eq!(e.y, 8);
        assert_eq!(e.w, 9);
        assert_eq!(e.h, 9);
    }

    #[test]
    fn rect_expand_creates_overlap() {
        let r1 = Rect { x: 0, y: 0, w: 5, h: 5 };
        let r2 = Rect { x: 6, y: 0, w: 5, h: 5 };
        assert!(!r1.intersects(&r2));
        assert!(r1.expand(2).intersects(&r2));
    }
}
