// I'm making a tic-tac-toe game to learn rust

// TO DO:
// Consider making union type of " ", "X", and "O" ?

struct Board {
    // a 3x3 board
    board: [[u8; 3]; 3],
}

fn format(n: &u8) -> String {
    match n {
        0 => " ",
        1 => "X",
        2 => "O",
        _ => panic!("not valid"),
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
        board: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
    };
    Board::print(&b);
}
