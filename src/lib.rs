/// The core library for rust-chess. (WIP)
pub mod parser;
pub mod pieces;
pub mod game;

pub mod core {
    
    use std::fmt;
    use std::collections::HashSet;

    #[derive(Debug, Clone, Copy, PartialEq, Hash)]
    pub enum Color { Black, White }

    #[derive(Debug, Clone, Copy, PartialEq, Hash)]
    pub enum Figure { Pawn, Rook, Knight, Bishop, Queen, King }
    
    #[derive(Debug, Clone, Copy, Eq, Hash)]
    pub struct Square {
        pub pos: (char, char)
    }

    #[derive(Debug, Clone)]
    pub struct Board {
        pub squares: [[Square; 8]; 8],
        pub pieces: Vec<Piece>,
        pub is_valid: bool
    }

    #[derive(Debug, Clone, Copy, Hash)]
    pub struct Piece {
        pub color: Color,
        pub figure: Figure,
        pub square: Square
    }


    impl Square {
        pub fn from_str(s: &str) -> Square {
            assert_eq!(s.chars().count(), 2);
            let chars: Vec<char> = s.chars().collect();
            Square{ pos: (chars[0], chars[1]) }
        }

        pub fn move_by(&self, delta: (i8,i8)) -> Option<Square> {
            let delta_f = ((self.pos.0 as i8) - ('a' as i8)) + delta.0; 
            let delta_r = ((self.pos.1 as i8) - ('1' as i8)) + delta.1;

            if delta_f >= 0 && delta_f >= 0 {
                let files: Vec<char> = ('a'..='h').collect();
                let ranks: Vec<char> = ('1'..='8').collect();
                let new_file: &char = files.get(delta_f as usize)?;
                let new_rank: &char = ranks.get(delta_r as usize)?;

                Some( Square {pos: (*new_file, *new_rank)} )
            } else {
                None
            }
        }
    }

    impl fmt::Display for Square {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}{}]", self.pos.0, self.pos.1)
        }
    }

    impl PartialEq for Square {
        fn eq(&self, other: &Self) -> bool {
            self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1
        }
    }

    impl Board {
        /// Creates a new board in standard position.
        pub fn new() -> Board {

            let files: [char; 8] = ('a'..='h').collect::<Vec<_>>().try_into().expect("Wrong size");
            let ranks: [char; 8] = ('1'..='8').collect::<Vec<_>>().try_into().expect("Wrong size");
            let mut squares: [[Square; 8]; 8] = [[Square::from_str("a1"); 8]; 8];

            for (i,f) in files.iter().enumerate() {
                for (j,r) in ranks.iter().enumerate() {
                    squares[j][i] = Square{ pos: (f.clone(), r.clone()) };
                }
            }
            Board {squares: squares, is_valid: true, pieces: Vec::new()}
        }

        pub fn get(&self, index_str: &str) -> Option<&Square> {
            // is valid notation string?
            let lowercase = index_str.to_ascii_lowercase();
            let bytes = lowercase.trim().as_bytes();
            if !bytes.len() == 2 { 
                return None 
            } else {
                let res = match (bytes[0] as char, bytes[1] as char) {
                    ('a'..='h', '1'..='8') => Some(&self.squares[(bytes[1] as u8 - b'1') as usize]
                                                                [(bytes[0] as u8 - b'a') as usize]),
                    _ => None
                };
                return res;
            }
        }

        pub fn get_unchecked(&self, index_str: &str) -> &Square {
            self.get(index_str).unwrap()
        }

        pub fn get_from_square_unchecked(&self, square: &Square) -> &Square {
            self.get_from_square(square).unwrap()
        }

        pub fn get_from_square(&self, square: &Square) -> Option<&Square> {
            self.get(&String::from_iter([square.pos.0, square.pos.1])[..])
        }
        
        pub fn get_from_coord(&self, rank_byte: u8, file_byte: u8) -> Option<&Square> {
            let rank_char = (b'a' + rank_byte) as char;
            let file_char = (b'1' + file_byte) as char;
            
            self.get(&String::from_iter([rank_char, file_char]))
        }

        pub fn get_from_tup(&self, index: (u8, u8)) -> Option<&Square> {
            self.get_from_coord(index.0, index.1)
        }


        pub fn squares_as_vec(&self) -> Vec<&Square> {
            let mut squares: Vec<&Square> = vec! [];
            for file in self.squares.iter() {
                let file: Vec<&Square> = file.iter().collect();
                for f in file {
                    squares.push(f);
                }
            }
            squares
        }

        pub fn squares_as_set(&self) -> HashSet<&Square> {
            HashSet::from_iter(self.squares_as_vec().iter().cloned())
        }
        
        pub fn get_rank_from_square(&self, square: &Square) -> HashSet<&Square> {
            let rank: Vec<&Square> = self.squares_as_vec().iter().map(|sq| *sq).filter(|&sq| sq.pos.1 == square.pos.1).collect();
            HashSet::from_iter(rank.iter().copied())
                        // self.squares_as_set().iter().map(|sq| *sq)
        }

        pub fn get_file_from_square(&self, square: &Square) -> HashSet<&Square> {
            let file: Vec<&Square> = self.squares_as_vec().iter().map(|sq| *sq).filter(|&sq| sq.pos.0 == square.pos.0).collect();
            HashSet::from_iter(file.iter().copied())
        }

        pub fn get_diag_from_square(&self, square: &Square) -> HashSet<&Square> {
            let directions = vec! [(1,1), (-1,-1), (1,-1), (-1,1)];
            let mut squares: Vec<&Square> = vec! [];
            squares.push(&self.get( &String::from_iter([square.pos.0, square.pos.1])[..]).unwrap());

            for direction in directions {
                let mut next_square = square.move_by(direction);
                while next_square.is_some() {
                    let next_pos = next_square.unwrap().pos;
                    let intermediate = &self.get( &String::from_iter([next_pos.0, next_pos.1])[..]);
                    if intermediate.is_none() {
                        break;
                    } else {
                        let next_square_board = intermediate.unwrap();
                        squares.push(next_square_board);
                        next_square = next_square_board.move_by(direction);
                    }
                }
            };
            HashSet::from_iter(squares.iter().copied())
        }

        pub fn check_square_for_piece(&self, square: &Square) -> Option<&Piece> {
            self.pieces.iter().find(|&p| p.square.pos == square.pos)
        }

        pub fn add_piece(&mut self, piece: Piece) -> &Self {
            self.pieces.push(piece);
            self
        }

        pub fn remove_piece(&mut self, piece: Piece) -> &Self {
            // find piece index in vec
            self.pieces.retain(|p| *p != piece);
            self
        }

        pub fn is_attacked(&self, piece: Piece) -> bool {
            // get pieces of other color
            // combine their possible squares
            // check if 'piece' position is in that set
            self.pieces.iter()
                .filter(|p| p.color != piece.color)
                .flat_map(|p| p.get_attacked_squares(&self))
                .find(|&sq| sq == piece.square).is_some()    
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: refactor this
            let mut rank_rev = self.squares.clone();
            // let str = String::new();
            rank_rev.reverse();
            for rank in rank_rev {
                for file in rank {
                    write!(f, "{} ", file)?;
                }
                writeln!(f, "")?;
            }
            write!(f, "")
        }
    }

    impl Piece {
    }

    impl PartialEq for Piece {
        fn eq(&self, other: &Self) -> bool {
            self.color == other.color &&
            self.figure == other.figure &&
            self.square == other.square
        }
    }

    impl Eq for Piece {}

}

