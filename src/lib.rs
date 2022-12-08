use rand::Rng;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Mode {
    Finite,
    Infinite,
}

pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    mode: Mode,
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, size: usize, mode: Mode) -> World {
        World {
            mode,
            width,
            size: width * width,
            snake: Snake::new(rand::thread_rng().gen_range(0..width * width), size),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;
    }

    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match self.snake.direction {
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    self.mode_test(SnakeCell((self.size - self.width) + treshold))
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    self.mode_test(SnakeCell(treshold - self.width))
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    self.mode_test(SnakeCell(treshold + (self.width - 1)))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    self.mode_test(SnakeCell(treshold - ((row + 1) * self.width)))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }

    fn mode_test(&self, snake_cell: SnakeCell) -> SnakeCell {
        if self.mode == Mode::Finite {
            snake_cell
        } else {
            SnakeCell(self.size)
        }
    }

    fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
}

// wasm-pack build --target web
