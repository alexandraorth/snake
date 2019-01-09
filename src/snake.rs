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
    pub body: VecDeque<(Segment)>,
    pub direction: Direction //Remove direction from here, move into Segment head
}

impl Segment {
    pub fn is_vertical(&self) -> bool {
        (self.direction == Direction::UP ) | (self.direction == Direction::DOWN)
    }
}

impl Snake {

    pub fn new() -> Snake {
        let mut snake = Snake{body: VecDeque::with_capacity(10), direction: Direction::RIGHT};
        snake.body.push_back(Segment{x: 2, y: 1, direction: Direction::RIGHT});
        snake.body.push_back(Segment{x: 1, y: 1, direction: Direction::RIGHT});
        snake
    }

    //TODO Swap head when reserving direction
    pub fn crawl(&mut self) {
        let head = self.body.front().unwrap(); //TODO dont use unwrap

        match self.direction {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_created_has_size_2(){
        let snake = Snake::new();

        assert_eq!(snake.body.len(), 2)
    }

    #[test]
    fn growing_snake_has_increased_size(){
        let mut snake = Snake::new();

        snake.grow();

        assert_eq!(snake.body.len(), 3)
    }

    #[test]
    fn crawling_snake_has_same_size(){
        let mut snake = Snake::new();

        snake.crawl();

        assert_eq!(snake.body.len(), 2)
    }

    #[test]
    fn crawling_snake_has_moved(){
        let mut snake = Snake::new();

        assert_eq!(2, snake.body.front().unwrap().x);
        assert_eq!(1, snake.body.front().unwrap().y);

        snake.crawl();

        assert_eq!(3, snake.body.front().unwrap().x);
        assert_eq!(1, snake.body.front().unwrap().y);
    }
}