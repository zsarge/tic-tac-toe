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


#[test]
fn test_vertical_1() {
    //! test vertical in first column
    let mut b = Board {
        board: [
            [Square::O, Square::Value(2), Square::Value(3)],
            [Square::O, Square::Value(5), Square::Value(6)],
            [Square::O, Square::Value(8), Square::Value(9)],
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
fn test_vertical_2() {
    //! test vertical in first column
    let mut b = Board {
        board: [
            [Square::X, Square::Value(2), Square::Value(3)],
            [Square::X, Square::Value(5), Square::Value(6)],
            [Square::X, Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == false);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message.contains("X"));
}

#[test]
fn test_vertical_3() {
    //! test vertical in middle column
    let mut b = Board {
        board: [
            [Square::Value(1), Square::O, Square::Value(3)],
            [Square::Value(4), Square::O, Square::Value(6)],
            [Square::Value(7), Square::O, Square::Value(9)],
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
fn test_vertical_4() {
    //! test vertical in last column
    let mut b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::O],
            [Square::Value(4), Square::Value(5), Square::O],
            [Square::Value(7), Square::Value(8), Square::O],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    assert!(b.all_moves_taken() == false);
    assert!(b.won == false);
    b.set_game_state();
    assert!(b.won == true);
    assert!(b.win_message.contains("O"));
}


#[test]
fn test_get_pos_from_choice_1() {
    // Board does not really matter
    let b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    
    // boring code is readable code
    assert!(b.get_pos_from_choice(1) == (0, 0));
    assert!(b.get_pos_from_choice(2) == (1, 0));
    assert!(b.get_pos_from_choice(3) == (2, 0));
    assert!(b.get_pos_from_choice(4) == (0, 1));
    assert!(b.get_pos_from_choice(5) == (1, 1));
    assert!(b.get_pos_from_choice(6) == (2, 1));
    assert!(b.get_pos_from_choice(7) == (0, 2));
    assert!(b.get_pos_from_choice(8) == (1, 2));
    assert!(b.get_pos_from_choice(9) == (2, 2));
}


#[test]
#[should_panic]
fn test_get_pos_from_choice_2() {
    // Board does not really matter
    let b = Board {
        board: [
            [Square::Value(1), Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    
    b.get_pos_from_choice(10);
}

#[test]
fn test_check_is_safe() {
    let mut b = Board {
        board: [
            [Square::X,        Square::Value(2), Square::Value(3)],
            [Square::Value(4), Square::Value(5), Square::Value(6)],
            [Square::Value(7), Square::Value(8), Square::Value(9)],
        ],
        won: false,
        win_message: "".to_string(),
        turn: Turn::X,
    };
    
    // boring code is readable code
    assert!(b.check_is_safe(0,0) == false);

    for i in 2..=9 {
        let (x, y) = b.get_pos_from_choice(i);
        assert!(b.check_is_safe(x, y) == true);
    }
}
