
#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

fn get_diagonal_directions() -> Vec<i8> {
    vec![-9, -7, 7, 9]
}

fn get_straight_directions() -> Vec<i8> {
    vec![8, -8, 1, -1]
}
