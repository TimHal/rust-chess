use crate::core::{Board, Piece, Color, Color::*, Square, Figure::*};

use std::collections::hash_set::HashSet;
use itertools::Itertools;


pub struct Game<'a> {
    meta: Option<GameMeta>,
    board: Board,
    state: State<'a>,
    moves: Vec<Move>,
}

pub struct State<'a> {
    // en-passant, castling etc is all done via the game, not the pieces or board as it requires knowledge about game state
    turn: Color, // the color to make the next move
    result: Option<GameResult>,
    check: Option<&'a Piece>, // can or can not be a king in check
    castling_privileges: HashSet<(&'a Piece, &'a Piece)>, 
    possible_en_passant_moves: HashSet<(&'a Piece, &'a Square)>, // the pawn to take en-passant and the target square
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

impl Game<'_> {

    pub fn new() -> Self {

        let board = Board::new();
        let state = State::new(&board);
        let meta = GameMeta::new();
        let moves: Vec<Move> = vec! [];

        Game {board: board, state: state, meta: Some(meta), moves: moves}
    }

    pub fn make_move(&self, move_: &Move) -> Result<&Self, &str> {
        Ok(self)
    }

    pub fn get_moves(&self, color: Color) -> HashSet<Move> {
        
        let result: HashSet<Move> = HashSet::new();
        result
        // get available moves for all pieces
        

        // extend with castling moves

        // extend with en-passant moves

        // extend with promotion moves
    }

}

impl State<'_> {

    pub fn new(board: &Board) -> Self {
        
        let possible_en_passant_moves: HashSet<(&Piece, &Square)> = HashSet::new();
        let castling_privileges: HashSet<(&Piece, &Piece)> = HashSet::new();
        // let mut castling_privileges: HashSet<(&Piece, &Piece)> = board.pieces.iter<'a, &Piece>()
        //         .filter(|p| p.figure == King || p.figure == Rook) // get the kings and rooks
        //         .tuple_combinations()
        //         .filter(|(a,b)| (a.figure == King || b.figure == King) && (a.color == b.color))
        //         .collect();
        State { turn: White, result: None, check: None, 
            castling_privileges: castling_privileges, 
            possible_en_passant_moves: possible_en_passant_moves }
    }

}

impl GameMeta {

    pub fn new() -> Self {
        GameMeta {}
    }

}

