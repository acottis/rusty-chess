use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(Debug, Clone, Copy)]
enum Colour {
    Light,
    Dark,
    None,
}

#[derive(Debug, Clone, Copy)]
struct Square {
    piece: Piece,
    colour: Colour,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square {
                piece: Piece::Pawn,
                colour: Colour::Dark,
            } => write!(f, "♟︎ "),
            Square {
                piece: Piece::Rook,
                colour: Colour::Dark,
            } => write!(f, "♜ "),
            Square {
                piece: Piece::Knight,
                colour: Colour::Dark,
            } => write!(f, "♞ "),
            Square {
                piece: Piece::Bishop,
                colour: Colour::Dark,
            } => write!(f, "♝ "),
            Square {
                piece: Piece::Queen,
                colour: Colour::Dark,
            } => write!(f, "♛ "),
            Square {
                piece: Piece::King,
                colour: Colour::Dark,
            } => write!(f, "♚ "),
            Square {
                piece: Piece::Pawn,
                colour: Colour::Light,
            } => write!(f, "♙ "),
            Square {
                piece: Piece::Rook,
                colour: Colour::Light,
            } => write!(f, "♖ "),
            Square {
                piece: Piece::Knight,
                colour: Colour::Light,
            } => write!(f, "♙ "),
            Square {
                piece: Piece::Bishop,
                colour: Colour::Light,
            } => write!(f, "♗ "),
            Square {
                piece: Piece::Queen,
                colour: Colour::Light,
            } => write!(f, "♕ "),
            Square {
                piece: Piece::King,
                colour: Colour::Light,
            } => write!(f, "♔ "),
            _ => write!(f, "☐ "),
        }
    }
}

impl Square {
    fn init(piece: Piece, colour: Colour) -> Square {
        Square {
            piece: piece,
            colour: colour,
        }
    }
}

/// BOARD CODE
pub struct Board {
    grid: [[Square; 8]; 8],
}
impl Board {
    // Set up initial board state
    pub fn init() -> Board {
        Board {
            grid: [
                [
                    Square::init(Piece::Rook, Colour::Dark),
                    Square::init(Piece::Knight, Colour::Dark),
                    Square::init(Piece::Bishop, Colour::Dark),
                    Square::init(Piece::King, Colour::Dark),
                    Square::init(Piece::Queen, Colour::Dark),
                    Square::init(Piece::Bishop, Colour::Dark),
                    Square::init(Piece::Knight, Colour::Dark),
                    Square::init(Piece::Rook, Colour::Dark),
                ],
                [Square::init(Piece::Pawn, Colour::Dark); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::Pawn, Colour::Light); 8],
                [
                    Square::init(Piece::Rook, Colour::Light),
                    Square::init(Piece::Knight, Colour::Light),
                    Square::init(Piece::Bishop, Colour::Light),
                    Square::init(Piece::King, Colour::Light),
                    Square::init(Piece::Queen, Colour::Light),
                    Square::init(Piece::Bishop, Colour::Light),
                    Square::init(Piece::Knight, Colour::Light),
                    Square::init(Piece::Rook, Colour::Light),
                ],
            ],
        }
    }

    // Draws the board state
    pub fn render(&self) {
        for col in &self.grid {
            for row in col {
                print!("{}", row);
            }
            println!();
        }
    }
}
