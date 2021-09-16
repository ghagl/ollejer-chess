mod piece;

struct OneDBoard {
    board: Vec<Box<dyn piece::Piece>>,

}

impl OneDBoard {
    fn new(&self, rows: u8, cols:u8) -> Self {
        OneDBoard {
            board: Vec::new()
        }
    }
    fn set_board_from_FEN(&mut self, fen_string: String) {
        /* 
        let mut split = fen_string.splitn(2, " ");
        let board_state = split.next().unwrap();
        let game_state = split.next().unwrap();

        let board_state = board_state.split("/");
        let mut board: Vec<char> = Vec::new();

        let i: u8 = 0;
        for rank in board_state {
            println!("{}", rank);
            for char in rank.chars() {
                if char.is_digit(10) {
                    let d: u8 = char.to_string().parse::<u8>().unwrap();
                    for _ in 0..d {
                        board.push(' ');
                    }
                }
                else {
                    board.push(char);
                }
            }
        }
        println!("{}", board.len());
        board
        */

    }
}


fn main() {
    println!("0");
}