// I'm making a tic-tac-toe game to learn rust

use std::fmt;

// A square in a tic-tac-toe board
enum Square {
    Value(u8),
    X,
    O,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Square::Value(x) => x.to_string(),
            Square::X => "X".to_string(),
            Square::O => "O".to_string(),
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
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
    };
    Board::print(&b);
}
