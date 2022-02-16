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
        pos: (char, char)
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
    }

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

