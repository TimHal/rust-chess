use crate::core::{Figure, Board, Piece, Color, Square};

use std::collections::HashSet;
use itertools::Itertools;

impl Piece {
       
    /// Get the theoretically possible moves for each piece type.
    /// This does not check reasons which might prevent a piece from doing a move, such as exposing the same colored king to a check.
    pub fn get_available_squares(&self, board: &Board) ->  HashSet<Square> {
        // TODO: make sure the piece can move from it's current square without exposing it's king to a check
        
        match (self.figure, self.color) {
            (Figure::Pawn, Color::White) => self.get_available_squares_pawn_white(board),
            (Figure::Pawn, Color::Black) => self.get_available_squares_pawn_black(board),
            (Figure::Rook, _) => self.get_available_squares_rook(board),
            (Figure::Knight, _) => self.get_available_squares_knight(board),
            (Figure::Bishop, _) => self.get_available_squares_bishop(board),
            (Figure::King, _) => self.get_available_squares_king(board),
            (Figure::Queen, _) => self.get_available_squares_queen(board),
        }
    }

    // Get the squares which can be reached by this piece when performing a capturing move
    pub fn get_attacked_squares(&self, board: &Board) -> HashSet<Square> {
        self.get_available_squares(&board).iter()
            .filter_map(|sq| board.check_square_for_piece(sq))
            .filter(|p| p.color != self.color)
            .map(|p| p.square)
            .collect::<HashSet<Square>>()
}

    fn get_available_squares_pawn_white(&self, board: &Board) ->  HashSet<Square> {
        let mut results = HashSet::new();
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

        // check if the pawn can hit either diagonal square
        for direction in [(1,1), (-1,1)] {
            let candidate_square = self.square.move_by(direction);
            if candidate_square.is_none() {continue;}
            match board.check_square_for_piece(&candidate_square.unwrap()) {
                Some(piece) => {
                    if piece.color != self.color {
                        results.insert(*board.get_from_square_unchecked(&candidate_square.unwrap()));
                    }
                },
                None => continue,
            }
        }

        results.clone()
    }

    fn get_available_squares_pawn_black(&self, board: &Board) ->  HashSet<Square> {
        let mut results = HashSet::new();
        let move_up_one = self.square.move_by((0,-1)); 
        if move_up_one.is_some() {
            // check there is no piece on that square
            if board.check_square_for_piece(board.get_from_square_unchecked(&move_up_one.unwrap())).is_none() {
                results.insert(*board.get_from_square_unchecked(&move_up_one.unwrap()));

                let move_up_two = move_up_one.unwrap().move_by((0,-1));
                if self.square.pos.1 == '2' && move_up_two.is_some() {
                    if board.check_square_for_piece(board.get_from_square_unchecked(&move_up_two.unwrap())).is_none() {
                        results.insert(*board.get_from_square_unchecked(&move_up_two.unwrap()));
                    }
                }
            }
        }

        // check if the pawn can hit either diagonal square
        for direction in [(1,-1), (-1,-1)] {
            let candidate_square = self.square.move_by(direction);
            if candidate_square.is_none() {continue;}
            match board.check_square_for_piece(&candidate_square.unwrap()) {
                Some(piece) => {
                    if piece.color != self.color {
                        results.insert(*board.get_from_square_unchecked(&candidate_square.unwrap()));
                    }
                },
                None => continue,
            }
        }

        results.clone()
    }

    fn get_available_squares_rook(&self, board: &Board) ->  HashSet<Square> {
        let directions = vec! [(0,1), (0, -1), (1,0), (-1, 0)];
        // check if piece can move at all

        // testing how far the rook can move in either direction
        self.get_directional_moves(board, directions)
    }
    fn get_available_squares_knight(&self, board: &Board) ->  HashSet<Square> {
        let directions = (-2..=2).cartesian_product(-2..=2)
            .filter(|sq| (sq.0 as i32).abs() + (sq.1 as i32).abs() == 3)
            .collect();
        // for more information about how knights move please see https://www.youtube.com/watch?v=gjMsHsd7N1Y
        
        self.get_directional_move(board, directions)
    }
    fn get_available_squares_bishop(&self, board: &Board) ->  HashSet<Square> {
        let directions = vec! [(1,1), (1, -1), (-1,1), (-1, -1)];
        // check if piece can move at all

        // testing how far the bishop can move in either direction
        self.get_directional_moves(board, directions)
    }
    
    fn get_available_squares_king(&self, board: &Board) ->  HashSet<Square> {
        let directions = (-1..=1).cartesian_product(-1..=1)
            .filter(|&dir| dir != (0,0))
            .collect();
        
        self.get_directional_move(board, directions)
    }
    
    fn get_available_squares_queen(&self, board: &Board) ->  HashSet<Square> {
        let directions = (-1..=1).cartesian_product(-1..=1)
            .filter(|&dir| dir != (0,0))
            .collect();
        // check if piece can move at all

        // testing how far the bishop can move in either direction
        self.get_directional_moves(board, directions)
    }

    fn get_directional_move(&self, board: &Board, directions: Vec<(i8, i8)>) -> HashSet<Square> {
        // other than get_directional_moves this method only checks a single move for any given direction. Useful for knights and kings
        // apart from that it works very much the same
        let mut result = HashSet::new();
        
        // testing how far the piece can move in either direction
        for direction in directions {
            let mut curr_square = self.square;
            if let Some(next_square) = curr_square.move_by(direction) {
                // check that the square is not occupied
                if let Some(other_piece) = board.check_square_for_piece(&next_square) {
                    match other_piece.color == self.color {
                        false => {
                            result.insert(next_square);
                        },
                        _ => {
                            continue
                        } 
                    }
                }    
                result.insert(next_square);
            }
        }

        result
    }

    fn get_directional_moves(&self, board: &Board, directions: Vec<(i8, i8)>) -> HashSet<Square> {
        // check moves in each direction step-by-step until there is no square at curr_pos + direction or the square is occupied
        let mut result = HashSet::new();
        
        // testing how far the piece can move in either direction
        for direction in directions {
            let mut curr_square = self.square;
            while let Some(next_square) = curr_square.move_by(direction) {
                // check that the square is not occupied
                if let Some(other_piece) = board.check_square_for_piece(&next_square) {
                    match other_piece.color == self.color {
                        false => {
                            result.insert(next_square);
                        },
                        _ => {} 
                    }
                    break;
                }
                
                result.insert(next_square);
                curr_square = next_square;
            }
        }

        result
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
