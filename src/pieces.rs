mod position_handler;

fn main() {
    let k = Knight::new(4);
    let k_m = k.get_fixed_moves();
    let r = Rook::new(42);
    let r_m = r.get_sliding_directions();
    let b = Bishop::new(2);
    let b_m = b.get_sliding_directions();
    let q = Queen::new(34);
    let q_m = q.get_sliding_directions();
    let k = King::new(5);
    let k_m = k.get_fixed_moves();
    println!("{:?}", k_m);
}

fn get_diagonal_directions(pos: i8) -> Vec<i8> {
    vec![-9, -7, 7, 9]   
}

fn get_straight_directions(pos: i8) -> Vec<i8> {
    vec![8, -8, 1, -1]
}

pub trait Piece {
    fn get_fixed_moves(&self) -> Vec<i8>;
    fn get_sliding_directions(&self) -> Vec<i8>;
}

pub struct Knight {
    position_handler: position_handler::OneDPositionHandler,
}
impl Piece for Knight {
    fn get_fixed_moves(&self) -> Vec<i8> {
        let candidate_moves: Vec<i8> = vec![-17, -15, -10, -6, 6, 10, 15, 17];
        candidate_moves
    }
    fn get_sliding_directions(&self) -> Vec<i8> {
        panic!("No sliding moves for the Knight");
    }
}
impl Knight {
    fn new(starting_position: i8) -> Self {
        Knight {
            position_handler: position_handler::OneDPositionHandler::new(starting_position),
        }
    }
}

pub struct Rook {
    position_handler: position_handler::OneDPositionHandler,
}

impl Piece for Rook {
    fn get_sliding_directions(&self) -> Vec<i8> {
        let pos = self.position_handler.get_position();
        
        let candidate_directions: Vec<i8> = get_straight_directions(pos);
        candidate_directions
    }
    fn get_fixed_moves(&self) -> Vec<i8> {
        panic!("No fixed moves for the Rook, try sliding moves");
    }
}
impl Rook {
    fn new(starting_position: i8) -> Self {
        Rook {
            position_handler: position_handler::OneDPositionHandler::new(starting_position),
        }
    }
}


pub struct Bishop {
    position_handler: position_handler::OneDPositionHandler,
}

impl Piece for Bishop {
    fn get_sliding_directions(&self) -> Vec<i8> {
        let pos = self.position_handler.get_position();
        
        let candidate_directions: Vec<i8> = get_diagonal_directions(pos);
        candidate_directions
    }
    fn get_fixed_moves(&self) -> Vec<i8> {
        panic!("No fixed moves for the Bishop, try sliding moves");
    }
}
impl Bishop {
    fn new(starting_position: i8) -> Self {
        Bishop {
            position_handler: position_handler::OneDPositionHandler::new(starting_position),
        }
    }
}

pub struct Queen {
    position_handler: position_handler::OneDPositionHandler,
}

impl Piece for Queen {
    fn get_sliding_directions(&self) -> Vec<i8> {
        let pos = self.position_handler.get_position();
        

        let mut all_moves = Vec::new();
        all_moves.extend(get_diagonal_directions(pos));
        all_moves.extend(get_straight_directions(pos));
        all_moves
    }
    fn get_fixed_moves(&self) -> Vec<i8> {
        panic!("No fixed moves for the Queen, try sliding moves");
    }
}
impl Queen {
    fn new(starting_position: i8) -> Self {
        Queen {
            position_handler: position_handler::OneDPositionHandler::new(starting_position),
        }
    }
}

pub struct King {
    position_handler: position_handler::OneDPositionHandler,
}

impl Piece for King {
    fn get_sliding_directions(&self) -> Vec<i8> {
        panic!("No fixed moves for the Queen, try sliding moves");
        
    }
    fn get_fixed_moves(&self) -> Vec<i8> {
        
        let pos = self.position_handler.get_position();
        

        let mut all_moves = Vec::new();
        all_moves.extend(get_diagonal_directions(pos));
        all_moves.extend(get_straight_directions(pos));
        all_moves
    }
}
impl King {
    fn new(starting_position: i8) -> Self {
        King {
            position_handler: position_handler::OneDPositionHandler::new(starting_position),
        }
    }
}



