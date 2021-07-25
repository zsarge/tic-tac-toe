use super::*;

#[test]
fn test_basic() {
    assert!(1 == 1);
}

#[test]
fn test_all_moves_taken() {
    let b = Board {
        // board must be labled in this order
        board: [
            [Square::X, Square::X, Square::X],
            [Square::X, Square::X, Square::X],
            [Square::X, Square::X, Square::X],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == true);
}

#[test]
fn test_diagonal_1() {
    let mut b = Board {
        // board must be labled in this order
        board: [
            [Square::O, Square::O, Square::X],
            [Square::O, Square::X, Square::O],
            [Square::X, Square::O, Square::X],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == true);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message != "".to_string());
}

#[test]
fn test_diagonal_2() {
    let mut b = Board {
        // board must be labled in this order
        board: [
            [Square::O, Square::O, Square::X],
            [Square::O, Square::X, Square::Value(6)],
            [Square::X, Square::O, Square::X],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == false);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message != "".to_string());
}
