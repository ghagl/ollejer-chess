mod board;
mod movement;
mod pieces;

//TODO CHANGE NAME TO rules.rs


fn print_all_moves(one_d_board: &board::OneDBoard) {
    let board_array = one_d_board.get_board();
    for (pos, tile) in board_array.iter().enumerate() {
        match tile {
            Some(t) => {
                let piece = one_d_board.get_piece(pos).unwrap();
                let moves = movement::get_moves_from_piece(piece, pos);
                println!("{:?} | {:?}",piece.piece_type, moves); 

            }
            None => continue,
        };
    }

}

fn denest_nested_moves(nested: Vec<Vec<i8>>) -> Vec<i8> {
    nested
        .into_iter()
        .flat_map(|v| v)
        .collect()
}



fn validate_knight_moves(one_d_board: &board::OneDBoard, all_moves: Vec<Vec<i8>>, position: usize) -> Vec<i8>{
    // No allied piece on tile
    let all_moves = denest_nested_moves(all_moves);

    // Move does not result in placing on king in check
}



fn get_valid_moves_from_piece(one_d_board: &board::OneDBoard, piece: pieces::Piece, pos: i8) -> Vec<i8> {
    let moves = movement::get_moves_from_piece(piece, pos);
    let valid_moves: Vec<i8> = match piece.piece_type {
        Knight => validate_knight_moves(position),
        //Bishop => validate_bishop_moves(position),
        //Rook   => validate_rook_moves(position),
        //Queen  => validate_queen_moves(position),
        //King   => validate_king_moves(position),
        //Pawn   => validate_pawn_moves(position),
        _ => panic!("Error! Cant get move from unknown piece.")
    };
    
}



fn main() {
    let chess_obj = board::OneDBoard::new_standard();
    let the_board = chess_obj.get_board();
    board::print_board(&the_board);
    //print_all_moves(&chess_obj); 


    let pos = board::translate_tile_to_usize("d1");
    let piece = chess_obj.get_piece(pos);
    let moves = movement::get_moves_from_piece(piece.unwrap(), pos);
    let moves: Vec<i8> = denest_nested_moves(moves);



    println!("Qm{:?}", moves);
    
    
    // take in move ex: e2e4
    // check if piece is on e2 => what piece
    // get valid moves of piece 
    //       - check that it does not lead to check
    // if e4 is valid move there

    
}
