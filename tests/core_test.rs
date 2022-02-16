
#[cfg(test)]
mod tests {
    use rust_chess::core::Board;
    use rust_chess::core::Square;

    #[test]
    fn board_indexing() {
        let board = Board::new();
        assert_eq!(&Square { pos: ('a', '4') }, board.get("a4").unwrap());
        assert_ne!(&Square { pos: ('h', '1') }, board.get("a1").unwrap());
    }

    #[test]
    fn board_indexing_out_of_bounds() {
        let board = Board::new();
        assert_eq!(None, board.get("j1"));
        assert_eq!(None, board.get("a9"));
    }
}