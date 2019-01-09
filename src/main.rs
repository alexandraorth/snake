extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::{thread, time};
use std::io::{Write, stdout, stdin};

mod board;
mod snake;

const SLEEP_MS: time::Duration = time::Duration::from_millis(1000);

struct Game {
    score: u8
}

fn main() {
    let mut snake = snake::Snake::new();

    let stdoutold = stdout(); // Needs to be separate to be bound to the scope

    let mut stdin =  stdin();
    let mut stdout = stdoutold.lock().into_raw_mode().unwrap();

    redraw(&mut stdout, &snake);

    for c in stdin.keys() {

        // Clear the current line.
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        // TODO Handle other keys
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Left      => snake.move_dir(snake::Direction::LEFT),
            Key::Right     => snake.move_dir(snake::Direction::RIGHT),
            Key::Up        => snake.move_dir(snake::Direction::UP),
            Key::Down      => snake.move_dir(snake::Direction::DOWN),
            _              => println!("Other"),
        }

        if game_over(&snake){
            break
        }

        redraw(&mut stdout, &snake);
    }
//
//    loop {
//        snake.move_dir(snake::Direction::RIGHT);
//
//        redraw(&mut stdout, &snake);
//
//        thread::sleep(SLEEP_MS)
//    }
}

fn game_over(snake: &snake::Snake) -> bool {
    let head = snake.body.get(0).unwrap();

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
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
}

//TODO: Only draw the updates
fn draw_snake(snake: &snake::Snake, stdout: &mut Write) {
    for (_, segment) in snake.body.iter().enumerate(){

        write!(stdout, "{}", termion::cursor::Goto(segment.x + 1, segment.y + 1)).unwrap();

        if segment.is_vertical() {
            write!(stdout, "{}", "|").unwrap()
        } else {
            write!(stdout, "{}", "-").unwrap()
        }
    }
}

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