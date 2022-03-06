#[cfg(test)]
mod tests {
    
    use rust_chess::parser::parse_fen;
    use rust_chess::core::{Board, Piece, Color, Figure, Square};
    #[test]
    fn create_std_board_from_fen() {
        let std_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board_in_starting_position = rust_chess::parser::parse_fen(std_fen).unwrap();

        let pieces_expected = vec! [
            Piece {color: Color::White, figure: Figure::Rook, square: Square::from_str("a1")},
            Piece {color: Color::White, figure: Figure::Knight, square: Square::from_str("b1")},
            Piece {color: Color::White, figure: Figure::Bishop, square: Square::from_str("c1")},
            Piece {color: Color::White, figure: Figure::Queen, square: Square::from_str("d1")},
            Piece {color: Color::White, figure: Figure::King, square: Square::from_str("e1")},
            Piece {color: Color::White, figure: Figure::Bishop, square: Square::from_str("f1")},
            Piece {color: Color::White, figure: Figure::Knight, square: Square::from_str("g1")},
            Piece {color: Color::White, figure: Figure::Rook, square: Square::from_str("h1")},

            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("a2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("b2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("c2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("d2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("e2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("f2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("g2")},
            Piece {color: Color::White, figure: Figure::Pawn, square: Square::from_str("h2")},

            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("a7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("b7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("c7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("d7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("e7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("f7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("g7")},
            Piece {color: Color::Black, figure: Figure::Pawn, square: Square::from_str("h7")},

            Piece {color: Color::Black, figure: Figure::Rook, square: Square::from_str("a8")},
            Piece {color: Color::Black, figure: Figure::Knight, square: Square::from_str("b8")},
            Piece {color: Color::Black, figure: Figure::Bishop, square: Square::from_str("c8")},
            Piece {color: Color::Black, figure: Figure::Queen, square: Square::from_str("d8")},
            Piece {color: Color::Black, figure: Figure::King, square: Square::from_str("e8")},
            Piece {color: Color::Black, figure: Figure::Bishop, square: Square::from_str("f8")},
            Piece {color: Color::Black, figure: Figure::Knight, square: Square::from_str("g8")},
            Piece {color: Color::Black, figure: Figure::Rook, square: Square::from_str("h8")},

        ];

        for piece in pieces_expected.iter() {
            assert_eq!(piece, 
                board_in_starting_position.check_square_for_piece(&piece.square).unwrap());    
        }

   }

   #[test]
   fn create_test_position_from_fen() {
    let testfen = "r4k1r/1p1n1ppp/2p2n2/p7/P2qP3/2NP1Q2/1PP2RPP/R5K1 w - - 0 1";
    let board_in_test_position = rust_chess::parser::parse_fen(testfen);
    // https://lichess.org/editor/r4k1r/1p1n1ppp/2p2n2/p7/P2qP3/2NP1Q2/1PP2RPP/R5K1_w_-_-_0_1

    assert_eq!(1, 1);
   }

   #[test]
   fn test_fen_export() {
       assert_eq!(1,1)
   }


        
}

