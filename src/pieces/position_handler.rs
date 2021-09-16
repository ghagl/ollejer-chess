

pub struct OneDPositionHandler {
    position: i8,
}

impl OneDPositionHandler {
    pub fn new(pos: i8) -> Self {
        OneDPositionHandler { position: pos }
    }
    pub fn get_position(&self) -> i8 {
        self.position
    }

    pub fn set_position(&mut self, pos: i8) {
        self.position = pos;
    }
}

fn main() {
    println!("PositionHandler")
}
