use rust_chess::parser::parser::parse_fen;

#[cfg(test)]
mod tests {

    #[test]
    fn test_works() {
        assert_eq!(1, 1);
        let some = "Hello, World!";
       
    }


    #[test]
    fn create_std_board_from_fen() {
        let std_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board_in_starting_position = rust_chess::parser::parser::parse_fen(std_fen);
        assert_eq!(1, 1);
    }

        
}

