extern crate termion;
extern crate rand;

use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use rand::Rng;
use std::{thread, time};
use std::io::{Write, Read, stdout};
use std::time::{Duration, Instant};

mod snake;

//TODO Sort out passing things around
//TODO Snake should die when it hits itself
//TODO Snake should eat things and grow
//TODO Snake should speed up when it eats
//TODO Scoring
//TODO Make pretty
//TODO Host on web
//TODO Clean up code
//TODO Quit when hitting "Q" char
//TODO Sort out where to have a +1 and where not
//TODO Snake should be able to reverse
//TODO Make a "position" trait?
//TODO Turning quickly?

struct Fruit {
    x: u16,
    y: u16,
}

impl Fruit {

    //Generate a bit of Fruit within the given constraints
    fn generate(width: u16, height: u16) -> Fruit{
        //TODO Remove this declaration from here
        let mut rng = rand::thread_rng();

        Fruit{x: rng.gen_range(2, width), y: rng.gen_range(2, height)}
    }
}

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

    redraw(&mut stdout, &game);

    let mut last_loop = Instant::now();
    loop {
        if last_loop.elapsed() < game.speed {
            continue
        } else {
            last_loop = Instant::now()
        }

        update_direction(&mut stdin, &mut game.snake);

        game.snake.crawl();

        if snake_eat_fruit(&game) {
            game.snake.grow();
            game.fruit = Fruit::generate(game.width, game.height);
            game.speed = game.speed.checked_sub(Duration::from_millis(100)).unwrap();
        }

        if game_over(&game){
            break
        } else {
            redraw(&mut stdout, &game);
        }
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

fn redraw(stdout: &mut Write, game: &Game){
    clear_board(stdout);
    draw_borders(stdout, game);
    draw_snake(&game.snake, stdout);
    draw_fruit(&game, stdout);
    flush(stdout, &game);
}

fn flush(stdout: &mut Write, game: &Game){
    //TODO Figure out why we need to do to the last point before flushing
    write!(stdout, "{}", termion::cursor::Goto(game.width, game.height)).unwrap();
    stdout.flush().unwrap();
}

fn clear_board(stdout: &mut Write) {
    write!(stdout, "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
}

fn draw_fruit(game: &Game, stdout: &mut Write){
    write!(stdout, "{}{}", termion::cursor::Goto(game.fruit.x + 1, game.fruit.y + 1), "*");
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

fn draw_borders(stdout: &mut Write, game: &Game) {
    for y in 2..game.height {
        write!(stdout, "{}{}", termion::cursor::Goto(0, y as u16), "0").unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(game.width, y as u16), "0").unwrap();
    }

    for x in 2..game.width {
        write!(stdout, "{}{}", termion::cursor::Goto(x as u16, 0), "0").unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(x as u16, game.height), "0").unwrap();
    }
}