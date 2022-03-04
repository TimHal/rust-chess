use rust_chess::core::{Board, Square, Color, Figure, Piece};
use std::collections::HashSet;

mod common;

#[test]
fn board_indexing() {
    let board = Board::new();
    assert_eq!(&Square { pos: ('a', '4') }, board.get("a4").unwrap());
    assert_ne!(&Square { pos: ('h', '1') }, board.get("a1").unwrap());
    // also check Square.from method
    assert_eq!(&Square::from_str("f3"), board.get("f3").unwrap());
    assert_eq!(&Square::from_str("f3"), board.get_from_tup((5,2)).unwrap());
    assert_eq!(&Square::from_str("a1"), board.get_from_tup((0,0)).unwrap());
    
    let square = &Square::from_str("a3");
    assert_eq!(square, board.get_from_square(square).unwrap());
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
    let board_file = &board.get_file_from_square(board.get_unchecked("b2"));
    
    let file = &["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];
    let file_squares: HashSet<&Square> = file.iter().map(|f| board.get(f).unwrap() ).collect();
    let diff: HashSet<&Square> = file_squares.symmetric_difference(board_file).cloned().collect();
    assert_eq!(true, diff.is_empty());

    let file_mixed = &["b1", "b2", "c3", "d4", "b5", "b6", "e7", "b8"];
    let file_squares_mixed: HashSet<&Square> = file_mixed.iter().map(|f| board.get(f).unwrap() ).collect();
    let diff: HashSet<&Square> = file_squares_mixed.symmetric_difference(board_file).cloned().collect();
    assert_eq!(6, diff.len());
}

#[test]
fn board_get_rank() {
    let board = Board::new();
    let board_rank = &board.get_rank_from_square(board.get_unchecked("c1"));

    let rank = &["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"];
    let rank_squares: HashSet<&Square> = rank.iter().map(|f| board.get(f).unwrap() ).collect();
    let diff: HashSet<&Square> = rank_squares.symmetric_difference(board_rank).cloned().collect();
    assert_eq!(true, diff.is_empty());
    
    let rank_mixed = &["b1", "b2", "c3", "d4", "b5", "b6", "e7", "b8"];
    let rank_squares_mixed: HashSet<&Square> = rank_mixed.iter().map(|f| board.get(f).unwrap() ).collect();
    let diff: HashSet<&Square> = rank_squares_mixed.symmetric_difference(board_rank).cloned().collect();
    assert_eq!(14, diff.len());
}

#[test]
fn board_get_diag() {
    let board = Board::new();
    let board_diag = &board.get_diag_from_square(board.get_unchecked("a1"));

    let diag = &["a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8"];
    let diag_squares: HashSet<&Square> = diag.iter().map(|d| board.get(d).unwrap() ).collect();
    let diff: HashSet<&Square> = diag_squares.symmetric_difference(board_diag).cloned().collect();
    assert_eq!(true, diff.is_empty());

    let board_diag = &board.get_diag_from_square(board.get_unchecked("e7"));
    let diag = &["a3", "b4", "c5", "d6", "e7", "f8", "d8", "f6", "g5", "h4"];
    let diag_squares: HashSet<&Square> = diag.iter().map(|d| board.get(d).unwrap() ).collect();
    let diff: HashSet<&Square> = diag_squares.symmetric_difference(board_diag).cloned().collect();
    assert_eq!(true, diff.is_empty());
}

#[test]
fn square_arithmetics() {
    let board = Board::new();
    let square = board.get_unchecked("e2");
    assert_eq!(square.pos, square.move_by((0,0)).unwrap().pos);
    assert_eq!(board.get_unchecked("f2").pos, square.move_by((1,0)).unwrap().pos);
    assert_eq!(board.get_unchecked("a8").pos, square.move_by((-4,6)).unwrap().pos);
}

#[test]
fn board_check_square_for_piece() {
    let board = &mut Board::new();
    let pieces = vec! [Piece {color: Color::Black, figure: Figure::Rook, square: *board.get_unchecked("a3")},
                        Piece {color: Color::White, figure: Figure::King, square: *board.get_unchecked("a8")},
                        Piece {color: Color::Black, figure: Figure::Pawn, square: *board.get_unchecked("a7")},
                        Piece {color: Color::White, figure: Figure::King, square: *board.get_unchecked("e6")}];
    
    for &piece in pieces.iter() {
        board.pieces.push(piece);
    }

    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("a3")).is_some());
    assert_eq!(false, board.check_square_for_piece(board.get_unchecked("b2")).is_some());
    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("b2")).is_none());
    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("a8")).is_some());
    assert_eq!(false, board.check_square_for_piece(board.get_unchecked("h1")).is_some());
    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("a7")).is_some());
    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("e6")).is_some());
    assert_eq!(pieces.len(), board.pieces.len());
}

#[test]
fn add_and_remove_pieces() {
    let board = &mut Board::new();
    let piece = Piece {color: Color::White, figure: Figure::King, square: *board.get_unchecked("b3")};
    
    board.add_piece(piece);
    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("b3")).is_some());
    board.remove_piece(piece);
    assert_eq!(true, board.check_square_for_piece(board.get_unchecked("b3")).is_none());
    assert_eq!(0, board.pieces.len());
}

#[test]
fn pieces_pawn_available_squares() {
    let board = &mut Board::new();
    let pieces_and_moves = vec! [ 
        // free pawn on starting position with no pieces to capture
        (Piece {color: Color::White, figure: Figure::Pawn, square: *board.get_unchecked("a2")}, 
        HashSet::from([*board.get_unchecked("a3"), *board.get_unchecked("a4")])),
        // pawn blocked by another pawn with no pieces to capture
        (Piece {color: Color::White, figure: Figure::Pawn, square: *board.get_unchecked("d3")}, 
        HashSet::from([])),
        // free pawn not on starting position with no pieces to capture
        (Piece {color: Color::White, figure: Figure::Pawn, square: *board.get_unchecked("d4")}, 
        HashSet::from([*board.get_unchecked("d5")])),
        // pawn in starting position where another pawn blocks the two-square move
        (Piece {color: Color::White, figure: Figure::Pawn, square: *board.get_unchecked("f2")}, 
        HashSet::from([*board.get_unchecked("f3")])),
        // pawn on non-starting position with the option to capture on g5
        (Piece {color: Color::White, figure: Figure::Pawn, square: *board.get_unchecked("f4")}, 
        HashSet::from([*board.get_unchecked("f5"), *board.get_unchecked("g5")])),
        // free pawn with capture
        (Piece {color: Color::Black, figure: Figure::Pawn, square: *board.get_unchecked("g5")}, 
        HashSet::from([*board.get_unchecked("g4"), *board.get_unchecked("f4")])),
                        ];

    for piece in pieces_and_moves.iter() {
        board.pieces.push(piece.0);
    }

    // pawn from its starting square should be able to move one or two squares up
    for curr in pieces_and_moves {
        assert_eq!(curr.0.get_available_squares(board), curr.1);
    }
}

#[test]
fn pieces_rook_available_squares() {
    let mut board = common::empty_board();
    let rook = Piece {color: Color::White, figure: Figure::Rook, square: *board.get_unchecked("d4")};
    let rook_2 = Piece {color: Color::White, figure: Figure::Rook, square: *board.get_unchecked("d5")};
    let rook_3 = Piece {color: Color::Black, figure: Figure::Rook, square: *board.get_unchecked("d3")};
    board.pieces.push(rook);
    board.pieces.push(rook_2);
    board.pieces.push(rook_3);
    dbg!(rook.get_available_squares(&board));
}
#[test]
fn pieces_knight_available_squares() {
    assert_eq!(1,1)
}
#[test]
fn pieces_bishop_available_squares() {
    assert_eq!(1,1)
}
#[test]
fn pieces_queen_available_squares() {
    assert_eq!(1,1)
}

#[test]
fn pieces_king_available_squares() {
    assert_eq!(1,1)
}

#[test]
fn is_checkmate() {
    assert_eq!(1,1)
}

#[test]
fn checked_king_available_squares() {
    assert_eq!(1,1)
}

#[test]
fn pinned_piece_cannot_move() {
    assert_eq!(1,1)
}
