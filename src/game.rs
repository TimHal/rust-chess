use crate::core::{Board, Piece, Color, Color::*, Square, Figure};

use std::collections::hash_set::HashSet;
use std::fmt::Display;
use itertools::Itertools;

pub struct GameBuilder {
    // ?
}

#[derive(Clone)]
pub struct Game{
    pub meta: Option<GameMeta>,
    pub board: Board,
    pub state: State,
    pub moves: Vec<Move>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct State {
    // en-passant, castling etc is all done via the game, not the pieces or board as it requires knowledge about game state
    turn: Color, // the color to make the next move
    result: Option<GameResult>,
    check: bool, 
    castling_privileges: HashSet<(Piece, Piece)>, 
    possible_en_passant_moves: HashSet<(Piece, Square)>, // the pawn to take en-passant and the target square
}

#[derive(Clone, Eq, PartialEq)]
pub enum GameResult { WhiteWin, BlackWin, Draw }

#[derive(Clone, Eq, PartialEq)]
pub struct GameMeta {
    // player info, year, place, tournament, player ratings etc etc
}

pub struct GameTreeNode {
    
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Move {
    piece: Piece,
    target_square: Square,
    // variations: Vec<Variation>
}

pub struct MoveMeta {
    // flags, move time, comments, engine evaluation etc
}

pub enum MoveMetaFlag {EnPassant, Castling, Promotion}

pub struct Variation {
    
}

impl Game {

    pub fn new() -> Self {

        let board = Board::new_in_standard_position();
        let state = State::new(&board);
        let meta = GameMeta::new();
        let moves: Vec<Move> = Vec::new();

        Game {board: board, state: state, meta: Some(meta), moves: moves}
    }

    pub fn make_move(&mut self, move_: Move) -> &mut Self {
        // check move validity? what to do if move invalid or board game in finished state? 

        // promotion? castles? 
        // in the case for a promotion it is enough to replace the move_.piece by the desired promotion and proceed as usual 
        
        
        // remove pieces from source and target square and add moving piece 
        // this also works for non-capturing moves (where there is no piece on the target square)
        // works by default for en-passant 
        self.board
            .remove_piece_by_square(&move_.piece.square)
            .remove_piece_by_square(&move_.target_square)
            .add_piece( Piece {square: move_.target_square, ..move_.piece} );
      

        // recalculate checks, privileges etc
        self.state.turn = self.next_color();
        self.state.check = self.in_check();
        self.moves.push(move_);

        self
    }

    pub fn simulate_move(&mut self, move_: Move) -> Self {
        // Evlaute a move on a copy of the game instance (original game is not affected)
        let mut res = self.clone();
        res.make_move(move_);
        res 
    }

    pub fn generate_move_from_str(&self, input: &str) -> Move {
        let mut split = input.split("-");
        let start_square = split.next().unwrap();
        let end_square = split.next().unwrap();
        let piece = *self.board.check_square_for_piece(&Square::from_str(start_square)).unwrap();
        
        Move { piece: piece, target_square: Square::from_str(end_square)}
    }

    pub fn move_from_str(&mut self, input: &str) -> &mut Self {
        self.make_move(self.generate_move_from_str(input))
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
        self.in_check_color(self.state.turn)
    }

    pub fn in_check_color(&self, color: Color) -> bool {
        // checks if the provided color is in check
        let curr_king = self.board.pieces.iter()
            .find(|&p| 
                    p.color == color && 
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
        
        false
    }

    pub fn current_color(&self) -> Color {
        self.state.turn
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

