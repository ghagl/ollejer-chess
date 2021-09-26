use std::cmp::min;

use crate::pieces::{
    Color::{self, Black, White},
    Piece,
    Pieces::{Bishop, King, Knight, Pawn, Queen, Rook},
};

fn get_moves_from_direction(dir: i8, range: usize) -> Vec<i8> {
    let mut moves: Vec<i8> = Vec::new();
    let mut m = dir;
    for _ in 0..range {
        moves.push(m);
        m += dir;
    }
    moves
}

fn diagonal_moves(position: usize) -> Vec<Vec<i8>> {
    let current_rank = position / 8;
    let current_file = position % 8;

    let dist_north = current_rank;
    let dist_south = 8 - current_rank - 1;
    let dist_west = current_file;
    let dist_east = 8 - current_file - 1;

    let mut moves: Vec<Vec<i8>> = Vec::new();

    let ne: Vec<i8> = get_moves_from_direction(-7, min(dist_north, dist_east));
    let nw: Vec<i8> = get_moves_from_direction(-9, min(dist_north, dist_west));
    let sw: Vec<i8> = get_moves_from_direction(7, min(dist_south, dist_west));
    let se: Vec<i8> = get_moves_from_direction(9, min(dist_south, dist_east));

    moves.push(ne);
    moves.push(nw);
    moves.push(sw);
    moves.push(se);

    moves
}

fn straight_moves(position: usize) -> Vec<Vec<i8>> {
    let current_rank = position / 8;
    let current_file = position % 8;

    let dist_north = current_rank;
    let dist_south = 8 - current_rank - 1;
    let dist_west = current_file;
    let dist_east = 8 - current_file - 1;

    let mut moves: Vec<Vec<i8>> = Vec::new();

    let n: Vec<i8> = get_moves_from_direction(-8, dist_north);
    let w: Vec<i8> = get_moves_from_direction(-1, dist_west);
    let s: Vec<i8> = get_moves_from_direction(8, dist_south);
    let e: Vec<i8> = get_moves_from_direction(1, dist_east);

    moves.push(n);
    moves.push(w);
    moves.push(s);
    moves.push(e);

    moves
}

pub fn get_knight_moves(position: usize) -> Vec<Vec<i8>> {
    let mut candidate_moves: Vec<i8> = vec![-17, -15, -10, -6, 6, 10, 15, 17];

    // Handle out of bounds
    let current_file = position % 8;
    if current_file <= 1 {
        candidate_moves.retain(|&x| x != 6 && x != -10);
        if current_file == 0 {
            candidate_moves.retain(|&x| x != -17 && x != 15);
        }
    } else if current_file >= 6 {
        candidate_moves.retain(|&x| x != -6 && x != 10);
        if current_file == 7 {
            candidate_moves.retain(|&x| x != -15 && x != 17);
        }
    }

    let current_rank = position / 8;
    if current_rank <= 1 {
        candidate_moves.retain(|&x| x != -17 && x != -15);
        if current_rank == 0 {
            candidate_moves.retain(|&x| x != -10 && x != -6);
        }
    } else if current_rank >= 6 {
        candidate_moves.retain(|&x| x != 15 && x != 17);
        if current_rank == 7 {
            candidate_moves.retain(|&x| x != 6 && x != 10);
        }
    }
    vec![candidate_moves]
}

pub fn get_king_moves(position: usize) -> Vec<Vec<i8>> {
    let current_rank = position / 8;
    let current_file = position % 8;

    let dist_north = current_rank;
    let dist_south = 8 - current_rank - 1;
    let dist_west = current_file;
    let dist_east = 8 - current_file - 1;

    let mut moves: Vec<i8> = Vec::new();

    // Covers top row moves -7 -8 -9
    if dist_north > 0 {
        moves.push(-8);
        if dist_west > 0 {
            moves.push(-7);
        }
        if dist_east > 0 {
            moves.push(-9);
        }
    }
    // Bottom row moves 7 8 9
    if dist_south > 0 {
        moves.push(8);
        if dist_west > 0 {
            moves.push(7);
        }
        if dist_east > 0 {
            moves.push(9);
        }
    }
    // Sides -1 and 1
    if dist_west > 0 {
        moves.push(-1);
    }
    if dist_east > 0 {
        moves.push(1);
    }

    let castle_moves = vec![2, -2];

    let mut all_moves: Vec<Vec<i8>> = Vec::new();
    all_moves.push(moves);
    all_moves.push(castle_moves);
    all_moves
}

pub fn get_queen_moves(position: usize) -> Vec<Vec<i8>> {
    let mut all_moves = Vec::new();
    let mut diagonal_moves = diagonal_moves(position);
    let mut straight_moves = straight_moves(position);
    all_moves.append(&mut straight_moves);
    all_moves.append(&mut diagonal_moves);
    all_moves
}

pub fn get_rook_moves(position: usize) -> Vec<Vec<i8>> {
    straight_moves(position)
}

pub fn get_bishop_moves(position: usize) -> Vec<Vec<i8>> {
    diagonal_moves(position)
}

pub fn get_pawn_moves(position: usize, color: Color) -> Vec<Vec<i8>> {
    let mut moves = match color {
        White => vec![vec![-8], vec![-16], vec![-7, -9]],
        Black => vec![vec![8], vec![16], vec![7, 9]],
    };

    let current_file = position % 8;
    if current_file == 0 {
        match color {
            White => moves.get_mut(2).unwrap().remove(1),
            Black => moves.get_mut(2).unwrap().remove(0),
        };
    } else if current_file == 7 {
        match color {
            White => moves.get_mut(2).unwrap().remove(0),
            Black => moves.get_mut(2).unwrap().remove(1),
        };
    }

    moves
}

pub fn get_moves_from_piece(piece: Piece, position: usize) -> Vec<Vec<i8>> {
    let moves: Vec<Vec<i8>> = match piece.piece_type {
        Knight => get_knight_moves(position),
        Bishop => get_bishop_moves(position),
        Rook => get_rook_moves(position),
        Queen => get_queen_moves(position),
        King => get_king_moves(position),
        Pawn => get_pawn_moves(position, piece.color),
        _ => panic!("Error! Cant get move from unknown piece."),
    };
    moves
}

