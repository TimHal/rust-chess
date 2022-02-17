
use core::*;

/// This module contains the (standard) chess game logic with support for contexts
mod game {

    pub struct Game<'a> {
        context: Option(Context),
        board: Board,
        moves: Vec<Move>,
    }

    pub struct Context<'a> {
    }

    pub struct Move {

    }

}
