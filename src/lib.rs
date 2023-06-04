pub type Position = (usize, usize);

pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: Vec<Position>,
    direction: Direction,
    food: Position,
}
