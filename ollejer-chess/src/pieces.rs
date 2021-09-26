use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Pieces {
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Pawn,
}

//TODO: change fields to getters to avoid changing type and color :)
#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub color: Color,
    pub piece_type: Pieces,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

