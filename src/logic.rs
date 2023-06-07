// pub mod events;
pub mod game_board;
pub mod tile;
mod timer;
use crate::util::Events;

use self::{tile::Tile, timer::Timer};
use game_board::GameBoard;
use std::error::Error;

#[derive(PartialEq, Default)]
pub enum GameState {
	#[default]
	Empty,
	Playing,
	GameOver,
	Victory,
}
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
pub struct Minesweeper {
	pub board: GameBoard,
	pub events: Events<GameEvent>,
	pub state: GameState,
	timer: Timer,
}

impl Minesweeper {
	pub fn new(width: usize, height: usize, mines: usize) -> Result<Minesweeper, Box<dyn Error>> {
		let game = Self {
			board: GameBoard::new(width, height, mines)?,
			..Default::default()
		};

		Ok(game)
	}
	pub fn reset(&mut self) {
		self.board.reset();
		self.state = GameState::Empty;
		self.events.clear();
		self.events.add(GameEvent::Reset);
		self.timer.clear();
	}
	pub fn update_and_reset(&mut self, width: usize, height: usize, mines: usize) {
		self.board.update(width, height, mines);
		self.reset();
	}

	pub fn reveal(&mut self, x: usize, y: usize) {
		if GameState::Empty == self.state {
			self.timer.start();
			self.state = GameState::Playing;
			self.events.add(GameEvent::InitDone);
		}
		if self.state != GameState::Playing {
			return;
		}
		if let Some(state) = self.board.sweep(x, y, &mut self.events) {
			if state == GameState::GameOver || state == GameState::Victory {
				self.timer.stop()
			}
			self.state = state;
		};
		self.events.add(GameEvent::SweepDone);
	}
	pub fn modify(&mut self, x: usize, y: usize) {
		if self.state != GameState::Playing {
			return;
		}
		self.board.modify(x, y, &mut self.events)
	}
	pub fn get_time(&self) -> Option<f64> {
		self.timer.elapsed()
	}
	pub fn highlight(&mut self, x: usize, y: usize) {
		if self.state == GameState::Playing || self.state == GameState::Empty {
			if let Some(tile) = self.board.get_tile_mut(x, y) {
				tile.highlight();
			}
		}
	}
	pub fn remove_highlight(&mut self, x: usize, y: usize) {
		if let Some(tile) = self.board.get_tile_mut(x, y) {
			tile.remove_highlight();
		}
	}
}
