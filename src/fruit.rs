extern crate rand;

use rand::Rng;

pub struct Fruit {
    pub x: u16,
    pub y: u16,
}

impl Fruit {

    // Fruit should be positioned within the constraints of the board.
    // Then again, fruit should have no knowledge of board, so maybe move this elsewhere.
    pub fn generate(width: u16, height: u16) -> Fruit {
        //TODO Remove this declaration from here
        let mut rng = rand::thread_rng();

        Fruit{x: rng.gen_range(2, width), y: rng.gen_range(2, height)}
    }
}
