pub(crate) struct Food;
pub(crate) struct SnakeHead {
    pub direction: Direction,
}
pub(crate) struct SnakeSegment;
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Position {
    pub x: i32,
    pub y: i32,
}
pub(crate) struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
#[derive(PartialEq, Copy, Clone)]
pub(crate) enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
