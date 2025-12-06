use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: i8,
    pub y: i8,
}
impl Vec2 {
    pub const fn new(x: i8, y: i8) -> Vec2 {
        Vec2 { x, y }
    }
}
impl Default for Vec2 {
    fn default() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}
impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
