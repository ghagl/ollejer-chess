use ggez::event::{self, MouseButton};
use ggez::graphics::{self, DrawParam};
use ggez::{Context, GameResult};
use std::env;
use std::path;
use ollejer_chess::{board, rules, pieces};

const GRID_SIZE: (i16, i16) = (8, 8);
const GRID_CELL_SIZE: (i16, i16) = (45, 45);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl From<GridPosition> for ggez::mint::Point2<f32> {
    fn from(pos: GridPosition) -> Self {
        ggez::mint::Point2 { x: (pos.x * GRID_CELL_SIZE.0 as i32) as f32, y: (pos.y * GRID_CELL_SIZE.1 as i32) as f32 }
    }
}

impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

struct Tile {
    position: GridPosition,
    color: graphics::Color,
}

impl Tile {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.position.into(), self.color)?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 }, ))
    }
}

struct TilePiece {
    sprite: graphics::Image,
    position: GridPosition,
}

impl TilePiece {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.sprite, DrawParam::default().dest(self.position)
            .scale(ggez::mint::Vector2 { x: GRID_CELL_SIZE.0 as f32 / 45.0, y: GRID_CELL_SIZE.1 as f32 / 45.0 }))
    }
}

struct ChessGraphics {
	sprites: Vec<((pieces::Color, pieces::Pieces), String)>,
	tiles: Vec<Tile>,
	graphics_pieces: Vec<TilePiece>
}

impl ChessGraphics {

	fn new(board:&board::OneDBoard, ctx:&mut Context) -> Self {
		let mut obj = ChessGraphics {
			sprites: ChessGraphics::load_pieces(),
            tiles: Vec::new(),
            graphics_pieces: Vec::new(),
		};
		obj.get_in_pieces(board, ctx);
		obj
	}

	fn get_in_pieces(&mut self, board:&board::OneDBoard, ctx: &mut Context) {
		let mut tiles = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                tiles.push(Tile {
                    position: GridPosition { x, y },
                    color: if (x + y) % 2 == 0 { [1.0, 0.81, 0.62, 1.0].into() } else { [0.82, 0.55, 0.28, 1.0].into() },
                });
            }
        }

        self.tiles = tiles;
        self.update_board(board, ctx);
	}

	fn update_board(&mut self, board:&board::OneDBoard, ctx: &mut Context) {
        let mut graphics_pieces = Vec::new();
        let pieces = board.get_board();

		for (i, piece) in pieces.iter().enumerate() {
			let unwrapped_piece;
			if piece.is_some() {
				unwrapped_piece = piece.unwrap();	
			} else {
				continue;
			}

            graphics_pieces.push(TilePiece {
                sprite: graphics::Image::new(ctx, self.sprites.iter()
                    .find(|element| (element.0).0 == unwrapped_piece.color && (element.0).1 == unwrapped_piece.piece_type).unwrap().1.clone()).unwrap(),
                position: GridPosition::from((((i as i32) % 8), i as i32/8)),
				// lodräta kontra vertikala rader, suck...
            });
        }

        self.graphics_pieces = graphics_pieces;
    }

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        for tile in &self.tiles {
            tile.draw(ctx)?;
        }

        for graphics_piece in &self.graphics_pieces {
            graphics_piece.draw(ctx)?;
        }

        Ok(())
    }

	fn load_pieces() -> Vec<((pieces::Color, pieces::Pieces), String)> {
		let mut pieces = Vec::new();
        pieces.push(((pieces::Color::Black, pieces::Pieces::King), "/black_king.png".to_string()));
        pieces.push(((pieces::Color::Black, pieces::Pieces::Queen), "/black_queen.png".to_string()));
        pieces.push(((pieces::Color::Black, pieces::Pieces::Rook), "/black_rook.png".to_string()));
        pieces.push(((pieces::Color::Black, pieces::Pieces::Pawn), "/black_pawn.png".to_string()));
        pieces.push(((pieces::Color::Black, pieces::Pieces::Bishop), "/black_bishop.png".to_string()));
        pieces.push(((pieces::Color::Black, pieces::Pieces::Knight), "/black_knight.png".to_string()));
        pieces.push(((pieces::Color::White, pieces::Pieces::King), "/white_king.png".to_string()));
        pieces.push(((pieces::Color::White, pieces::Pieces::Queen), "/white_queen.png".to_string()));
        pieces.push(((pieces::Color::White, pieces::Pieces::Rook), "/white_rook.png".to_string()));
        pieces.push(((pieces::Color::White, pieces::Pieces::Pawn), "/white_pawn.png".to_string()));
        pieces.push(((pieces::Color::White, pieces::Pieces::Bishop), "/white_bishop.png".to_string()));
        pieces.push(((pieces::Color::White, pieces::Pieces::Knight), "/white_knight.png".to_string()));
        pieces
	}
	fn update(&mut self, board:&mut board::OneDBoard, ctx: &mut Context) {
		self.update_board(board, ctx);
    }
}

struct ChessLogic {

}

impl ChessLogic {

	fn new() -> Self {
		ChessLogic {

		}
	}

	fn make_move(&mut self, board: &mut board::OneDBoard, src:GridPosition, dest:GridPosition) -> bool {
		//println!("{:?} {:?}", src, dest);
		let trans_src = 8 * src.y + src.x;
		let trans_dest = 8 * dest.y + dest.x;
		//println!("{:?}", board.get_piece(trans_dest as usize));
		let res = rules::advance_piece_simple(board, trans_src as usize, trans_dest as usize);
		if res.is_ok() {
			return true;
		}
		println!("{:?}", res.err());
		false
	}
}

struct ChessInput {
	clicked_tile: Option<GridPosition>
}

impl ChessInput {

	fn new() -> Self {
		ChessInput {
			clicked_tile: None
		}
	}

	fn clicked(&mut self, ctx: &mut Context, board: &mut board::OneDBoard, button: MouseButton, x: f32, y: f32, cplogic:&mut ChessLogic, cgraphics: &mut ChessGraphics) {
		if button == MouseButton::Left {
			let clicked_pos = GridPosition { x: (x / GRID_CELL_SIZE.0 as f32).floor() as i32,
                y: (y / GRID_CELL_SIZE.1 as f32).floor() as i32};

			if self.clicked_tile.is_none() {
				self.clicked_tile = Some(clicked_pos)
			}
			else {
				if cplogic.make_move(board, self.clicked_tile.unwrap(), clicked_pos) {
					cgraphics.update(board, ctx);
				}
				self.clicked_tile = None;
			}
		}
	}
}

struct MainState {
	chess_board: board::OneDBoard,
	chess_graphics: ChessGraphics,
	chess_input: ChessInput,
	chess_logic: ChessLogic
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
		let board = board::OneDBoard::new();
		let cgraphics = ChessGraphics::new(&board, ctx);
		let cinput = ChessInput::new();
		let clogic = ChessLogic::new();
		let s = MainState {
			chess_board: board,
			chess_graphics: cgraphics,
			chess_input: cinput,
			chess_logic: clogic,
		};
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.chess_graphics.update(&mut self.chess_board, ctx);
        Ok(())
    }

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		self.chess_graphics.draw(ctx)?;
        graphics::present(ctx)?;
		Ok(())
	}

	fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.chess_input.clicked(ctx, &mut self.chess_board, button, x, y, &mut self.chess_logic, &mut self.chess_graphics);
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("schackdepression", "ghagl").add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default().title("Schack"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1).resizable(false));

    let (mut ctx, events_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}