
pub struct Board {
    pub grid: [[u8; Board::WIDTH as usize] ; Board::HEIGHT as usize],
}

impl Board {
    pub const WIDTH: u8 = 20;
    pub const HEIGHT: u8 = 20;

    pub fn new() -> Board {
        Board{ grid: [[0; Board::WIDTH as usize]; Board::HEIGHT as usize]}
    }
}