
use crate::pieces::Pieces::{self, Knight, Pawn, Queen, Rook, Bishop, King};

pub fn get_fixed_moves(piece_type: Pieces) -> Vec<i8> {
    
    let moves: Vec<i8> = Vec::new();
    let moves: Vec<i8> = match piece_type {
        Pawn => vec![7, 8, 9, 16],
        Knight => vec![-17, -15, -10, -6, 6, 10, 15, 17],
        King => vec![-7, -8, -9, -1, 1, 7, 8, 9],
        Queen => panic!("Queen Does not have fixed moves, try sliding"),
        Bishop => panic!("Bishop Does not have fixed moves, try sliding"),
        Rook => panic!("Rook Does not have fixed moves, try sliding"),
        _ => panic!("Unknown Piece type"),
    };
    moves
}


pub fn get_sliding_directions(piece_type: Pieces) -> Vec<i8> {
    let mut moves: Vec<i8> = Vec::new();
    let directions: Vec<i8> = match piece_type {
        Rook => vec![-1 ,1, -8, 8],
        Bishop => vec![-7, -9, 7, 9],
        Queen => vec![-7, -8, -9, -1, 1, 7, 8, 9],
        Knight => panic!("Knight Does not have fixed moves, try sliding"),
        Pawn => panic!("Pawn Does not have fixed moves, try sliding"),
        King => panic!("King Does not have fixed moves, try sliding"),
        _ => panic!("Unknown Piece type"),
    };
    directions
}

fn main() {
    println!("0");
}