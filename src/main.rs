// I'm making a tic-tac-toe game to learn rust

// TO DO:
// Consider making union type of " ", "X", and "O" ?

enum Square {
    Null,
    X,
    O,
}

struct Board {
    // a 3x3 board
    board: [[Square; 3]; 3],
}

fn format(s: &Square) -> String {
    match s {
        Square::Null => " ",
        Square::X => "X",
        Square::O => "O",
    }
    .to_string()
}

impl Board {
    fn print(&self) {
        for row in self.board.iter() {
            print!("[");
            for (idx, val) in row.iter().enumerate() {
                print!("{}", format(val));
                if idx < row.len() - 1 {
                    print!(", ");
                }
            }
            println!("]");
        }
    }
}

fn main() {
    let b = Board {
        board: [[Square::Null, Square::Null, Square::Null], [Square::Null, Square::Null, Square::Null], [Square::Null, Square::Null, Square::Null]],
    };
    Board::print(&b);
}
