
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

pub struct Snake {
    pub body: VecDeque<(u8, u8)>
}

impl Snake {

    pub fn new() -> Snake {
        Snake{body: vec_deque![(1,1)(1,2)]}
    }

//    pub fn move_dir(&self, direction: Direction) {
//        match direction {
//            Direction::UP     =>  ,
//            Direction::DOWN   =>  ,
//            Direction::RIGHT  =>  ,
//            Direction::LEFT   =>  ,
//        }
//    }

//    pub fn grow(&self) {
//        self.body
//    }
}