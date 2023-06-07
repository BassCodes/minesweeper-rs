use std::error::Error;

use std::collections::VecDeque;

use macroquad::time;

use super::tile::{TileModifier, TileState};
use super::{Events, GameEvent, GameState};
use crate::logic::tile::Tile;

use crate::util::{ADJACENT_WITHOUT_CENTER, ADJACENT_WITH_CENTER};

#[derive(Clone, Default)]
pub struct GameBoard {
	pub tiles: Vec<Vec<Tile>>,
	width: usize,
	height: usize,

	state: BoardState,
	non_mine_tiles: usize,
	pub revealed_tiles: usize,
	flags: usize,
	mines: usize,
	pub modify_mode: ModifyMode,
}
#[derive(Default, Clone)]
enum BoardState {
	#[default]
	Ungenerated,
	Generated,
}
#[derive(Default, Clone, Debug)]
pub enum ModifyMode {
	#[default]
	Flag,
	Question,
}

impl GameBoard {
	pub fn new(width: usize, height: usize, mines: usize) -> Result<Self, Box<dyn Error>> {
		if width == 0 || height == 0 {
			return Err("Can't make game board with zero length dimension".into());
		};
		if (width * height) as isize - 9 < mines as isize {
			// Can't have more mines than tiles on the game board. Else, the
			// loop that places mines on board will never complete.
			return Err("Not enough space for mines".into());
		}

		let board = Self {
			tiles: vec![vec![Tile { ..Default::default() }; height]; width],
			width,
			height,
			mines,
			non_mine_tiles: width * height - mines,
			..Default::default()
		};
		Ok(board)
	}

	pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
		if self.is_valid_coord(x, y) {
			return Some(&self.tiles[x][y]);
		}
		None
	}

	pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
		if self.is_valid_coord(x, y) {
			return Some(&mut self.tiles[x][y]);
		}
		None
	}

	pub fn is_valid_coord(&self, x: usize, y: usize) -> bool {
		if x < self.width && y < self.height {
			return true;
		}
		false
	}

	pub fn reset(&mut self) {
		self.state = BoardState::Ungenerated;

		self.revealed_tiles = 0;
		self.flags = 0;
		self.tiles = vec![vec![Tile { ..Default::default() }; self.height]; self.width]
	}
	pub fn update(&mut self, width: usize, height: usize, mines: usize) {
		self.mines = mines;
		self.height = height;
		self.width = width;
		self.non_mine_tiles = width * height - mines;
		self.state = BoardState::Ungenerated;
	}

	pub fn remaining_flags(&self) -> isize {
		self.mines as isize - self.flags as isize
	}

	pub fn modify(&mut self, x: usize, y: usize, event_handler: &mut Events<GameEvent>) {
		if let Some(&tile) = &self.get_tile(x, y) {
			if tile.swept {
				return;
			}
			let modifier = if let Some(modifier) = tile.modifier {
				match modifier {
					TileModifier::Flagged => {
						self.flags -= 1;
						match self.modify_mode {
							ModifyMode::Flag => {
								event_handler.add(GameEvent::FlagTile(x, y, tile.clone()));
								None
							}
							ModifyMode::Question => {
								event_handler.add(GameEvent::QuestionTile(x, y, tile.clone()));

								Some(TileModifier::Unsure)
							}
						}
					}
					TileModifier::Unsure => None,
				}
			} else {
				self.flags += 1;
				event_handler.add(GameEvent::FlagTile(x, y, tile.clone()));
				Some(TileModifier::Flagged)
			};
			if let Some(tile) = self.get_tile_mut(x, y) {
				tile.modifier = modifier;
			}
		}
	}

	pub fn sweep(&mut self, x: usize, y: usize, event_handler: &mut Events<GameEvent>) -> Option<GameState> {
		if let BoardState::Ungenerated = self.state {
			self.generate(x, y);
		}
		let &tile = &self.tiles[x][y];
		if let Some(_) = tile.modifier {
			return None;
		}
		if tile.swept {
			return None;
		}
		self.tiles[x][y].swept = true;
		self.revealed_tiles += 1;
		event_handler.add(GameEvent::RevealTile(x, y, self.tiles[x][y].clone()));

		if tile.state == TileState::Mine {
			event_handler.add(GameEvent::Lose(x, y, tile.clone()));

			event_handler.add(GameEvent::GameEnd(self.clone()));
			return Some(GameState::GameOver);
		};
		event_handler.add(GameEvent::SweepBegin);

		let mut scan_list = VecDeque::from([(x, y)]);
		let mut revealed: usize = 0;
		while scan_list.len() > 0 {
			for &scan_location in ADJACENT_WITHOUT_CENTER.iter() {
				if let Some((x, y)) = scan_list.front() {
					if let Some(old_tile) = self.get_tile(*x, *y) {
						if old_tile.adjacent > 0 {
							continue;
						}
						let x = *x as isize + scan_location.0;
						let y = *y as isize + scan_location.1;

						if x < 0 || y < 0 {
							continue;
						}
						let y = y as usize;
						let x = x as usize;
						if let Some(tile) = self.get_tile_mut(x, y) {
							if tile.swept {
								continue;
							}
							scan_list.push_back((x, y));
							tile.swept = true;
							revealed += 1;
							event_handler.add(GameEvent::RevealTile(x, y, tile.clone()));
						}
					}
				}
			}
			scan_list.pop_front();
		}
		self.revealed_tiles += revealed;
		if self.revealed_tiles == self.non_mine_tiles {
			event_handler.add(GameEvent::Win);
			event_handler.add(GameEvent::GameEnd(self.clone()));
			return Some(GameState::Victory);
		}
		None
	}

	fn generate(&mut self, avoid_x: usize, avoid_y: usize) {
		let width = self.width;
		let height = self.height;
		// Make list of all safe positions which are actually on board,
		//removing all which are before, or past the bounds of the board.
		let mut valid_safezone: Vec<(isize, isize)> = ADJACENT_WITH_CENTER.to_vec();
		valid_safezone.retain_mut(|pos: &mut (isize, isize)| {
			let adjusted_x = pos.0 + avoid_x as isize;
			let adjusted_y = pos.1 + avoid_y as isize;
			if adjusted_x >= 0 && adjusted_y >= 0 && adjusted_x < width as isize && adjusted_y < height as isize {
				pos.0 = adjusted_x;
				pos.1 = adjusted_y;
				return true;
			}
			false
		});
		for safe_pos in valid_safezone.iter() {
			let safe_x = safe_pos.0 as usize;
			let safe_y = safe_pos.1 as usize;
			self.tiles[safe_x][safe_y].safe = true;
		}

		let mut i = 0;

		let seed = (time::get_time() * 1000000.0) as u64;
		macroquad::rand::srand(seed);
		while i != self.mines {
			let x = macroquad::rand::gen_range(0, width);
			let y = macroquad::rand::gen_range(0, height);
			let mut tile = &mut self.tiles[x][y];

			if tile.state == TileState::Mine || tile.safe == true {
				continue;
			}

			tile.state = TileState::Mine;

			i += 1;
		}

		for x in 0..width {
			for y in 0..height {
				if let Some(tile) = self.get_tile(x, y) {
					if tile.state == TileState::Mine {
						for &scan_location in ADJACENT_WITH_CENTER.iter() {
							let new_x = x as isize + scan_location.0;
							let new_y = y as isize + scan_location.1;
							if new_x < 0 || new_y < 0 {
								continue;
							}
							let new_x = new_x as usize;
							let new_y = new_y as usize;
							if let Some(tile) = self.get_tile_mut(new_x, new_y) {
								tile.increment_adjacent();
							}
						}
					}
				}
			}
		}
		self.state = BoardState::Generated;
	}
}
