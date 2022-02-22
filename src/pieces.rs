use crate::core::{Figure, Board, Piece, Color, Square};

use std::collections::HashSet;

impl Piece {
       
    /// Get the theoretically possible moves for each piece type.
    /// This does not check reasons which might prevent a piece from doing a move, such as exposing the same colored king to a check.
    pub fn get_available_squares(&self, board: &Board) ->  HashSet<Square> {
        // TODO: make sure the piece can move from it's current square without exposing it's king to a check
        
        match (self.figure, self.color) {
            (Figure::Pawn, Color::White) => self.get_available_squares_pawn_white(board),
            // (Figure::Pawn, Color::Black) => self.get_available_squares_pawn_black(board),
            // (Figure::Rook, _) => self.get_available_squares_rook(board),
            // (Figure::Knight, _) => self.get_available_squares_knight(board),
            // (Figure::Bishop, _) => self.get_available_squares_bishop(board),
            // (Figure::King, _) => self.get_available_squares_king(board),
            // (Figure::Queen, _) => self.get_available_squares_queen(board),
            _ => panic!("Unrecognized piece type at get_available_squares()!"),
        }
    }

    fn get_available_squares_pawn_white(&self, board: &Board) ->  HashSet<Square> {
        let results = &mut HashSet::new();
        let move_up_one = self.square.move_by((0,1)); 
        if move_up_one.is_some() {
            // check there is no piece on that square
            if board.check_square_for_piece(board.get_from_square_unchecked(&move_up_one.unwrap())).is_none() {
                results.insert(*board.get_from_square_unchecked(&move_up_one.unwrap()));

                let move_up_two = move_up_one.unwrap().move_by((0,1));
                if self.square.pos.1 == '2' && move_up_two.is_some() {
                    if board.check_square_for_piece(board.get_from_square_unchecked(&move_up_two.unwrap())).is_none() {
                        results.insert(*board.get_from_square_unchecked(&move_up_two.unwrap()));
                    }
                }
            }
        }
        results.clone()
    }

    fn get_available_squares_pawn_black(&self, board: &Board) ->  HashSet<&Square> {
        HashSet::new()
    }

    fn get_available_squares_rook(&self, board: &Board) ->  HashSet<&Square> {
        HashSet::new()
    }
    fn get_available_squares_knight(&self, board: &Board) ->  HashSet<&Square> {
        HashSet::new()
    }
    fn get_available_squares_bishop(&self, board: &Board) ->  HashSet<&Square> {
        HashSet::new()
    }
    
    fn get_available_squares_king(&self, board: &Board) ->  HashSet<&Square> {
        HashSet::new()
    }
    
    fn get_available_squares_queen(&self, board: &Board) ->  HashSet<&Square> {
        HashSet::new()
    }

    fn print_piece(&self) -> String {
        let str_repr = match self.figure {
            Figure::Pawn => String::from("p"),
            Figure::Rook => String::from("r"),
            Figure::Knight => String::from("n"),
            Figure::Bishop => String::from("b"),
            Figure::Queen => String::from("q"),
            Figure::King =>String::from( "k")
        };

        if self.color == Color::White {
            return str_repr.to_ascii_uppercase()
        }
        
        str_repr
    }
}
