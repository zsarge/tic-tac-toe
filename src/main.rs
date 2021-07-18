// I'm making a tic-tac-toe game to learn rust

use std::fmt;
use std::io;

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

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("DEBUG: {}", input);
    input
}

fn get_filtered_input() -> u8 {
    let mut valid = false;
    let mut input = 0;
    while !valid {
        input = get_input().trim_end().parse::<u8>().unwrap();
        valid = match input {
            1..=9 => true,
            _ => false,
        };
        if !valid {
            println!("Please enter a number from 1 to 9.");
        }
    }
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

    fn alternate(&mut self) {
        self.turn = match self.turn {
            Turn::O => Turn::X,
            Turn::X => Turn::O,
        };
    }

    fn move_to(&mut self, choice: u8) {
        let y = match choice { // I'm sure there's a better way to do this
            1..=3 => 0,
            4..=6 => 1,
            7..=9 => 2,
            _ => panic!("can only move to values from 1 to 9")
        };
        let x = usize::from((choice - 1) % 3);
        self.board[y][x] = match self.turn {
            Turn::O => Square::O,
            Turn::X => Square::X,
        }
    }

    fn prompt(&mut self) {
        Board::print(self);

        println!("- {} move: ", self.turn);
        let input = get_filtered_input();
        Board::move_to(self, input);

        Board::alternate(self);
    }
}

fn main() {
    let mut b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        turn: Turn::X,
    };

    while !b.won {
        Board::prompt(&mut b);
    }
}
