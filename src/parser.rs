use crate::core::Piece;
use crate::core::Color;
use crate::core::Square;
use crate::core::Figure;
/// This module holds the supported parsers and formats. Common formats are Fen, ACN, DCN and PGN
pub mod parser {
    
    use crate::core::Board;
    /// Returns an Option of Board if fen_string is a valid Fen-encoded position
    ///
    /// # Arguments
    ///
    /// * `fen_string` - String slice to contain the Fen-encoded position and nothing else
    ///
    /// # Examples
    ///
    /// ```
    /// let board_in_starting_position = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    pub fn parse_fen(fen_string: &str) -> Result<Board, &str> {
        let board = Board::new();

        // The 'fields' in a Fen record are separated by whitespaces. 
        let fields: Vec<&str> = fen_string.trim().split(' ').collect();

        // The first field contains the position as seen by the white player from the last to first rank.
        // However, we are parsing it from to first to the last rank, so it needs to be inverted
        let ranks = fields.iter().next().unwrap().split('/');
        for r in ranks {
            println!("{}", r);
        }


        
        for (_i, &char) in fen_string.trim().as_bytes().iter().enumerate() {
            if char == b' ' {
                break;
            }
        }

        Ok(board)
    }

    pub fn is_fen_piece_char(c: &char) -> bool {
        let valid_chars = vec!['p', 'r', 'n', 'b', 'k', 'q', 'P', 'R', 'N', 'B', 'K', 'Q'];
        valid_chars.contains(c)
    }

    // Board -> fen_str
    // Game -> fen_str

}

impl Piece<'_> {

        

    pub fn from_fen<'a>(s: &'a str, pos: &'a Square) -> Result<Piece<'a>, &'a str> {
        // check if it is a valid FEN Piece character

        let fen_char: char = s.to_ascii_lowercase().chars().next().unwrap(); 
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
        let color = match s.chars().next().unwrap() {
            'a'..='z' => Color::Black,
            _ => Color::White,
        };

        Ok(Piece { color: color, figure: figure, square: pos  })
    }

}


