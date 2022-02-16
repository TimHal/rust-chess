/// The core library for rust-chess. (WIP)
pub mod parser;

pub mod core {

    use std::fmt;

    #[derive(Debug)]
    pub enum Color { Black, White }
    #[derive(Debug)]
    pub enum Figure { Pawn, Rook, Knight, Bishop, Queen, King }
    
    #[derive(Debug, Copy, Clone)]
    pub struct Square {
        pub pos: (char, char)
    }

    #[derive(Debug)]
    pub struct Board<'a> {
        pub squares: [[Square; 8]; 8],
        pub pieces: Vec<Piece<'a>>,
        pub is_valid: bool
    }

    #[derive(Debug)]
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

