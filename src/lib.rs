/// The core library for rust-chess. (WIP)
pub mod parser;

pub mod core {

    use std::fmt;
    use std::collections::HashSet;

    #[derive(Debug, Clone, Copy)]
    pub enum Color { Black, White }

    #[derive(Debug, Clone, Copy)]
    pub enum Figure { Pawn, Rook, Knight, Bishop, Queen, King }
    
    #[derive(Debug, Clone, Copy, Eq, Hash)]
    pub struct Square {
        pub pos: (char, char)
    }

    #[derive(Debug, Clone)]
    pub struct Board<'a> {
        pub squares: [[Square; 8]; 8],
        pub pieces: Vec<Piece<'a>>,
        pub is_valid: bool
    }

    #[derive(Debug, Clone)]
    pub struct Piece<'a> {
        pub color: Color,
        pub figure: Figure,
        pub square: &'a Square
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

    // Not a good idea, better use move_by
    //
    // impl std::ops::Add for Square {
    //     type Output = Self; 
    //     fn add(self, other: Self) -> Self {
    //         Self {pos: ((self.pos.0 as u8 + other.pos.0 as u8) as char, 
    //                     (self.pos.1 as u8 + other.pos.1 as u8) as char)}
    //     }
    // }

    // impl std::ops::Add<(i8, i8)> for Square {
    //     type Output = Self;
    //     fn add(self, other: (i8, i8)) -> Self {
    //         let delta_0 = other.0 + (self.pos.0 as i8) - ('a' as i8);
    //         let delta_1 = other.1 + (self.pos.1 as i8) - ('a' as i8);
    //         self
    //     }
    // }

    

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

    impl<'a> Board<'a> {
        /// Creates a new board in standard position.
        pub fn new() -> Board<'a> {

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
        
        pub fn get_rank_from_square(&self, square: &Square) -> Vec<&Square> {
            let rank = self.squares_as_vec().iter().map(|sq| *sq).filter(|&sq| sq.pos.1 == square.pos.1).collect();
            rank
        }

        pub fn get_file_from_square(&self, square: &Square) -> Vec<&Square> {
            let rank = self.squares_as_vec().iter().map(|sq| *sq).filter(|&sq| sq.pos.0 == square.pos.0).collect();
            rank
        }

        pub fn get_diag_from_square(&self, square: &Square) -> Vec<&Square> {
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
            }

            squares
        }

        

    }

    // This is too dangerous because Index can not return Option, use get instead.
    // impl<'a> std::ops::Index<&str> for Board<'a> {
    //     type Output = Option<Square>;
    //     // type Output = Square;
    //     fn index(&self, index: &str) -> a Option<Square> {
    //         Some(self.squares[4][4])
    //     }
    // }

    impl<'a> fmt::Display for Board<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut rank_rev = self.squares.clone();
            rank_rev.reverse();
            for rank in rank_rev {
                for file in rank {
                    write!(f, "{} ", file);
                }
                writeln!(f, "");
            }
            write!(f, "")
        }
    }

    impl Piece<'_> {
    }

}

