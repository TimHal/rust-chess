use itertools::Itertools;
mod common; 
use rust_chess::{core::{Color, Square, Piece, Figure}, game::{Game}};


#[test]
fn create_game() {
    let mut game = Game::new();

    // for i in (1..=10) {
    //     let moves = game.get_moves(game.current_color());
    //     let move_to_make = *moves.iter().next().unwrap();
    //     //dbg!(move_to_make);
    //     game.make_move(move_to_make);

    //     println!("{}", game.board);
    // }

    // for move_ in game.moves.iter() {
    //     println!("{}", move_);
    // }

      game
        .move_from_str("e2-e4")
        .move_from_str("e7-e6")
        .move_from_str("d2-d4")
        .move_from_str("f8-b4")
        .move_from_str("b1-c3");
    println!("{}", game.board);

    assert_eq!(1,1)
}

#[test]
fn simulate_move() {
    let mut original = Game::new();
    original.move_from_str("e2-e4")
            .move_from_str("e7-e5")
            .move_from_str("d2-d4"); 

    let copy = original.simulate_move(original.generate_move_from_str("e5-d4"));
    let copyd4 = copy.board.check_square_for_piece(&Square::from_str("d4")).unwrap(); 
    let origd4 = original.board.check_square_for_piece(&Square::from_str("d4")).unwrap(); 

    assert_eq!(Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("d4")}, *copyd4);
    assert_eq!(Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("d4")}, *origd4);

    // simulate_move with state checks
    let mut board = common::empty_board();
    let kd8 = Piece {color: Color::Black, figure: Figure::King, square: *board.get_unchecked("d8")};
    let kc2 = Piece {color: Color::White, figure: Figure::King, square: *board.get_unchecked("c2")};
    let rc1 = Piece {color: Color::White, figure: Figure::Rook, square: *board.get_unchecked("c1")};
    board.pieces.extend(vec! [kd8, kc2, rc1]);

    let mut game = Game::new();
    game.board = board; 

    assert_eq!(Color::White, game.current_color());
    // check state against virtual move 
    // move the rook from c1 to a checking square (d1)
    let mut vmove = game.generate_move_from_str("c1-d1");
    assert_eq!(true, game.simulate_move(vmove).in_check());
    assert_eq!(false, game.in_check());

    // put an extra piece to precent check from d1 and see if removing the pinned piece correctly leads to a check-state
    game.board.pieces.push(Piece {color: Color::Black, figure: Figure::Rook, square: Square::from_str("d7")});
    game.move_from_str("c1-d1");
    assert_eq!(false, game.in_check_color(Color::Black));
    vmove = game.generate_move_from_str("d7-e7");
    assert_eq!(true, game.simulate_move(vmove).in_check_color(Color::Black));
}

#[test]
fn available_moves() {
    assert_eq!(1,1)
}

#[test]
fn make_valid_moves() {
    assert_eq!(1,1)
}

#[test]
fn attempt_invalid_moves() {
    assert_eq!(1,1)
}

#[test]
fn is_checkmate() {
    assert_eq!(1,1)
}

#[test]
fn check_threefold_repitition() {
    assert_eq!(1,1)
}

#[test]
fn check_insufficient_material() {
    assert_eq!(1,1)
}

#[test]
fn check_en_passant_move() {
    assert_eq!(1,1)
}

#[test]
fn check_fifty_move_rule() {
    assert_eq!(1,1)
}