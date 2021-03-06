// I'm making a tic-tac-toe game to learn rust

use std::fmt;
use std::io;

#[cfg(test)]
mod tests;

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
    win_message: String,
    turn: Turn,
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input
}

impl Board {
    fn get_filtered_input(&mut self) -> u8 {
        let mut valid = false;
        let mut input = 0;
        while !valid {
            input = get_input().trim_end().parse::<u8>().unwrap();

            // check if choice is on board
            valid = match input {
                1..=9 => true,
                _ => false,
            };

            if valid {
                let (x, y) = self.get_pos_from_choice(input);
                valid = self.check_is_safe(x, y);

                if !valid {
                    // check_is_safe returned false
                    println!("- Please enter a number that has not been chosen.");
                }
            } else {
                // number was not from 1 to 9
                println!("- Please enter a number from 1 to 9.");
            }
        }

        input
    }

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

    // get x and y values from index in entire matrix:
    fn get_pos_from_choice(&self, choice: u8) -> (usize, usize) {
        let y = match choice {
            1..=3 => 0,
            4..=6 => 1,
            7..=9 => 2,
            _ => panic!("can only move to values from 1 to 9"),
        };
        let x = usize::from((choice - 1) % 3);
        (x, y)
    }

    // you can't move where someone already moved:
    fn check_is_safe(&mut self, x: usize, y: usize) -> bool {
        match self.board[y][x] {
            Square::Value(_) => true,
            _ => false,
        }
    }

    // set choice to current user
    fn move_to(&mut self, choice: u8) {
        let (x, y) = self.get_pos_from_choice(choice);
        self.board[y][x] = match self.turn {
            Turn::O => Square::O,
            Turn::X => Square::X,
        }
    }

    // leverage fmt::Display to compare Squares
    fn are_equal(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
        self.board[y1][x1].to_string() == self.board[y2][x2].to_string()
    }

    fn all_moves_taken(&self) -> bool {
        for row in self.board.iter() {
            for value in row.iter() {
                match value {
                    Square::Value(_) => return false,
                    _ => (),
                }
            }
        }
        true
    }

    // says that the person at (x,y) won
    fn _set_won(&mut self, x: usize, y: usize) {
        self.won = true;
        self.win_message = format!("{0} won!", self.board[y][x]);
    }

    fn set_game_state(&mut self) {
        // check vertical
        for x in 0..3 {
            if self.are_equal((x, 0), (x, 1)) && self.are_equal((x, 1), (x, 2)) {
                self._set_won(x, 0);
                return;
            }
        }

        // check horizontal
        for y in 0..3 {
            if self.are_equal((0, y), (1, y)) && self.are_equal((1, y), (2, y)) {
                self._set_won(0, y);
                return;
            }
        }

        // check left diagonal
        if self.are_equal((0, 0), (1, 1)) && self.are_equal((1, 1), (2, 2)) {
            self._set_won(0, 0);
            return;
        }

        // check right diagonal
        if self.are_equal((2, 0), (1, 1)) && self.are_equal((1, 1), (0, 2)) {
            self._set_won(2, 0);
            return;
        }

        if self.all_moves_taken() {
            self.won = true;
            self.win_message = "Stalemate!".to_string();
        }
    }

    fn prompt(&mut self) {
        self.print();

        println!("- {0}'s move: ", self.turn);
        let input = self.get_filtered_input();
        self.move_to(input);
        self.set_game_state();

        self.alternate();
    }
}

fn main() {
    let mut b = Board {
        // board must be labled in this order
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };

    while !b.won {
        b.prompt();
    }
    b.print();
    println!("Game over!");
    println!("{}", b.win_message);
}
