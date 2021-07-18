// I'm making a tic-tac-toe game to learn rust

struct Board {
    // a 3x3 board
    board: [[u8; 3]; 3],
}

impl Board {
    fn print(&self) {
        for i in self.board.iter() {
            print!("[");
            for (idx, val) in i.iter().enumerate() {
                print!("{}", val);
                if idx < i.len() - 1 {
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
