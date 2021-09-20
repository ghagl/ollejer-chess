mod board;
mod movement;
mod pieces;
mod rules;


fn main() {
    let chess_board = board::OneDBoard::new_standard();
    
    board::print_board(&chess_board.board);
    
    // take in move ex: e2e4
    // check if piece is on e2 => what piece
    // get valid moves of piece 
    //       - check that it does not lead to check
    // if e4 is valid move there

    
    
    println!("0");
}
