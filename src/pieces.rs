use crate::core::{Figure, Board, Piece, Color, Square};
use crate::game::Game;

impl Piece {
       
    /// Get the theoretically possible moves for each piece type.
    /// This does not check reasons which might prevent a piece from doing a move, such as exposing the same colored king to a check.
    pub fn get_candidate_squares(&self, board: &board) -> Vec<Square> {

    }

    fn candidate_moves<Color, Figure>(&self, board: &board) -> Vec<&Square> {
        
    }

    fn get_candidate_squares_pawn(&self, board: &board) -> Vec<&Square> {
        let curr_square = &self.square as (u8, u8);
        let board_squares = board.squares_as_vec();
        let candidate_squares: Vec<&Square> = vec! [];

        match self.color {
            Color::White => {
                candidate_squares.push(})
            },
            _ => vec! []
        }
    }
    fn get_candidate_squares_rook(&self, board: &board) -> Vec<Square> {
        let directions = [(1,0), (-1,0), (0,1), (0,-1)];
        for direction in directions {
            let curr_square = self.square;
            
        }
        board.get_file_from_square(&self.square).iter().chain(
            board.get_rank_from_square(&self.square).iter()
        ).collect()
    }

    fn get_candidate_squares_knight(&self, board: &board) -> Vec<Square> {}
    fn get_candidate_squares_bishop(&self, board: &board) -> Vec<Square> {
        let directions = [(1,1), (1,-1), (-1,1), (-1,-1)];
    }
    fn get_candidate_squares_queen(&self, board: &board) -> Vec<Square> {}
    fn get_candidate_squares_king(&self, board: &board) -> Vec<Square> {}
    
}