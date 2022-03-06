use crate::core::{Board, Piece, Color, Color::*, Square, Figure::*};

use std::collections::hash_set::HashSet;
use itertools::Itertools;


    // Where to put the game state???
    // castling_privs: set of (Piece, Piece)  (King + castling rook)
    // e_p_pawns: set of pawns

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
    ckeck: Option<&Piece>, // can or can not be a king in check
    castling_privileges: HashSet<(&Piece, &Piece)>, 
    possible_en_passant_moves: HashSet<(&Piece, &Square)>, // the pawn to take en-passant and the target square
}

pub enum GameResult { WhiteWin, BlackWin, Draw }

pub struct GameMeta {
    // player info, year, place, tournament, player ratings etc etc
}

pub struct GameTreeNode {
    
}

pub struct Move {
    piece: Piece,
    target_square: Square,
    variations: Vec<Variation>
}

pub struct MoveMeta {
    // move time, comments, engine evaluation etc
}

pub struct Variation {
    
}

impl Game {

    pub fn new() -> Self {

        let mut board = Board::new();
        let mut state = State::new(&board);
        let mut meta = GameMeta::new();
        let mut moves: Vec<Move> = vec! [];

        Game {board: board, state: state, meta: Some(meta), moves: moves}
    }

    pub fn make_move(&self, move_: &Move) -> Result<&Self> {
        Ok(self)
    }

    pub fn get_moves(&self, color: Color) -> HashSet<Move> {
        
        let mut result = HashSet::new();
        // get available moves for all pieces
        

        // extend with castling moves

        // extend with en-passant moves

        // extend with promotion moves
    }

}

impl State {

    pub fn new(board: &Board) -> Self {
        
        let mut possible_en_passant_moves: HashSet<(&Piece, &Square)> = HashSet::new();
        let mut castling_privileges: HashSet<(&Piece, &Piece)> = board.pieces.iter()
                .filter(|p| p.figure == King || p.figure == Rook) // get the kings and rooks
                .tuple_combinations()
                .filter(|comb| (comb.0.figure == King || comb.1.figure == King) && (comb.0.color == comb.1.color))
                .collect();

        State { turn: White, 
            result: None, 
            check: None, 
            castling_privileges: castling_privileges, 
            possible_en_passant_moves: possible_en_passant_moves }
    }

}

impl GameMeta {

    pub fn new() -> Self {

    }

}

