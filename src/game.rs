
use core::*;

/// This module contains the (standard) chess game logic with support for contexts
mod game {
    // Where to put the game state???
    // castling_privs: set of (Piece, Piece)  (King + castling rook)
    // e_p_pawns: set of pawns

    pub struct Game<'a> {
        context: Option(Context),
        board: Board,
        moves: Vec<Move>,
    }

    pub struct Context<'a> {
    }

    pub struct GameTreeNode {
        
    }

    pub struct Move {
        piece: Piece;
        target_square: Square;
        variations: Vector<Variation>;
    }

    pub struct Variation {

    }

}
