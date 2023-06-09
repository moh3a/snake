use crate::random::random_range;
use std::collections::VecDeque;

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
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Position>,
    pub direction: Direction,
    pub food: Position,
    pub finished: bool,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            finished: false,
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

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.len() == 0 {
            return;
        }

        // move snake
        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_position = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .filter(|position| !self.snake.contains(position))
                    .collect::<Vec<_>>();

                if free_position.is_empty() {
                    self.finished = true;
                    return;
                }

                self.food = free_position[random_range(0, free_position.len())];
            }
            self.snake.push_front(new_head);
        }
    }
}
