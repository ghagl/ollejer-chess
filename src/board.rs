
use crate::pieces::{
    Color::{Black, White},
    Piece,
    Pieces::{self, Bishop, King, Knight, Pawn, Queen, Rook},
};


pub fn print_board(board: &[Option<Piece>; 64]) {
    let mut chars: [char; 64] = ['E'; 64];
    for (i, piece) in board.iter().enumerate() {
        let p = match piece {
            Some(x) => match (x.color, x.piece_type) {
                (Black, Pawn) => 'p',
                (White, Pawn) => 'P',
                (Black, Rook) => 'r',
                (White, Rook) => 'R',
                (Black, Knight) => 'n',
                (White, Knight) => 'N',
                (Black, Bishop) => 'b',
                (White, Bishop) => 'B',
                (Black, Queen) => 'q',
                (White, Queen) => 'Q',
                (Black, King) => 'k',
                (White, King) => 'K',
            },
            _ => ' ',
        };
        chars[i] = p;
    }
    let mut rank = 8;
    print!("    a   b   c   d   e   f   g   h   ");
    for (i, c) in chars.iter().enumerate() {
        if i % 8 == 0 {
            print!("|\n  ---------------------------------\n{} ", rank);
            rank -= 1;
        }

        print!("| {} ", c);
    }
    print!("|");
    println!("");
}

fn get_piece_from_ascii(representation: char) -> Pieces {
    // Placeholder
    let piece: Pieces = match representation.to_ascii_lowercase() {
        'p' => Pawn,
        'n' => Knight,
        'b' => Bishop,
        'q' => Queen,
        'k' => King,
        'r' => Rook,
        _ => panic!("ERROR reading piece type!"),
    };
    piece
}


pub fn translate_tile_to_usize(move_input: &str) -> usize {
    if move_input.chars().count() != 2 {
        panic!("Wrong inputsize")
    }

    let file: usize = match move_input.chars().nth(0) {
        Some(f) => match f {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Invalid filechar")
        },
        _ => panic!("Error reading file")
    };
    let rank: usize = match move_input.chars().nth(1).unwrap().to_string().parse::<usize>() {
        Ok(x) => x,
        Err(_) => panic!("Error parsing rank"),
    };
    if rank < 0 || rank > 8 {
        panic!("Rank not valid");
    }
    if file < 0 || file > 8 {
        panic!("File not valid");
    }
    let cordinate = 8*(8-rank) + file;
    cordinate

   


}

pub struct OneDBoard {
    board: [Option<Piece>; 64],
    turn: char,
    castling: [bool; 4], //KQkq (White -> Black, King -> Queen)
    en_passant_target: Option<i8>,
    halfmove_clock: u32,
    fullmove_clock: u32,
}

impl OneDBoard {
    pub fn make_move(&mut self, origin: usize, destination: usize) {
        self.board[destination] = self.board[origin];
        self.board[origin] = None;
    }

    pub fn get_board(&self) -> [Option<Piece>; 64] {
        self.board
    }

    pub fn get_piece(&self, index: usize) -> Option<Piece> {
        self.board[index]
    }

    pub fn new_standard() -> Self {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let split: Vec<String> = fen.split(" ").map(|s| s.to_string()).collect();
        let board_str: String = split.get(0).unwrap().to_string();
        let mut board: [Option<Piece>; 64] = [None; 64];

        let mut position: usize = 0;
        for c in board_str.chars() {
            if c == '/' {
                continue;
            } else if c.is_alphabetic() {
                let piece_type = get_piece_from_ascii(c);
                if c.is_ascii_lowercase() {
                    let piece = Piece {
                        color: Black,
                        piece_type,
                    };
                    board[position] = Some(piece);
                } else if c.is_ascii_uppercase() {
                    let piece = Piece {
                        color: White,
                        piece_type,
                    };
                    board[position] = Some(piece);
                }
                position += 1;
            } else if c.is_digit(10) {
                for _ in 0..c.to_digit(10).unwrap() {
                    board[position] = None;
                    position += 1;
                }
            }
        }

        OneDBoard {
            board,
            turn: 'w',
            castling: [true; 4],
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        }
    }
    
    pub fn promote(square: i8, new_piece: Piece) {
        
    }
}



