use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

pub struct Segment {
    pub x: u16,
    pub y: u16,
    pub direction: Direction
}

pub struct Snake {
    pub body: VecDeque<(Segment)>
}

impl Segment {

    pub fn is_vertical(&self) -> bool {
        (self.direction == Direction::UP ) | (self.direction == Direction::DOWN)
    }
}

impl Snake {

    pub fn new() -> Snake {
        let mut snake = Snake{body: VecDeque::with_capacity(10)};
        snake.body.push_back(Segment{x: 2, y: 1, direction: Direction::RIGHT});
        snake.body.push_back(Segment{x: 1, y: 1, direction: Direction::RIGHT});
        snake
    }

    pub fn move_dir(&mut self, direction: &Direction) {
        let head = self.body.front().unwrap(); //TODO dont use unwrap

        match direction {
            Direction::UP     => self.body.push_front(Segment{x: head.x, y: head.y - 1, direction: Direction::UP}),
            Direction::DOWN   => self.body.push_front(Segment{x: head.x, y: head.y + 1, direction: Direction::DOWN}),
            Direction::RIGHT  => self.body.push_front(Segment{x: head.x + 1, y: head.y, direction: Direction::RIGHT}),
            Direction::LEFT   => self.body.push_front(Segment{x: head.x - 1, y: head.y, direction: Direction::LEFT}),
        }

        // Remove tail segment
        self.body.pop_back();
    }

    //TODO How to grow when on the edge
    pub fn grow(&mut self) {
        let tail = self.body.back().unwrap(); //TODO dont use unwrap

        match tail.direction {
            Direction::UP     => self.body.push_back(Segment{x: tail.x, y: tail.y + 1, direction: Direction::UP}),
            Direction::DOWN   => self.body.push_back(Segment{x: tail.x, y: tail.y - 1, direction: Direction::DOWN}),
            Direction::RIGHT  => self.body.push_back(Segment{x: tail.x - 1, y: tail.y, direction: Direction::RIGHT}),
            Direction::LEFT   => self.body.push_back(Segment{x: tail.x + 1, y: tail.y, direction: Direction::LEFT}),
        }

    }
}