use std::{thread, time};

use ollejer_chess::{board, rules};


fn main() {
    let mut chess_obj = board::OneDBoard::new_standard();
    //println!("{}", get_king_pos(&mut chess_obj, pieces::Color::White));

    let c = rules::every_move(&mut chess_obj, 4);
    println!("Moves: {}", c);

    let mut t = 0;
    while t < 100 {
        let ept = chess_obj.get_en_passant_target();
        println!("EPT: {:?}", ept);
        {
            board::print_board(&chess_obj);
        }
        let turn = chess_obj.get_turn();

        if rules::check_if_team_is_in_check(&mut chess_obj, turn) {
            println!("You are in Check!");
        }

        let result = rules::advance_piece(&mut chess_obj);
        match result {
            Ok(_) => println!("U made a valid move, Congrats!"),
            Err(e) => println!("{}", e),
        }
        thread::sleep(time::Duration::from_millis(200));

        t += 1;
    }
}