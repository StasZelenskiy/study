use wasm_bindgen::prelude::*;
use direction::Direction;
use snake::{Snake, SnakeCell};
use rand::rand;
use game_status::GameStatus;

mod allocator;
mod direction;
mod snake;
mod rand;
mod game_status;

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
    points_to_win: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_spawn_idx: usize, points_to_win: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_spawn_idx, 3);

        Self {
             width,
             reward_cell: Some(World::gen_reward_cell(size, &snake.body)),
             snake,
             size,
             next_cell: None,
             status: None,
             points: 0,
             points_to_win,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_head(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("Victory!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No Status"),
        }
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0 != next_cell.0
        {
            self.next_cell = Some(next_cell);
            self.snake.direction = direction;
        }
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => self.make_next_step(),
            None => {},
            _ => {}
        }
    }

    fn make_next_step(&mut self) {
        let next_cell = match self.next_cell {
            Some(cell) => {
                self.next_cell = None;
                cell
            },
            None => self.gen_next_snake_cell(&self.snake.direction),
        };

        self.snake.body.insert(0, next_cell);

        if Some(self.snake_head()) != self.reward_cell {
            self.snake.body.pop();
        
            if self.snake.body[1..].contains(&self.snake.body[0]) {
                self.status = Some(GameStatus::Lost);
            }
        } else {
            self.reward_cell = match self.points {
                x if x == self.points_to_win => {
                    self.status = Some(GameStatus::Won);
                    None
                },
                _ => {
                    self.points += 1;
                    Some(World::gen_reward_cell(self.size, &self.snake.body))
                },
            };
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = rand(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        reward_cell
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head();
        let row = snake_idx / self.width;

        match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if threshold == snake_idx + 1 {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
            Direction::Left => {
                let threshold = row * self.width;
                if threshold == snake_idx {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if threshold == snake_idx {
                    SnakeCell(self.size - self.width + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let threshold = snake_idx + ((self.width - row) * self.width);
                if threshold == snake_idx + self.width {
                    SnakeCell(threshold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            },
        }
    } 
}
