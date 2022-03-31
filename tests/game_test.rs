use itertools::Itertools;
use rust_chess::{core::Color, game::{Game}};


#[test]
fn create_game() {
    let mut game = Game::new();
    let moves = game.get_moves(Color::White);
    let move_to_make = moves.iter().next().unwrap();
    //dbg!(move_to_make);
    game.make_move(move_to_make);
    
    // for move_ in game.get_moves(Color::White).iter().map(|m| m.to_str()).sorted() {
    //     println!("{}", move_);
    // }
    // dbg!(game.get_moves(Color::White).len());

    assert_eq!(1,1)
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