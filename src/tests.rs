use super::*;

#[test]
fn test_basic() {
    assert!(1 == 1);
}

#[test]
fn test_all_moves_taken() {
    let b = Board {
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
    //! test all_moves_taken and set_game_state
    let mut b = Board {
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
    //! test all_moves_taken and set_game_state
    let mut b = Board {
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

#[test]
fn test_diagonal_3() {
    //! test 2 diagonals
    let mut b = Board {
        board: [
            [Square::X, Square::O, Square::X],
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


#[test]
fn test_horizontal_1() {
    //! test horizontal in top row
    let mut b = Board {
        board: [
            [Square::O, Square::O, Square::O],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == false);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message.contains("O"));
}


#[test]
fn test_horizontal_2() {
    //! test horizontal in middle row
    let mut b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::O, Square::O, Square::O],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == false);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message.contains("O"));
}


#[test]
fn test_horizontal_3() {
    //! test horizontal in last row
    let mut b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::O, Square::O, Square::O],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == false);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message.contains("O"));
}


