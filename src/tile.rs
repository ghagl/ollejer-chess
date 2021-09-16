
mod colors;
mod piece;

trait Tile {
    fn get_color(&self) -> (u8, u8, u8);
    fn get_piece(&self) -> piece::Piece;
}

struct WhiteTile {
    containing_piece: Box<piece::Piece>,
    color: (u8, u8, u8),
}

impl Tile for WhiteTile {
    fn new(&self, p: &piece::Piece) {
        self.containing_piece = box::new(p);
        color = (255, 255, 255);
    }
    fn get_color(&self) -> (u8, u8, u8) {
        self.color
    }
    fn get_piece(&self) -> Option<Piece> {
        let val = match *self.containing_piece{
            Some(p) => p,
            None(_) => None,
        };
        val
    }
}

fn main() {
    println!("0")
}