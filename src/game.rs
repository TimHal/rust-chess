use crate::core::{Board, Piece, Color, Color::*, Square, Figure};

use std::collections::hash_set::HashSet;
use std::fmt::Display;
use itertools::Itertools;

pub struct GameBuilder {
    // ?
}

pub struct Game {
    meta: Option<GameMeta>,
    board: Board,
    state: State,
    moves: Vec<Move>,
}

pub struct State {
    // en-passant, castling etc is all done via the game, not the pieces or board as it requires knowledge about game state
    turn: Color, // the color to make the next move
    result: Option<GameResult>,
    check: bool, 
    castling_privileges: HashSet<(Piece, Piece)>, 
    possible_en_passant_moves: HashSet<(Piece, Square)>, // the pawn to take en-passant and the target square
}

pub enum GameResult { WhiteWin, BlackWin, Draw }

pub struct GameMeta {
    // player info, year, place, tournament, player ratings etc etc
}

pub struct GameTreeNode {
    
}

pub struct MoveBuilder {

}
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Move {
    piece: Piece,
    target_square: Square,
    // variations: Vec<Variation>
}

pub struct MoveMeta {
    // flags, move time, comments, engine evaluation etc
}

pub struct Variation {
    
}

impl Game {

    pub fn new() -> Self {

        let board = Board::new_in_standard_position();
        let state = State::new(&board);
        let meta = GameMeta::new();
        let moves: Vec<Move> = vec! [];

        Game {board: board, state: state, meta: Some(meta), moves: moves}
    }

    pub fn make_move<'a>(&'a mut self, move_: &Move) -> &'a mut Self {
        // check move validity? what to do if move invalid or board game in finished state? 

        // promotion? en-passant? castles? 
        
        // remove pieces from source and target square and add moving piece 
        // this also works for non-capturing moves (where there is no piece on the target square)
        self.board
            .remove_piece_by_square(&move_.piece.square)
            .remove_piece_by_square(&move_.target_square)
            .add_piece( Piece {square: move_.target_square, ..move_.piece} );
      

        // recalculate checks, privileges etc
        self.state.turn = self.next_color();
        self.state.check = self.in_check();

        self
    }

    pub fn get_moves(&self, color: Color) -> HashSet<Move> {
        
        let mut result: HashSet<Move> = HashSet::new();
        
        for piece in self.board.pieces.iter().filter(|p| p.color == color).collect::<Vec<&Piece>>() {
            for target_square in &piece.get_available_squares(&self.board) {
                result.insert(Move {piece: *piece, target_square: *target_square});
            }
        }
        
        result
        // get available moves for all pieces
        

        // extend with castling moves

        // extend with en-passant moves

        // extend with promotion moves

        // set move meta information, if applicable
    }

    pub fn in_check(&self) -> bool {
        // get king to move 
        let curr_king = self.board.pieces.iter()
            .find(|&p| 
                    p.color == self.state.turn && 
                    p.figure == Figure::King)
            .unwrap();
        
        self.board.is_attacked(*curr_king)
    }

    pub fn in_checkmate(&self) -> bool {
        // curr_king is in check
        // no possible move can end the check 
        
        false
    }

    pub fn in_stale_mate(&self) -> bool {
        // curr player has no valid moves 

    }

    fn next_color(&self) -> Color {
        match self.state.turn {
            Color::White => Color::Black,
            _ => Color::White 
        }
    }

}

impl State {

    pub fn new(board: &Board) -> Self {
        
        let possible_en_passant_moves: HashSet<(Piece, Square)> = HashSet::new();
        let mut castling_privileges: HashSet<(Piece, Piece)> = HashSet::new();
        for rook in board.pieces.iter()
                        .filter(|&p| p.figure == Figure::Rook) {
            let king = board.pieces.iter().find(|&k| k.color == rook.color && k.figure == Figure::King).unwrap();
            castling_privileges.insert((*king, *rook));
        }
        // let mut castling_privileges: HashSet<(Piece, Piece)> = board.pieces.iter()
        //         .filter(|p| p.figure == King || p.figure == Rook) // get the kings and rooks
        //         .tuple_combinations()
        //         .filter(|(&a,&b)| (a.figure == King || b.figure == King) && (a.color == b.color))
        //         .collect::<HashSet<(Piece, Piece)>>();
        State { 
            turn: White, 
            result: None, 
            check: false, 
            castling_privileges: castling_privileges, 
            possible_en_passant_moves: possible_en_passant_moves }
    }

}

impl Move {

    pub fn to_str(&self) -> String {
        let str = String::new();
        
        let piece_letter = match self.piece.figure {
            Figure::Pawn => return self.target_square.to_string(),
            Figure::Rook => "R",
            Figure::Knight => "N",
            Figure::Bishop => "B",
            Figure::Queen => "Q",
            Figure::King => "K"
        };

        String::from_iter([piece_letter, &self.piece.square.to_string()[..], &self.target_square.to_string()[..]])

    }

}

impl std::fmt::Display for Move {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_letter = match self.piece.figure {
            Figure::Pawn => return write!(f, "{}", self.target_square.to_string()),
            Figure::Rook => "R",
            Figure::Knight => "N",
            Figure::Bishop => "B",
            Figure::Queen => "Q",
            Figure::King => "K"
        };
        
        write!(f, "{}{}-{}", piece_letter, self.piece.square, self.target_square)
    }
    
}

impl GameMeta {

    pub fn new() -> Self {
        GameMeta {}
    }

}

