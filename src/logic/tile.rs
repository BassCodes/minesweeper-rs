#[derive(Copy, Clone, PartialEq, Default)]
pub enum TileState {
	#[default]
	Empty,
	Mine,
}
#[derive(Copy, Clone, PartialEq)]
pub enum TileModifier {
	Flagged,
	Unsure,
}

#[derive(Copy, Clone, Default)]
pub struct Tile {
	pub state: TileState,
	pub modifier: Option<TileModifier>,
	pub swept: bool,
	pub adjacent: u8,
	pub safe: bool,
	pub highlighted: bool,
}

impl Tile {
	pub fn increment_adjacent(&mut self) {
		if self.state == TileState::Empty {
			self.adjacent += 1;
		}
	}
	pub fn highlight(&mut self) {
		if self.swept == false {
			self.highlighted = true;
		}
	}
	pub fn remove_highlight(&mut self) {
		self.highlighted = false;
	}
}
