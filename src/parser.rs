mod parser {

    use crate::core::*;

    pub fn parse_fen(fen_string: &str) -> Option<Board> {
        let board = Board::new();

        let fields: Vec<&str> = fen_string.split(' ').collect();
        dbg!(fields);

        for (_i, &char) in fen_string.trim().as_bytes().iter().enumerate() {
            if char == b' ' {
                break;
            }
        }

        Some(board)
    }
}
