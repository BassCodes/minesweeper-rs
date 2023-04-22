use super::game_board::GameBoard;
use super::tile::Tile;
pub enum GameEvent {
	Lose(usize, usize, Tile),
	RevealTile(usize, usize, Tile),
	FlagTile(usize, usize, Tile),
	QuestionTile(usize, usize, Tile),
	SweepDone,
	SweepBegin,
	InitDone,
	Win,
	Reset,
	GameEnd(GameBoard),
}
#[derive(Default)]
pub struct Events {
	events: Vec<GameEvent>,
}

impl Events {
	pub fn add(&mut self, event: GameEvent) {
		self.events.push(event);
	}
	pub fn next(&mut self) -> Option<GameEvent> {
		if self.events.len() > 0 {
			self.events.pop()
		} else {
			None
		}
	}
	pub fn clear(&mut self) {
		self.events.clear();
	}
}
