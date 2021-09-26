use std::{thread, time};

use ollejer_chess::{board, rules};


fn main() {
    // let mut chess_obj = board::OneDBoard::new_from_FEN("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string());
    // println!("{}", get_king_pos(&mut chess_obj, pieces::Color::White));
    let mut chess_obj = board::OneDBoard::new();

    let c = rules::every_move(&mut chess_obj, 3);
    println!("Moves: {}", c);
    println!("Done");

    
    
    let mut t = 0;
    while t < 100 {
        let ept = chess_obj.get_en_passant_target();
        println!("EPT: {:?}", ept);
        {
            board::print_board(&chess_obj);
        }
        let turn = chess_obj.get_turn();

        if rules::check_if_team_is_in_check(&mut chess_obj, turn) {
            let avalible_moves = rules::get_all_valid_moves(&mut chess_obj, true);
            if avalible_moves.len() == 0 {
                println!("Checkmate");
                break;
            }
            
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