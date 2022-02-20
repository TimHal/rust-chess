use crate::core::{Figure, Board, Piece, Color, Square};
use crate::game::Game;

use std::collections::HashSet;

impl Piece {
       
    /// Get the theoretically possible moves for each piece type.
    /// This does not check reasons which might prevent a piece from doing a move, such as exposing the same colored king to a check.
    pub fn get_available_squares(&self, board: &Board) ->  HashSet<&Square> {
        self.get_available_squares_for_piece<self.color, self.figure>(board)
    }

    fn get_available_squares_for_piece<Color, Figure>(&self, board: &Board) -> HashSet<&Square> {
        
    } 
}