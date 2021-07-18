// I'm making a tic-tac-toe game to learn rust

use std::fmt;

// A square in a tic-tac-toe board
enum Square {
    Null,
    X,
    O,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Square::Null => '*',
            Square::X => 'X',
            Square::O => 'O',
        };
        write!(f, "{}", printable)
    }
}

struct Board {
    // a 3x3 board
    board: [[Square; 3]; 3],
}

impl Board {
    fn print(&self) {
        for (index, row) in self.board.iter().enumerate() {
            println!(" {0} | {1} | {2} ", row[0], row[1], row[2]);

            if index < self.board.len() - 1 {
                println!("---+---+---");
            }
        }
    }
}

fn main() {
    let b = Board {
        board: [[Square::Null, Square::Null, Square::Null], [Square::Null, Square::Null, Square::Null], [Square::Null, Square::Null, Square::Null]],
    };
    Board::print(&b);
}
