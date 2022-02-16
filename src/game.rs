
use core::*;

/// This module contains the (standard) chess game logic with support for contexts
mod game {

    pub struct Game<'a> {
        context: Option(Context),
        board: Board
    }

    pub struct Context<'a> {
    }

}

fn main() {
    println!("Hello from game");
}
