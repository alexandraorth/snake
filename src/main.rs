extern crate termion;

use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::{thread, time};
use std::io::{Write, Read, stdout};
use std::time::{Duration, Instant};

mod snake;
mod fruit;

use self::fruit::{Fruit};

//TODO Sort out passing things around
//TODO Snake should die when it hits itself
//TODO Scoring
//TODO Make pretty
//TODO Host on web
//TODO Clean up code
//TODO Quit when hitting "Q" char
//TODO Sort out where to have a +1 and where not (indexing)
//TODO Snake should be able to reverse
//TODO Make a "position" trait?
//TODO Turning quickly?
//TODO Snake should speed up on a curve

struct Game {
    speed: Duration,
    snake: snake::Snake,
    fruit: Fruit,
    width: u16,
    height: u16,
}

fn main() {
    let mut game = Game {
        speed: Duration::from_millis(1000),
        snake: snake::Snake::new(),
        width: 20,
        height: 20,
        fruit: Fruit::generate(20, 20), //TODO Use above width and height
    };

    let stdout = stdout(); // Needs to be separate to be bound to the scope
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin =  async_stdin();
    
    clear_board(&mut stdout);
    draw_borders(&mut stdout, &game);
    draw_snake(&mut stdout, &game);
    draw_fruit(&mut stdout, &game);
    flush(&mut stdout, &game);

    let mut last_loop = Instant::now();
    loop {
        if last_loop.elapsed() < game.speed {
            continue
        } else {
            last_loop = Instant::now()
        }

        clear_fruit(&mut stdout, &game);
        clear_snake(&mut stdout, &game);

        update_direction(&mut stdin, &mut game.snake);

        game.snake.crawl();

        if snake_eat_fruit(&game) {
            game.snake.grow();
            game.fruit = Fruit::generate(game.width - 1, game.height - 1);
            game.speed = game.speed.checked_sub(Duration::from_millis(100)).unwrap();
        }

        if game_over(&game){
            break
        }

        draw_fruit(&mut stdout, &game);
        draw_snake(&mut stdout, &game);

        flush(&mut stdout, &game);
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

fn snake_eat_fruit(game: &Game) -> bool {
    let head = game.snake.body.front().unwrap();

    head.x == game.fruit.x && head.y == game.fruit.y
}

fn game_over(game: &Game) -> bool {
    let head = game.snake.body.front().unwrap();

    //TODO Check for hitting itself
    head.x == 0|| head.y == 0 || head.x == game.width - 1 || head.y == game.height - 1
}

fn flush<W: Write>(stdout: &mut W, game: &Game){
    //TODO Figure out why we need to do to the last point before flushing
    write!(stdout, "{}", termion::cursor::Goto(game.width, game.height)).unwrap();
    stdout.flush().unwrap();
}

fn clear_board<W: Write>(stdout: &mut W) {
    write!(stdout, "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
}

fn clear_fruit<W: Write>(stdout: &mut W, game: &Game){
    draw_at_position(stdout, game.fruit.x, game.fruit.y, ' ');
}

fn draw_fruit<W: Write>(stdout: &mut W, game: &Game){
    draw_at_position(stdout, game.fruit.x, game.fruit.y, '*');
}

fn clear_snake<W: Write>(stdout: &mut W, game: &Game){
    for (_, segment) in game.snake.body.iter().enumerate(){
        draw_at_position(stdout, segment.x, segment.y, ' ');
    }
}

fn draw_snake<W: Write>(stdout: &mut W, game: &Game) {
    for (_, segment) in game.snake.body.iter().enumerate(){
        let symbol = if segment.is_vertical() { '|' } else { '-' };

        draw_at_position(stdout, segment.x, segment.y, symbol);
    }
}

fn draw_at_position<W: Write>(stdout: &mut W, x: u16, y: u16, symbol: char){
    write!(stdout, "{}{}", termion::cursor::Goto(x + 1, y + 1), symbol).unwrap();
}

fn draw_borders<W: Write>(stdout: &mut W, game: &Game) {
    for y in 2..game.height {
        write!(stdout, "{}{}", termion::cursor::Goto(0, y as u16), "0").unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(game.width, y as u16), "0").unwrap();
    }

    for x in 2..game.width {
        write!(stdout, "{}{}", termion::cursor::Goto(x as u16, 0), "0").unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(x as u16, game.height), "0").unwrap();
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