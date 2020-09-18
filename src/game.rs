use std::fmt;

const EMPTY_SQUARE: Square = Square {
    piece: Piece::None,
    colour: Colour::None,
};

#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colour {
    Light,
    Dark,
    None,
}

#[derive(Clone, Copy, PartialEq)]
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
            } => write!(f, "♟︎"),
            Square {
                piece: Piece::Rook,
                colour: Colour::Dark,
            } => write!(f, "♜"),
            Square {
                piece: Piece::Knight,
                colour: Colour::Dark,
            } => write!(f, "♞"),
            Square {
                piece: Piece::Bishop,
                colour: Colour::Dark,
            } => write!(f, "♝"),
            Square {
                piece: Piece::Queen,
                colour: Colour::Dark,
            } => write!(f, "♛"),
            Square {
                piece: Piece::King,
                colour: Colour::Dark,
            } => write!(f, "♚"),
            Square {
                piece: Piece::Pawn,
                colour: Colour::Light,
            } => write!(f, "♙"),
            Square {
                piece: Piece::Rook,
                colour: Colour::Light,
            } => write!(f, "♖"),
            Square {
                piece: Piece::Knight,
                colour: Colour::Light,
            } => write!(f, "♘"),
            Square {
                piece: Piece::Bishop,
                colour: Colour::Light,
            } => write!(f, "♗"),
            Square {
                piece: Piece::Queen,
                colour: Colour::Light,
            } => write!(f, "♕"),
            Square {
                piece: Piece::King,
                colour: Colour::Light,
            } => write!(f, "♔"),
            _ => write!(f, "☐"),
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
                    Square::init(Piece::Rook, Colour::Light),
                    Square::init(Piece::Knight, Colour::Light),
                    Square::init(Piece::Bishop, Colour::Light),
                    Square::init(Piece::Queen, Colour::Light),
                    Square::init(Piece::King, Colour::Light),
                    Square::init(Piece::Bishop, Colour::Light),
                    Square::init(Piece::Knight, Colour::Light),
                    Square::init(Piece::Rook, Colour::Light),
                ],
                [Square::init(Piece::Pawn, Colour::Light); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::None, Colour::None); 8],
                [Square::init(Piece::Pawn, Colour::Dark); 8],
                [
                    Square::init(Piece::Rook, Colour::Dark),
                    Square::init(Piece::Knight, Colour::Dark),
                    Square::init(Piece::Bishop, Colour::Dark),
                    Square::init(Piece::Queen, Colour::Dark),
                    Square::init(Piece::King, Colour::Dark),
                    Square::init(Piece::Bishop, Colour::Dark),
                    Square::init(Piece::Knight, Colour::Dark),
                    Square::init(Piece::Rook, Colour::Dark),
                ],
            ],
        }
    }

    pub fn move_piece(&mut self, colour: Colour, src: String, dst: String) -> bool {
        let src = self.str_to_coords(src).unwrap();
        let dst = self.str_to_coords(dst).unwrap();

        if self.valid_move(colour, src, dst) == true {
            println!("move was valid");
            self.grid[dst.0][dst.1] = self.grid[src.0][src.1];
            self.grid[src.0][src.1] = Square::init(Piece::None, Colour::None);
            return true;
        }
        false
    }

    // Draws the board state
    pub fn render(&self) {
        for col in &self.grid {
            for row in col {
                print!("{} ", row);
            }
            println!();
        }
    }

    fn str_to_coords(&self, coords: String) -> Option<(usize, usize)> {
        let row = coords.chars().nth(0).unwrap() as usize - 97;
        let col = coords.chars().nth(1).unwrap() as usize - 49;

        if (col < 8) && (row < 8) {
            Some((col, row))
        } else {
            panic!("Invalid move {}, {}", row, col);
            //None
        }
    }

    fn valid_move(&self, piece_colour: Colour, src: (usize, usize), dst: (usize, usize)) -> bool {
        let square = self.grid[src.0][src.1];

        // Only move your pieces
        if square.colour != piece_colour {
            println!("Not your piece");
            return false;
        }

        println!("src:{:?},dst:{:?}", src, dst);
        match square.piece {
            Piece::Pawn => return self.pawn_moves(src, piece_colour).unwrap().contains(&dst),
            Piece::Knight => return self.knight_moves(src, piece_colour).unwrap().contains(&dst),
            Piece::Bishop => return self.bishop_moves(src, piece_colour).unwrap().contains(&dst),
            Piece::Rook => return self.rook_moves(src, piece_colour).unwrap().contains(&dst),
            Piece::Queen => return self.queen_moves(src, piece_colour).unwrap().contains(&dst),
            Piece::King => return self.king_moves(src, piece_colour).unwrap().contains(&dst),
            _ => return false,
        };
    }

    // Returns a list of valid King moves
    fn king_moves(&self, src: (usize, usize), colour: Colour) -> Option<Vec<(usize, usize)>> {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let mut legal_moves : Vec<(usize, usize)> = Vec::new();

        for x in 0..3 {
            for y in 0..3 {
                if 
                src.1 + 1 >= x && 
                src.0 + 1 >= y && 
                src.1 + 1 + x < self.grid.len() && 
                src.0 + 1 + y < self.grid.len()  
                {
                    moves.push((src.0 + 1 - y, src.1 + 1 - x));
                }
            }
        }

        // Filter out illegal moves - checks for same colour peices
        for mov in moves{
            if self.grid[mov.0][mov.1].colour != colour{
                legal_moves.push(mov);
            }
        }

        println!("Legal King moves {:?}", legal_moves);
        Some(legal_moves)
    }

    // Return a list of valid queen moves
    fn queen_moves(&self, src: (usize, usize), colour: Colour) -> Option<Vec<(usize, usize)>> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        let rook_moves = self.rook_moves(src, colour).unwrap();
        let bishop_moves = self.bishop_moves(src, colour).unwrap();

        for mov in rook_moves {
            moves.push(mov);
        }
        for mov in bishop_moves {
            moves.push(mov);
        }

        Some(moves)
    }

    // Return a list of valid rook moves for a given square
    fn rook_moves(&self, src: (usize, usize), colour: Colour) -> Option<Vec<(usize, usize)>> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        // Vertical +
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds
            if src.0 + i >= self.grid.len() {
                break;
            }

            // Check for white pieces
            if self.grid[src.0 + i][src.1].colour == colour {
                break;
            }

            moves.push((src.0 + i, src.1));

            // If a black peice is there stop
            if self.grid[src.0 + i][src.1].colour != colour
                && self.grid[src.0 + i][src.1].colour != Colour::None
            {
                break;
            }
        }

        // Vertical -
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds
            if src.0 < i {
                break;
            }

            // Check for white pieces
            if self.grid[src.0 - i][src.1].colour == colour {
                break;
            }

            moves.push((src.0 - i, src.1));

            // If a black peice is there stop
            if self.grid[src.0 - i][src.1].colour != colour
                && self.grid[src.0 - i][src.1].colour != Colour::None
            {
                break;
            }
        }

        // Horizontal +
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds
            if src.1 + i >= self.grid.len() {
                break;
            }

            // Check for white pieces
            if self.grid[src.0][src.1 + i].colour == colour {
                break;
            }

            moves.push((src.0, src.1 + i));

            // If a black peice is there stop
            if self.grid[src.0][src.1 + i].colour != colour
                && self.grid[src.0][src.1 + i].colour != Colour::None
            {
                break;
            }
        }

        // Horizontal -
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds
            if src.1 < i {
                break;
            }

            // Check for white pieces
            if self.grid[src.0][src.1 - i].colour == colour {
                break;
            }

            moves.push((src.0, src.1 - i));

            // If a black peice is there stop
            if self.grid[src.0][src.1 - i].colour != colour
                && self.grid[src.0][src.1 - i].colour != Colour::None
            {
                break;
            }
        }

        println!("Rook moves: {:?}", moves);
        Some(moves)
    }

    // Return a list of valid bishop moves for a given square
    fn bishop_moves(&self, src: (usize, usize), colour: Colour) -> Option<Vec<(usize, usize)>> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        // Diagonal ++
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds squares
            if src.0 + i >= self.grid.len() || src.1 + i >= self.grid.len() {
                break;
            }

            // Check for white pieces
            if self.grid[src.0 + i][src.1 + i].colour == colour {
                break;
            }
            // adds to vector
            moves.push((src.0 + i, src.1 + i));

            // If a black peice is there stop
            if self.grid[src.0 + i][src.1 + i].colour != colour
                && self.grid[src.0 + i][src.1 + i].colour != Colour::None
            {
                break;
            }
        }

        // Diagonal --
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds squares
            if src.0 <= i - 1 || src.1 <= i - 1 {
                break;
            }

            // Check for white pieces
            if self.grid[src.0 - i][src.1 - i].colour == colour {
                break;
            }
            // adds to vector
            moves.push((src.0 - i, src.1 - i));

            // If a black peice is there stop
            if self.grid[src.0 - i][src.1 - i].colour != colour
                && self.grid[src.0 - i][src.1 - i].colour != Colour::None
            {
                break;
            }
        }

        // Diagonal -+
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds squares
            if src.0 <= i - 1 || src.1 + i >= self.grid.len() {
                break;
            }

            // Check for white pieces
            if self.grid[src.0 - i][src.1 + i].colour == colour {
                break;
            }
            // adds to vector
            moves.push((src.0 - i, src.1 + i));

            // If a black peice is there stop
            if self.grid[src.0 - i][src.1 + i].colour != colour
                && self.grid[src.0 - i][src.1 + i].colour != Colour::None
            {
                break;
            }
        }

        // Diagonal +-
        for (i, _) in self.grid.iter().enumerate() {
            if i == 0 {
                continue;
            }

            // Checks for out of bounds squares
            if src.0 + i >= self.grid.len() || src.1 <= i - 1 {
                break;
            }

            // Check for white pieces
            if self.grid[src.0 + i][src.1 - i].colour == colour {
                break;
            }
            // adds to vector
            moves.push((src.0 + i, src.1 - i));

            // If a black peice is there stop
            if self.grid[src.0 + i][src.1 - i].colour != colour
                && self.grid[src.0 + i][src.1 - i].colour != Colour::None
            {
                break;
            }
        }

        println!("Bishop moves: {:?}", moves);

        Some(moves)
    }

    // Return a list of valid knight moves for a given square
    fn knight_moves(&self, src: (usize, usize), colour: Colour) -> Option<Vec<(usize, usize)>> {
        let mut knight_moves: Vec<(usize, usize)> = Vec::new();
        let mut moves: Vec<(usize, usize)> = Vec::new();

        if src.0 < self.grid.len() - 2 && src.1 < self.grid.len() - 1 {
            knight_moves.push((src.0 + 2, src.1 + 1));
        }
        if src.0 < self.grid.len() - 2 && src.1 >= 1 {
            knight_moves.push((src.0 + 2, src.1 - 1));
        }
        if src.0 >= 2 && src.1 < self.grid.len() - 1 {
            knight_moves.push((src.0 - 2, src.1 + 1));
        }
        if src.0 >= 2 && src.1 >= 1 {
            knight_moves.push((src.0 - 2, src.1 - 1));
        }
        if src.0 < self.grid.len() - 1 && src.1 < self.grid.len() - 2 {
            knight_moves.push((src.0 + 1, src.1 + 2));
        }
        if src.0 >= 1 && src.1 < self.grid.len() - 2 {
            knight_moves.push((src.0 - 1, src.1 + 2));
        }
        if src.0 < self.grid.len() - 1 && src.1 >= 2 {
            knight_moves.push((src.0 + 1, src.1 - 2));
        }
        if src.0 >= 1 && src.1 >= 2 {
            knight_moves.push((src.0 - 1, src.1 - 2));
        }

        //Remove moves to squares occupied with your own pieces
        for mov in knight_moves {
            if self.grid[mov.0][mov.1].colour != colour {
                moves.push(mov);
            }
        }

        println!("Knight moves: {:?}", moves);
        Some(moves)
    }

    // Return a list of valid pawn moves for a given square
    fn pawn_moves(&self, src: (usize, usize), colour: Colour) -> Option<Vec<(usize, usize)>> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        if colour == Colour::Light {
            if self.grid[src.0 + 1][src.1] == EMPTY_SQUARE {
                moves.push((src.0 + 1, src.1));
            }
            if src.0 == 1
                && self.grid[src.0 + 2][src.1] == EMPTY_SQUARE
                && self.grid[src.0 + 1][src.1] == EMPTY_SQUARE
            {
                moves.push((src.0 + 2, src.1));
            }
            if src.1 < self.grid.len() - 1 {
                if self.grid[src.0 + 1][src.1 + 1].colour == Colour::Dark {
                    moves.push((src.0 + 1, src.1 + 1));
                }
            }
            if src.1 > 0 {
                if self.grid[src.0 + 1][src.1 - 1].colour == Colour::Dark {
                    moves.push((src.0 + 1, src.1 - 1));
                }
            }
        }

        if colour == Colour::Dark {
            if self.grid[src.0 - 1][src.1] == EMPTY_SQUARE {
                moves.push((src.0 - 1, src.1));
            }
            if src.0 == 6
                && self.grid[src.0 - 2][src.1] == EMPTY_SQUARE
                && self.grid[src.0 - 1][src.1] == EMPTY_SQUARE
            {
                moves.push((src.0 - 2, src.1));
            }
            if src.1 > 0 {
                if self.grid[src.0 - 1][src.1 - 1].colour == Colour::Light {
                    moves.push((src.0 - 1, src.1 - 1));
                }
            }
            if src.1 < self.grid.len() - 1 {
                if self.grid[src.0 - 1][src.1 + 1].colour == Colour::Light {
                    moves.push((src.0 - 1, src.1 + 1));
                }
            }
        }
        println!("{:?}", moves);
        Some(moves)
    }
}
