use rust_chess::parser::parse_fen;

#[cfg(test)]
mod tests {

    #[test]
    fn create_std_board_from_fen() {
        let std_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board_in_starting_position = rust_chess::parser::parse_fen(std_fen);
        assert_eq!(1, 1);
   }

   #[test]
   fn create_test_position_from_fen() {
    let testfen = "r4k1r/1p1n1ppp/2p2n2/p7/P2qP3/2NP1Q2/1PP2RPP/R5K1 w - - 0 1";
    let board_in_test_position = rust_chess::parser::parse_fen(testfen);
    // https://lichess.org/editor/r4k1r/1p1n1ppp/2p2n2/p7/P2qP3/2NP1Q2/1PP2RPP/R5K1_w_-_-_0_1
    dbg!(board_in_test_position);
    assert_eq!(1, 1);
   }


        
}

