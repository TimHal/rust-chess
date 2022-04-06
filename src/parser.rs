/// This module holds the supported parsers and formats. Common formats are Fen, ACN, DCN and PGN

use crate::core::Figure;    
use crate::core::Board;
use crate::core::Piece;
use crate::core::Square;
use crate::core::Color;
/// Returns an Option of Board if fen_string is a valid Fen-encoded position
///
/// # Arguments
///
/// * `fen_string` - String slice to contain the Fen-encoded position and nothing else
///
/// # Examples
///
/// ```
/// let board_in_starting_position = rust_chess::parser::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
/// ```
pub fn parse_fen(fen_string: &str) -> Result<Board, &str> {
    let board = &mut Board::new();

    // The 'fields' in a Fen record are separated by whitespaces. 
    let fields: Vec<&str> = fen_string.trim().split(' ').collect();
    let pieces_vec = &mut vec! [];

    // The first field contains the position as seen by the white player from the last to first rank.
    // However, we are parsing it from to first to the last rank, so it needs to be inverted
    let mut ranks: Vec<&str> = fields.iter().next().unwrap().split('/').collect();
    ranks.reverse();
    for (r_index,r) in ranks.iter().enumerate() {
        let rank_as_bytes = r.as_bytes();
        // make sure they are all valid as fen-chars
        // can not use enumerate here to skip on 1..8 
        let mut f_index = 0;
        for byte in rank_as_bytes {
            if !is_fen_piece_char(*byte as char) { 
                return  Err("FEN Parsing Error");
            };

            match *byte as char {
                '1'..='8' => { 
                    f_index += *byte - b'1'; 
                    continue; 
                },
                _ => ()
            } 
            // TODO: if *byte in 1..8 skip as many

            // let piece = Piece{color: Color::White, figure: Piece::from_fen(byte).unwrap(), square: board.get_from_coord(r_index as u8, f_index as u8).unwrap()};
            let square = board.get_from_coord(f_index as u8, r_index as u8).unwrap();
            let piece = Piece::from_fen(*byte as char, square).unwrap();
            pieces_vec.push(piece);
            f_index += 1;
        };
    };
    for &piece in pieces_vec.iter() {
        // board.pieces.push(piece.unwrap());
        board.add_piece(piece);
    }
    
    Ok(board.clone())
}



pub fn is_fen_piece_char(c: char) -> bool {
    let valid_chars = vec!['p', 'r', 'n', 'b', 'k', 'q', 'P', 'R', 'N', 'B', 'K', 'Q', '1', '2', '3', '4', '5', '6', '7', '8'];
    valid_chars.contains(&c)
}

/// Tokenizer:
/// Parse a string representation of a move into tokens which can be interpreted by the game 
/// The simples way to do this: [start_square]-[end_square]
/// More advanced methods will follow

pub enum Token {Piece, Square, Takes, Castles, CastlesSide, EnPassant 
}


// Board -> fen_str
// Game -> fen_str

impl Piece {

//                                                        .::.
//                                             _()_       _::_
//                                   _O      _/____\_   _/____\_
//            _  _  _     ^^__      / //\    \      /   \      /
//           | || || |   /  - \_   {     }    \____/     \____/
//           |_______| <|    __<    \___/     (____)     (____)
//     _     \__ ___ / <|    \      (___)      |  |       |  |
//    (_)     |___|_|  <|     \      |_|       |__|       |__|
//   (___)    |_|___|  <|______\    /   \     /    \     /    \
//   _|_|_    |___|_|   _|____|_   (_____)   (______)   (______)
//  (_____)  (_______) (________) (_______) (________) (________)
//  /_____\  /_______\ /________\ /_______\ /________\ /________\
//                                              __By Alefith 22.02.95__

// Chess pieces by Joan G. Stark

//                                                      _:_
//                                                     '-.-'
//                                            ()      __.'.__
//                                         .-:--:-.  |_______|
//                                  ()      \____/    \=====/
//                                  /\      {====}     )___(
//                       (\=,      //\\      )__(     /_____\
//       __    |'-'-'|  //  .\    (    )    /____\     |   |
//      /  \   |_____| (( \_  \    )__(      |  |      |   |
//      \__/    |===|   ))  `\_)  /____\     |  |      |   |
//     /____\   |   |  (/     \    |  |      |  |      |   |
//      |  |    |   |   | _.-'|    |  |      |  |      |   |
//      |__|    )___(    )___(    /____\    /____\    /_____\
//     (====)  (=====)  (=====)  (======)  (======)  (=======)
//     }===={  }====={  }====={  }======{  }======{  }======={
// jgs(______)(_______)(_______)(________)(________)(_________)
// https://www.asciiart.eu/sports-and-outdoors/chess


    pub fn to_symbol(&self) -> String {
        let res = match (self.figure, self.color) {
            (Figure::Pawn, Color::White) => "♙",
            (Figure::Rook, Color::White) => "♖",
            (Figure::Knight, Color::White) => "♘",
            (Figure::Bishop, Color::White) => "♗",
            (Figure::Queen, Color::White) => "♕",
            (Figure::King, Color::White) => "♔",
            (Figure::Pawn, Color::Black) => "♟",
            (Figure::Rook, Color::Black) => "♜",
            (Figure::Knight, Color::Black) => "♞",
            (Figure::Bishop, Color::Black) => "♝",
            (Figure::Queen, Color::Black) => "♛",
            (Figure::King, Color::Black) => "♚"
            
        }; 

        String::from(res)
    }

    pub fn to_fen_letter(&self) -> String {
        let res = match self.figure {
            Figure::Pawn => "p",
            Figure::Rook => "r",
            Figure::Knight => "n",
            Figure::Bishop => "b",
            Figure::Queen => "q",
            Figure::King => "k"
        }; 

        if self.color == Color::White {
            res.to_uppercase()
        } else {
            String::from(res)
        }
    }

    pub fn from_fen(s: char, pos: &Square) -> Result<Piece, &str> {
        // check if it is a valid FEN Piece character

        let fen_char: char = s.to_ascii_lowercase(); 
        let figure = match fen_char {
            'p' => Figure::Pawn,
            'r' => Figure::Rook,
            'n' => Figure::Knight,
            'b' => Figure::Bishop,
            'q' => Figure::Queen,
            'k' => Figure::King,
            _ => return Err("Checked Fen char could not be parsed. Aborting.")
        };

        // it is safe to use unwrap here once the fen checking is done
        let color = match s {
            'a'..='z' => Color::Black,
            _ => Color::White,
        };

        Ok(Piece { color: color, figure: figure, square: *pos  })
    }


}


