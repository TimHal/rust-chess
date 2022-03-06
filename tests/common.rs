/// Module for setup and teardown of test configurations

use rust_chess::core::{Board, Square, Color, Figure, Piece};
use std::collections::HashSet;

pub fn empty_board() -> Board {
    rust_chess::core::Board::new()
}

pub fn setup_board_in_std_start_position() -> Board {
    let std_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board_in_starting_position = rust_chess::parser::parse_fen(std_fen).unwrap();
    board_in_starting_position
}

// utility function for quicker testing, provide a ','-separated list of squares
pub fn sqrs(board: &Board, squares: &str) -> HashSet<Square> {
    
    let sq_refs: HashSet<Square> = squares.split(",")  
        .map(|sq| *board.get_unchecked(sq.trim())).collect::<HashSet<Square>>();
    sq_refs
}
