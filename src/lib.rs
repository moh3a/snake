pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: Vec<Position>,
    direction: Direction,
    food: Position,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: vec![((width - 2).max(0), height / 2)],
            direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Top, Direction::Top)
            | (Direction::Top, Direction::Bottom)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Bottom, Direction::Top)
            | (Direction::Bottom, Direction::Bottom)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {}
            (_, direction) => self.direction = direction,
        }
    }

    pub fn tick(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10));
    }
}
