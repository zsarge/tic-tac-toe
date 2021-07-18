// I'm making a tic-tac-toe game to learn rust

use std::fmt;
use std::io;
use std::io::*;

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

enum Turn {
    X,
    O,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Turn::X => 'X',
            Turn::O => 'O',
        };
        write!(f, "{}", printable)
    }
}

struct Board {
    // a 3x3 board
    board: [[Square; 3]; 3],
    won: bool,
    turn: Turn,
}

fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("DEBUG: {}", input);
    input
}

impl Board {
    fn print(&self) {
        println!("");
        for (index, row) in self.board.iter().enumerate() {
            println!(" {0} | {1} | {2} ", row[0], row[1], row[2]);

            if index < self.board.len() - 1 {
                println!("---+---+---");
            }
        }
        println!("");
    }

    fn prompt(&self) {
        Board::print(self);

        println!("- {} move: ", self.turn);
        let input = take_input();
    }
}

fn main() {
    let b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        turn: Turn::X,
    };

    Board::prompt(&b);
}
