
#[cfg(test)]
mod tests {
    use rust_chess::core::Board;
    use rust_chess::core::Square;
    
    #[test]
    fn board_indexing() {
        let board = Board::new();
        assert_eq!(&Square { pos: ('a', '4') }, board.get("a4").unwrap());
        assert_ne!(&Square { pos: ('h', '1') }, board.get("a1").unwrap());
        // also check Square.from method
        assert_eq!(&Square::from_str("f3"), board.get("f3").unwrap());
        assert_eq!(&Square::from_str("f3"), board.get_from_tup((5,2)).unwrap());
        assert_eq!(&Square::from_str("a1"), board.get_from_tup((0,0)).unwrap());
    }

    #[test]
    fn board_indexing_out_of_bounds() {
        let board = Board::new();
        assert_eq!(None, board.get("j1"));
        assert_eq!(None, board.get("a9"));
    }

    #[test]
    fn board_get_file() {
        let board = Board::new();
        let file = &["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];
        let file_squares: Vec<&Square> = file.iter().map(|f| board.get(f).unwrap() ).collect();
        let src_square = board.get("b2").unwrap();
        
        assert_eq!(board.get_file_from_square(src_square), file_squares);
        let file_mixed = &["b1", "b2", "c3", "d4", "b5", "b6", "e7", "b8"];
        let file_squares_mixed: Vec<&Square> = file_mixed.iter().map(|f| board.get(f).unwrap() ).collect();
        assert_ne!(board.get_file_from_square(src_square), file_squares_mixed);
    }

    #[test]
    fn board_get_rank() {
        let board = Board::new();
        let rank = &["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"];
        let rank_squares: Vec<&Square> = rank.iter().map(|f| board.get(f).unwrap() ).collect();
        let src_square = board.get("c1").unwrap();
        
        assert_eq!(board.get_rank_from_square(src_square), rank_squares);
        let rank_mixed = &["b1", "b2", "c3", "d4", "b5", "b6", "e7", "b8"];
        let rank_squares_mixed: Vec<&Square> = rank_mixed.iter().map(|f| board.get(f).unwrap() ).collect();
        assert_ne!(board.get_rank_from_square(src_square), rank_squares_mixed);
    }
}