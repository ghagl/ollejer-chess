use std::io;
use std::{thread, time};

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
                println!("{:?} | {:?}", piece.piece_type, moves);
            }
            None => continue,
        };
    }
}

fn denest_nested_moves(nested: Vec<Vec<i8>>) -> Vec<i8> {
    nested.into_iter().flat_map(|v| v).collect()
}

fn calculate_target_pos(position: usize, a_move: i8) -> usize {
    let target: i8 = position as i8 + a_move;
    let target: usize = target as usize;
    target
}

fn validate_fixed_moves(
    one_d_board: &board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    let all_moves: Vec<i8> = denest_nested_moves(all_moves);
    let mut valid_moves: Vec<i8> = Vec::new();
    let board_array = one_d_board.get_board();
    let this_piece_color = one_d_board.get_piece(position).unwrap().color;

    for (i, a_move) in all_moves.iter().enumerate() {
        let target = calculate_target_pos(position, *a_move);
        match one_d_board.get_piece(target) {
            Some(target_piece) => match this_piece_color {
                this_knight_color if this_piece_color == target_piece.color => continue,
                _ => valid_moves.push(*a_move),
            },
            None => valid_moves.push(*a_move),
        }
    }
    valid_moves
}

fn validate_sliding_moves(
    one_d_board: &board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    // [[7, 14], [8, 16, 24]]
    let mut valid_moves: Vec<i8> = Vec::new();
    let board_array = one_d_board.get_board();
    let this_slider_color = one_d_board.get_piece(position).unwrap().color;

    for direction in all_moves {
        for a_move in direction {
            // if alied or enemy piece found:
            //      break inner loop and include current if enemy
            let target = calculate_target_pos(position, a_move);
            match one_d_board.get_piece(target) {
                Some(target_piece) => match this_slider_color {
                    this_slider_color if this_slider_color == target_piece.color => break,
                    _ => {
                        valid_moves.push(a_move);
                        break;
                    }
                },
                None => valid_moves.push(a_move),
            }
        }
    }
    valid_moves
}

fn validate_knight_moves(
    one_d_board: &board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    let valid_moves = validate_fixed_moves(one_d_board, all_moves, position);
    // Move does not result in placing on king in check

    valid_moves
}

fn validate_queen_moves(
    one_d_board: &board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    // Validate sliding moves
    let semi_valid_moves = validate_sliding_moves(one_d_board, all_moves, position);
    // Validate not creating check

    semi_valid_moves
}

fn validate_rook_moves(
    one_d_board: &board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    // Validate sliding moves
    let semi_valid_moves = validate_sliding_moves(one_d_board, all_moves, position);

    semi_valid_moves
}

fn validate_bishop_moves(
    one_d_board: &board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    let semi_valid_moves = validate_sliding_moves(one_d_board, all_moves, position);
    semi_valid_moves
}

fn validate_straight_moves_for_pawn(
    one_d_board: &board::OneDBoard,
    straight_moves: Vec<i8>,
    position: usize,
) -> Vec<i8> {
    let mut valid_straight_moves = Vec::new();
    for a_move in straight_moves.iter() {
        // if alied or enemy piece found:
        //      break inner loop and include current if enemy
        let target = calculate_target_pos(position, *a_move);
        match one_d_board.get_piece(target) {
            Some(target_piece) => break,

            None => valid_straight_moves.push(*a_move),
        }
    }
    valid_straight_moves
}

fn validate_pawn_moves(
    one_d_board: &mut board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
) -> Vec<i8> {
    let mut valid_moves: Vec<i8> = Vec::new();

    let regular_moves = all_moves.get(0).unwrap();
    let mut passant_moves = all_moves.get(1).unwrap();
    let mut valid_passant_moves = Vec::new();

    let this_pawn_color = one_d_board.get_piece(position).unwrap().color;
    let current_rank = position / 8;

    // Check if passant is allowed
    for pm in passant_moves {
        match (this_pawn_color, current_rank) {
            (pieces::Color::White, 6) => valid_passant_moves.push(*pm),
            (pieces::Color::Black, 1) => valid_passant_moves.push(*pm),
            (_, _) => continue,
        };
    }

    // Get non blocked straight moves
    let straight_moves: Vec<i8> =
        denest_nested_moves(vec![regular_moves.to_vec(), valid_passant_moves.to_vec()]);
    let mut valid_straight_moves: Vec<i8> =
        validate_straight_moves_for_pawn(one_d_board, straight_moves, position);

    // Get en passant moves
    let capturing_moves = all_moves.get(2).unwrap();
    let mut valid_capturing_moves: Vec<i8> = Vec::new();

    let en_passant_position = one_d_board.get_en_passant_target();

    for cm in capturing_moves {
        let target_position = calculate_target_pos(position, *cm);
        let tile = one_d_board.get_piece(target_position);
        match tile {
            Some(piece) => match this_pawn_color {
                this_pawn_color if this_pawn_color == piece.color => continue,
                _ => valid_capturing_moves.push(*cm),
            },
            // None should check if this square is en passant square
            None => match en_passant_position {
                Some(epp) => match target_position {
                    target_position if target_position == epp => valid_capturing_moves.push(*cm),
                    _ => continue,
                },
                None => continue,
            },
        }
    }

    let this_pawn_color = one_d_board.get_piece(position).unwrap().color;

    valid_moves.extend(valid_straight_moves);
    valid_moves.extend(valid_capturing_moves);
    valid_moves
}

fn validate_castle_moves(
    one_d_board: &mut board::OneDBoard,
    possible_castle_moves: Vec<i8>,
    position: usize,
) -> Vec<i8> {
    let mut valid_castle_moves: Vec<i8> = Vec::new();
    let this_king = one_d_board.get_piece(position).unwrap();
    for pcm in possible_castle_moves {
        let dest = position as i8 + pcm;
        let dest = dest as usize;

        let mut largest = 0;
        let mut smallest = 0;
        if dest > position {
            largest = dest;
            smallest = position;
        } else {
            largest = position;
            smallest = dest;
        }
        let mut move_alowed: bool = true;
        for square in smallest..=largest {
            if check_if_square_is_attacked(one_d_board, this_king.color, square) {
                move_alowed = false;
                break;
            }
            if square != position {
                match one_d_board.get_piece(square) {
                    Some(piece) => {
                        move_alowed = false;
                        break;
                    }
                    None => (),
                }
            }
        }
        if move_alowed {
            //board::print_board(&one_d_board);
            valid_castle_moves.push(pcm)
        }
    }
    valid_castle_moves
}

fn validate_king_moves(
    one_d_board: &mut board::OneDBoard,
    all_moves: Vec<Vec<i8>>,
    position: usize,
    check_castle_moves: bool,
) -> Vec<i8> {
    let default_moves = vec![all_moves.get(0).unwrap().to_vec()];
    let mut valid_moves = validate_fixed_moves(one_d_board, default_moves, position);
    if check_castle_moves {
        let castle_moves = all_moves.get(1).unwrap().to_vec();
        let valid_castle_moves = validate_castle_moves(one_d_board, castle_moves, position);
        valid_moves.extend(valid_castle_moves);
    }

    valid_moves
}

fn get_valid_moves_from_piece(
    one_d_board: &mut board::OneDBoard,
    piece: pieces::Piece,
    position: usize,
    extra_validation: bool,
) -> Vec<i8> {
    let moves = movement::get_moves_from_piece(piece, position);

    // Board legal moves
    let board_legal_moves: Vec<i8> = match piece.piece_type {
        pieces::Pieces::Knight => validate_knight_moves(one_d_board, moves, position),
        pieces::Pieces::Queen => validate_queen_moves(one_d_board, moves, position),
        pieces::Pieces::Rook => validate_rook_moves(one_d_board, moves, position),
        pieces::Pieces::Bishop => validate_bishop_moves(one_d_board, moves, position),
        pieces::Pieces::Pawn => validate_pawn_moves(one_d_board, moves, position),
        pieces::Pieces::King => validate_king_moves(one_d_board, moves, position, extra_validation),
        _ => panic!("Error! Cant get move from unknown piece."), //WARNING WTF IS THIS GHOST
    };

    // Check if moves selfchecks
    if extra_validation {
        let mut valid_moves = Vec::new();
        for valid_move in board_legal_moves {
            if !check_if_move_checks_yourself(one_d_board, position, valid_move) {
                valid_moves.push(valid_move);
            } else {
                let dest = position as i8 + valid_move;
                let dest = dest as usize;
                println!(
                    "You will be checked if you goes to {}. Try another piece",
                    dest
                );
            }
        }
        return valid_moves;
    }

    board_legal_moves
}

fn every_move(one_d_board: &mut board::OneDBoard, depth: u8) -> usize {
    if depth == 0 {
        return 1;
    }
    let mut count: usize = 0;
    for pos in 0..64 {
        let tile = one_d_board.get_piece(pos);
        let piece = match tile {
            Some(p) => p,
            None => continue,
        };

        let piece_color = piece.color;
        let turn = one_d_board.get_turn();

        match turn {
            turn if turn == piece_color => (),
            _ => continue,
        }
        let valid_moves = get_valid_moves_from_piece(one_d_board, piece, pos, true);
        for vm in valid_moves {
            let dest = pos as i8 + vm;
            let dest = dest as usize;

            let mut new_board = one_d_board.clone();
            let result = new_board.make_move(pos, dest);
            count += every_move(&mut new_board, depth - 1);
        }
    }
    count
}

fn get_king_pos(one_d_board: &board::OneDBoard, king_color: pieces::Color) -> usize {
    for pos in 0..64 {
        let tile = one_d_board.get_piece(pos);
        let piece = match tile {
            Some(p) => p,
            None => continue,
        };

        match king_color {
            king_color if king_color == piece.color => (),
            _ => continue,
        }

        let piece_type = piece.piece_type;
        match piece_type {
            piece_type if piece_type == pieces::Pieces::King => return pos,
            _ => continue,
        }
    }
    panic!(format!("No king of color: {:?} on the board!", king_color));
}

fn check_if_square_is_attacked(
    current_board: &mut board::OneDBoard,
    attacking_color: pieces::Color,
    pos: usize,
) -> bool {
    for pos in 0..64 {
        // get enemy piece
        let square = current_board.get_piece(pos);
        let piece = match square {
            Some(p) => p,
            None => continue,
        };

        match attacking_color {
            attacking_color if attacking_color == piece.color => (),
            _ => continue,
        }

        let valid_moves = get_valid_moves_from_piece(current_board, piece, pos, false);

        for valid_move in valid_moves {
            let dest = pos as i8 + valid_move;
            let dest = dest as usize;
            if dest == pos {
                return true;
            }
        }
    }
    false
}

fn check_if_team_is_in_check(current_board: &mut board::OneDBoard, color: pieces::Color) -> bool {
    for pos in 0..64 {
        // get enemy piece
        let tile = current_board.get_piece(pos);
        let piece = match tile {
            Some(p) => p,
            None => continue,
        };
        let piece_color = piece.color;
        let turn = current_board.get_turn();
        match color {
            color if color == piece_color => continue,
            _ => (),
        }

        let valid_moves = get_valid_moves_from_piece(current_board, piece, pos, false);
        let allied_king_pos = get_king_pos(&current_board, color);
        for valid_move in valid_moves {
            let dest = pos as i8 + valid_move;
            let dest = dest as usize;
            if dest == allied_king_pos {
                return true;
            }
        }
    }
    false
}

fn check_if_move_checks_yourself(
    current_board: &mut board::OneDBoard,
    origin: usize,
    a_move: i8,
) -> bool {
    let mut new_board = current_board.clone();
    let color: pieces::Color = current_board.get_turn();
    let dest = origin as i8 + a_move;
    let dest = dest as usize;
    new_board.make_move(origin, dest);
    let checked = check_if_team_is_in_check(&mut new_board, color);
    return checked;
}

fn read_move() -> Result<(usize, usize), &'static str> {
    let mut std_input = String::new();
    std::io::stdin()
        .read_line(&mut std_input)
        .expect("Error reading");

    let origin = &std_input[..2];
    let dest = &std_input[2..4];

    let origin = board::translate_tile_to_usize(origin);
    let dest = board::translate_tile_to_usize(dest);

    let origin: usize = match origin {
        Ok(pos) => pos,
        Err(e) => return Err(e),
    };
    let dest: usize = match dest {
        Ok(pos) => pos,
        Err(e) => return Err(e),
    };

    Ok((origin, dest))
}

fn advance_piece(one_d_board: &mut board::OneDBoard) -> Result<(), &'static str> {
    let result = read_move();
    let (origin, dest): (usize, usize) = match result {
        Ok((o, d)) => (o, d),
        Err(e) => return Err(e),
    };

    let piece = one_d_board.get_piece(origin);
    let piece: pieces::Piece = match piece {
        Some(p) => p,
        None => return Err("No Piece on that tile"),
    };

    let valid_moves = get_valid_moves_from_piece(one_d_board, piece, origin, true);
    let the_move: i8 = dest as i8 - origin as i8;

    if !(valid_moves.contains(&the_move)) {
        println!("Valid moves: {:?}", valid_moves);
        return Err("Can't move there!");
    }
    let result = one_d_board.make_move(origin, dest);
    match result {
        Ok(_) => Ok(()),
        Err(E) => return Err(E),
    }
}

fn main() {
    let mut chess_obj = board::OneDBoard::new_standard();
    //println!("{}", get_king_pos(&mut chess_obj, pieces::Color::White));

    let c = every_move(&mut chess_obj, 3);
    println!("Moves: {}", c);

    let mut t = 0;
    while t < 100 {
        let ept = chess_obj.get_en_passant_target();
        println!("EPT: {:?}", ept);
        {
            board::print_board(&chess_obj);
        }
        let turn = chess_obj.get_turn();

        if check_if_team_is_in_check(&mut chess_obj, turn) {
            println!("You are in Check!");
        }

        let result = advance_piece(&mut chess_obj);
        match result {
            Ok(_) => println!("U made a valid move, Congrats!"),
            Err(e) => println!("{}", e),
        }
        thread::sleep(time::Duration::from_millis(300));

        t += 1;
    }
}
