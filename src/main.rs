extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

mod board;

fn main() {
    let board = board::Board::new();

    let mut stdout = stdout(); // Needs to be separate to be bound to the scope
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    clear_board(&mut stdout);
    draw_board(board, &mut stdout)
}

fn clear_board(stdout: &mut Write) {
    write!(stdout, "{}{}{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();
}

fn draw_board(board: board::Board, stdout: &mut Write) {

    for(rownum, row) in board.grid.iter().enumerate(){
        for (colnum, _) in row.iter().enumerate() {
            write!(stdout, "{}{}",

                   // Move cursor to correct place on grid
                   termion::cursor::Goto(colnum as u16 + 1, rownum as u16 + 1),

                   // Write character at that point
                   board.grid[rownum][colnum]).unwrap();
        }

        write!(stdout, "\n");
    }

    stdout.flush().unwrap();
}