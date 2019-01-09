extern crate termion;

use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::{thread, time};
use std::io::{Write, Read, stdout};

mod board;
mod snake;

const SLEEP_MS: time::Duration = time::Duration::from_millis(1000);

struct Game {
    score: u8,
    direction: snake::Direction
}

fn main() {
    let mut snake = snake::Snake::new();

    let stdout = stdout(); // Needs to be separate to be bound to the scope
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin =  async_stdin();

    redraw(&mut stdout, &snake);

    loop {
        snake.crawl();

        update_direction(&mut stdin, &mut snake);

        if game_over(&snake){
            break
        } else {
            redraw(&mut stdout, &snake);
        }

        thread::sleep(SLEEP_MS)
    }
}

fn update_direction(stdin: &mut Read, snake: &mut snake::Snake){
    if let Some(Ok(c)) = stdin.keys().last() {
        match c {
            Key::Left => snake.direction = snake::Direction::LEFT,
            Key::Right => snake.direction = snake::Direction::RIGHT,
            Key::Up => snake.direction = snake::Direction::UP,
            Key::Down => snake.direction = snake::Direction::DOWN,
            _ => (), //TODO Handle other keys
        }
    }
}

fn game_over(snake: &snake::Snake) -> bool {
    let head = snake.body.front().unwrap();

    //TODO Check for hitting itself
    head.x == 0|| head.y == 0 || head.x == board::Board::WIDTH - 1 || head.y == board::Board::HEIGHT - 1
}

fn redraw(stdout: &mut Write, snake: &snake::Snake){
    clear_board(stdout);
    draw_borders(stdout);
    draw_snake(snake, stdout);
    flush(stdout);
}

fn flush(stdout: &mut Write){
    write!(stdout, "{}", termion::cursor::Goto(board::Board::WIDTH, board::Board::HEIGHT)).unwrap();
    stdout.flush().unwrap();
}

fn clear_board(stdout: &mut Write) {
    write!(stdout, "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
}

fn draw_snake(snake: &snake::Snake, stdout: &mut Write) {
    for (_, segment) in snake.body.iter().enumerate(){
        let symbol = if segment.is_vertical() { "|" } else { "-" };

        write!(stdout, "{}{}", termion::cursor::Goto(segment.x + 1, segment.y + 1), symbol).unwrap();
    }
}
//
//fn draw_snake_update(snake: &snake::Snake, stdout: &mut Write) {
//    // To only draw the updates: remove the tail, draw the head
//
//    let head = snake.body.front().unwrap();
//    let tail = snake.body.back().unwrap();
//
//    let symbol = if head.is_vertical() { "|" } else { "-" };
//
//    write!(stdout, "{}{}", termion::cursor::Goto(tail.x + 1, tail.y + 1), " ").unwrap();
//    write!(stdout, "{}{}", termion::cursor::Goto(head.x + 1, head.y + 1), symbol).unwrap();
//}

fn draw_borders(stdout: &mut Write) {
    for y in 2..board::Board::HEIGHT {
        write!(stdout, "{}{}", termion::cursor::Goto(0, y as u16), "0").unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(board::Board::WIDTH, y as u16), "0").unwrap();
    }

    for x in 2..board::Board::WIDTH {
        write!(stdout, "{}{}", termion::cursor::Goto(x as u16, 0), "0").unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(x as u16, board::Board::HEIGHT), "0").unwrap();
    }
}