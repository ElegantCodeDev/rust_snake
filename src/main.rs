
extern crate crossterm;
extern crate rand;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{KeyCode, KeyEvent, read, Event};
use crossterm::terminal::{self, ClearType};
use crossterm::cursor;
use crossterm::execute;
use std::collections::VecDeque;
use std::time::Duration;
use std::io::{stdout};



#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<(u16, u16)>,
    direction: Direction,
}

impl Snake {
    fn move_forward(&mut self) {
        let head = *self.body.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1.saturating_sub(1)),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0.saturating_sub(1), head.1),
            Direction::Right => (head.0 + 1, head.1),
        };
        self.body.push_front(new_head);
        self.body.pop_back();
    }

    fn grow(&mut self) {
        let tail = *self.body.back().unwrap();
        self.body.push_back(tail);
    }

    fn change_direction(&mut self, new_direction: Direction) {
        if !matches!((self.direction, new_direction), 
            (Direction::Up, Direction::Down) | 
            (Direction::Down, Direction::Up) | 
            (Direction::Left, Direction::Right) | 
            (Direction::Right, Direction::Left)) {
            self.direction = new_direction;
        }
    }
}



fn draw_snake(snake: &Snake) {
    for &(x, y) in &snake.body {
        execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
        print!("■");
    }
}

fn draw_food((x, y): (u16, u16)) {
    execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
    print!("★");
}


const GAME_WIDTH: u16 = 150;
const GAME_HEIGHT: u16 = 50;

fn draw_border() {
    for y in 0..GAME_HEIGHT + 2 {
        for x in 0..GAME_WIDTH + 2 {
            if x == 0 || x == GAME_WIDTH + 1 || y == 0 || y == GAME_HEIGHT + 1 {
                execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
                print!("▓");
            }
        }
    }
}


fn draw_score(score: usize) {
    execute!(stdout(), cursor::MoveTo(2, GAME_HEIGHT + 2)).unwrap();
    print!("Score: {}", score);
}

fn game_over() {
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout(), cursor::MoveTo(GAME_WIDTH / 4, GAME_HEIGHT / 2)).unwrap();
    print!("Game Over! Press Esc to exit...");
}
fn main() {
    enable_raw_mode().unwrap();
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

    let mut snake = Snake {
        body: VecDeque::from(vec![(10, 10), (10, 11), (10, 12)]),
        direction: Direction::Up,
    };

    let mut food = spawn_food();

    loop {
        execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();

        draw_snake(&snake);
        draw_food(food);
        draw_score(snake.body.len());

        if snake_collides(&snake) {
            game_over();
            if let Event::Key(KeyEvent { code: KeyCode::Esc, .. }) = read().unwrap() {
                break;
            }
        }

        if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Up => snake.change_direction(Direction::Up),
                    KeyCode::Down => snake.change_direction(Direction::Down),
                    KeyCode::Left => snake.change_direction(Direction::Left),
                    KeyCode::Right => snake.change_direction(Direction::Right),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        snake.move_forward();

        if snake.body.front() == Some(&food) {
            snake.grow();
            food = spawn_food();
        }

        execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
    }

    disable_raw_mode().unwrap();
}



fn spawn_food() -> (u16, u16) {
    (rand::random::<u16>() % (GAME_WIDTH - 10) + 1, rand::random::<u16>() % (GAME_HEIGHT - 10) + 1)
}

fn snake_collides(snake: &Snake) -> bool {
    let &(head_x, head_y) = snake.body.front().unwrap();
    if head_x == 0 || head_x == GAME_WIDTH + 1 || head_y == 0 || head_y == GAME_HEIGHT + 1 {
        return true;
    }
    for (i, &(x, y)) in snake.body.iter().enumerate().skip(1) {
        if x == head_x && y == head_y {
            return true;
        }
    }
    false
}