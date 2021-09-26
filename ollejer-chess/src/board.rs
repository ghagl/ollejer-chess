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

    print!("|\n  ---------------------------------");
    println!("\n    a   b   c   d   e   f   g   h   ");
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
        Err(_) => return Err("Error parsing file"),
    };
    if rank > 8 {
        return Err("Rank not valid");
    }
    if file > 8 {
        return Err("File not valid");
    }
    let cordinate = 8 * (8 - rank) + file;
    Ok(cordinate)
}

fn read_promotion() -> Result<Pieces, &'static str> {
    let mut std_input = String::new();
    println!("Which piece do you want to promote to: (q, r, b, n)");
    std::io::stdin()
        .read_line(&mut std_input)
        .expect("Error reading");

    let piece_input = &std_input.chars().nth(0);

    let result: Result<Pieces, &'static str> = match piece_input.unwrap() {
        'q' => Ok(Queen),
        'n' => Ok(Knight),
        'b' => Ok(Bishop),
        'r' => Ok(Rook),
        _ => Err("Could not read promotion: {}"),
    };

    result
}

fn string_to_board(board_string: String) -> [Option<Piece>; 64] {
    let mut board: [Option<Piece>; 64] = [None; 64];

    let mut position: usize = 0;
    for c in board_string.chars() {
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
board
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
    pub fn make_move(&mut self, origin: usize, destination: usize, debug: bool) -> Result<(), &'static str> {
        let unmoved_state = self.board;
        let origin_piece = self.board[origin];

        let origin_piece: Piece = match origin_piece {
            Some(p) => p,
            None => return Err("No piece on that tile!"),
        };

        let turn = self.turn;
        let mut _result: Result<(), &'static str> = match turn {
            turn if turn == origin_piece.color => Ok(()),
            _ => return Err("Not this colors turn!"),
        };

        // Todo Adapt for special moves (Promotion, castle, en passant)
        let mut normal_move = true;
        if origin_piece.piece_type == Pawn {
            _result = self.move_pawn(origin_piece, origin, destination, debug);
        } else {
            self.en_passant_target = None;
        }

        if origin_piece.piece_type == King {
            _result = self.move_king(origin_piece, origin, destination, debug);
            normal_move = false;
        }

        if origin_piece.piece_type == Rook {
            _result = self.move_rook(origin_piece, origin, destination);
        }

        if normal_move {
            self.board[destination] = Some(origin_piece);
            self.board[origin] = None;
        }

        self.turn = match turn {
            White => Black,
            Black => White,
        };

        self.previous_turn_board = unmoved_state;
        Ok(())
    }
    fn move_rook(
        &mut self,
        rook: Piece,
        origin: usize,
        destination: usize,
    ) -> Result<(), &'static str> {

        if origin == 0 {
            self.castling[3] = false;
        }
        else if origin == 7 {
            self.castling[2] = false;
        }
        else if origin == 56 {
            self.castling[1] = false;
        }
        else if origin == 63 {
            self.castling[0] = false;
        }

        Ok(())
    }

    fn move_king(
        &mut self,
        king: Piece,
        origin: usize,
        destination: usize,
        debug: bool,
    ) -> Result<(), &'static str> {
        let m = destination as i8 - origin as i8;

        if m == 2 {
            // Short castle
            let rook_pos = destination + 1;
            let rook_dest = destination - 1;
            if (king.color == White && self.castling[0]) || (king.color == Black && self.castling[1]) {
                self.board[rook_dest] = self.board[rook_pos];
                self.board[destination] = self.board[origin];
                self.board[rook_pos] = None;
                self.board[origin] = None;
                if king.color == White {
                    self.castling[0] = false;
                    if debug {
                        println!("White castled Short");
                    }
                }
                else if king.color == Black{
                    self.castling[2] = false;
                    if debug {
                        println!("Black castled Short");
                    }
                }
            }


        } else if m == -2 {
            // Short castle
            let rook_pos = destination - 2;
            let rook_dest = destination + 1;
            if (king.color == White && self.castling[1]) || (king.color == Black && self.castling[3]) {
                self.board[rook_dest] = self.board[rook_pos];
                self.board[destination] = self.board[origin];
                self.board[rook_pos] = None;
                self.board[origin] = None;
                if king.color == White {
                    self.castling[1] = false;
                    if debug {
                        println!("White castled long");
                    }
                    

                }
                else if king.color == Black{
                    self.castling[3] = false;
                    if debug {
                        println!("Black castled long");
                    }
                    
                }
            }
            
        } else {
            self.board[destination] = self.board[origin];
            self.board[origin] = None;
        }

        for i in 0..=3 {
            self.castling[i] = false;
        }

        Ok(())
    }

    fn move_pawn(
        &mut self,
        pawn: Piece,
        origin: usize,
        destination: usize,
        debug: bool,
    ) -> Result<(), &'static str> {
        let m: i8 = destination as i8 - origin as i8;

        // Check en passant
        match self.en_passant_target {
            Some(ept) => {
                if (m == -7 || m == -9) && destination == ept {
                    // White made en passant
                    if debug {
                        println!("White made en passant. Removing: {:?}", ept);
                    }
                    self.board[ept + 8] = None;
                } else if (m == 7 || m == 9) && destination == ept {
                    // Black made en passant
                    if debug {
                        println!("Black made en passant. Removing {:?}", ept);
                    }
                    self.board[ept - 8] = None;
                }
            }
            None => (),
        }

        if m == 16 {
            // Black made passant
            // println!("Black made passant");
            let en_passant_pos = destination - 8;
            self.en_passant_target = Some(en_passant_pos);
        } else if m == -16 {
            // White made passant
            // println!("White made passant");
            let en_passant_pos = destination + 8;
            self.en_passant_target = Some(en_passant_pos);
        } else {
            self.en_passant_target = None;
        }

        let destination_rank = destination / 8;
        let result: Result<(), &'static str>;
        if pawn.color == White && destination_rank == 0 {
            result = match read_promotion() {
                Ok(piece_type) => {
                    self.promote(
                        destination,
                        Piece {
                            color: White,
                            piece_type,
                        },
                    );
                    Ok(())
                }
                Err(e) => return Err(e),
            }
        } else if pawn.color == Black && destination_rank == 7 {
            result = match read_promotion() {
                Ok(piece_type) => {
                    self.promote(
                        destination,
                        Piece {
                            color: Black,
                            piece_type,
                        },
                    );
                    Ok(())
                }
                Err(e) => return Err(e),
            };
        } else {
            result = Ok(());
        }

        Ok(())
    }

    pub fn get_board(&self) -> &[Option<Piece>; 64] {
        &self.board
    }

    pub fn get_turn(&self) -> Color {
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

    pub fn get_castles(&self) -> [bool; 4] {
        self.castling
    }
    
    
    
    pub fn new_from_FEN(fen: String) -> Self {
        let split: Vec<String> = fen.split(" ").map(|s| s.to_string()).collect();
        let board = split.get(0).unwrap().to_string();
        let board = string_to_board(board);

        let turn = split.get(1).unwrap();
        let turn = &turn[..];
        let turn = match turn {
            "w" => White,
            "b" => Black,
            _ => panic!("Error parsing FEN turn"),
        };

        let castle_avalability_str = split.get(2).unwrap().to_string();
        let mut castle_avalability: [bool; 4] = [false; 4];
        for c in castle_avalability_str.chars() {
            match c {
                'K' => castle_avalability[0] = true,
                'Q' => castle_avalability[1] = true,
                'k' => castle_avalability[2] = true,
                'q' => castle_avalability[3] = true,
                _  => (),
            }
        }

        let en_passant_target = split.get(3).unwrap();
        let en_passant_target = &en_passant_target[..];
        let en_passant_target: Option<usize> = match en_passant_target {
            "-" => None,
            _ => match translate_tile_to_usize(en_passant_target) {
                Ok(pos) => Some(pos),
                Err(e) => panic!("Error rading en passant target: {}", e),
            },
        };

        let halfmove_clock = split.get(4).unwrap();
        let halfmove_clock: u32 = match halfmove_clock.parse::<u32>() {
            Ok(halfmoves) => halfmoves,
            Err(e) => panic!(e),
        };

        let fullmove_clock = split.get(4).unwrap();
        let fullmove_clock: u32 = match fullmove_clock.parse::<u32>() {
            Ok(fullmoves) => fullmoves,
            Err(e) => panic!(e),
        }; 

        OneDBoard {
            board,
            turn: White,
            castling: castle_avalability,
            en_passant_target,
            halfmove_clock,
            fullmove_clock,
            previous_turn_board: board,
        }

    }  

    pub fn new() -> Self {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let split: Vec<String> = fen.split(" ").map(|s| s.to_string()).collect();
        let board_str: String = split.get(0).unwrap().to_string();
        let board = string_to_board(board_str);

        OneDBoard {
            board,
            turn: White,
            castling: [true; 4],
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
            previous_turn_board: board,
        }
    }

    pub fn promote(&mut self, position: usize, new_piece: Piece) {
        self.board[position] = Some(new_piece);
    }
}
