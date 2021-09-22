use crate::pieces::{
    Color::{self, Black, White},
    Piece,
    Pieces::{self, Bishop, King, Knight, Pawn, Queen, Rook},
};

pub fn print_board(board_struct: &OneDBoard) {
    let board = board_struct.get_board();
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

pub fn translate_tile_to_usize(move_input: &str) -> Result<usize, &'static str> {
    if move_input.chars().count() != 2 {
        return Err("Invalid size of move_input");
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
            _ => return Err("Invalid filechar"),
        },
        _ => return Err("Error parsing file"),
    };
    let rank: usize = match move_input
        .chars()
        .nth(1)
        .unwrap()
        .to_string()
        .parse::<usize>()
    {
        Ok(x) => x,
        Err(e) => return Err("Error parsing file"),
    };
    if rank < 0 || rank > 8 {
        return Err("Rank not valid");
    }
    if file < 0 || file > 8 {
        return Err("File not valid");
    }
    let cordinate = 8 * (8 - rank) + file;
    Ok(cordinate)
}

#[derive(Clone, Copy, Debug)]
pub struct OneDBoard {
    board: [Option<Piece>; 64],
    turn: Color,
    castling: [bool; 4], //KQkq (White -> Black, King -> Queen)
    en_passant_target: Option<usize>,
    halfmove_clock: u32,
    fullmove_clock: u32,
    previous_turn_board: [Option<Piece>; 64],
}

impl OneDBoard {
    pub fn make_move(&mut self, origin: usize, destination: usize) -> Result<(), &'static str> {
        let unmoved_state = self.board;
        let origin_piece = self.board[origin];

        let origin_piece: Piece = match origin_piece {
            Some(p) => p,
            None => return Err("No piece on that tile!"),
        };

        let turn = self.turn;
        let result = match turn {
            turn if turn == origin_piece.color => Ok(()),
            _ => return Err("Not this colors turn!"),
        };

        // Todo Adapt for special moves (Promotion, castle, en passant)
        self.board[destination] = Some(origin_piece);
        self.board[origin] = None;

        self.turn = match turn {
            White => Black,
            Black => White,
        };

        self.previous_turn_board = unmoved_state;
        result
    }

    // Made after make move
    pub fn unmake_move(&mut self) -> Result<(), &'static str> {
        self.board = self.previous_turn_board;

        let turn = self.turn;
        self.turn = match turn {
            White => Black,
            Black => White,
        };
        Ok(())
    }
    
    pub fn get_board(&self) -> &[Option<Piece>; 64] {
        &self.board
    }

    pub fn get_turn(&self) -> Color{
        self.turn 
    }

    pub fn set_piece_UNSAFE(&mut self, position: usize, piece: Option<Piece>) {
        self.board[position] = piece;
    }

    pub fn get_piece(&self, index: usize) -> Option<Piece> {
        self.board[index]
    }

    pub fn get_en_passant_target(&self) -> Option<usize> {
        self.en_passant_target
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
            turn: White,
            castling: [true; 4],
            en_passant_target: Some(8), // Change to None
            halfmove_clock: 0,
            fullmove_clock: 1,
            previous_turn_board: board,
        }
    }

    pub fn promote(square: i8, new_piece: Piece) {}
}
