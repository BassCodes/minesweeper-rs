#![allow(unused)]

use crate::{
	logic::{
		game_board::GameBoard,
		tile::{TileModifier, TileState},
	},
	util::Events,
	util::{ADJACENT_WITHOUT_CENTER, ADJACENT_WITH_CENTER},
};

use super::{top_menu::smile::SmileyState, GUIEvent, UIState};
use macroquad::prelude::*;
use std::default;

#[derive(Default)]
pub struct Highlighter {
	cursor_old: Option<(usize, usize)>,
	pub highlight: Highlight,
}
#[derive(Clone, Copy, Default)]
pub enum Highlight {
	#[default]
	None,
	Normal,
	Wide,
}

impl Highlighter {
	pub fn events(&mut self, ui_state: &UIState, event_handler: &mut Events<GUIEvent>, game_board: &mut GameBoard) {
		if !ui_state.frozen && ui_state.mouse_in_minefield {
			if is_mouse_button_pressed(MouseButton::Left) {
				self.highlight = Highlight::Normal;
			}
			if is_mouse_button_pressed(MouseButton::Middle) {
				self.highlight = Highlight::Wide;
				self.check_reveal(event_handler, ui_state, game_board)
			}
		}
		if is_mouse_button_released(MouseButton::Left) {
			self.reset_highlight(ui_state, event_handler);
			if !ui_state.frozen {
				event_handler.add(GUIEvent::SetSmileyState(SmileyState::Chillin));
			}
		}
		if is_mouse_button_released(MouseButton::Middle) {
			self.reset_highlight(ui_state, event_handler);
			if !ui_state.frozen {
				event_handler.add(GUIEvent::SetSmileyState(SmileyState::Chillin));
			}
		}
	}
	fn check_reveal(&self, event_handler: &mut Events<GUIEvent>, interface: &UIState, game_board: &mut GameBoard) {
		let (x, y) = interface.cursor;
		if let Some(tile) = game_board.get_tile_mut(x, y) {
			let adjacent_mines = tile.adjacent;
			if !tile.swept {
				return;
			}
			if adjacent_mines == 0 {
				return;
			}
			let mut adjacent_flags = 0;
			let mut near = ADJACENT_WITHOUT_CENTER.to_vec();
			near.retain_mut(|pos| {
				pos.0 += x as isize;
				pos.1 += y as isize;
				if pos.0 < 0 || pos.1 < 0 {
					return false;
				}
				let x = pos.0 as usize;
				let y = pos.1 as usize;
				if let Some(tile) = game_board.get_tile(x, y) {
					if let Some(TileModifier::Flagged) = tile.modifier {
						adjacent_flags += 1;
						return false;
					}
					return true;
				}
				false
			});
			if adjacent_flags == adjacent_mines {
				for empty_tile in near.iter() {
					let x = empty_tile.0 as usize;
					let y = empty_tile.1 as usize;
					event_handler.add(GUIEvent::ClickTile(x, y));
				}
			}
		}
	}
	pub fn highlight(&mut self, interface: &UIState, event_handler: &mut Events<GUIEvent>) {
		if interface.frozen {
			return;
		}
		let (x, y) = interface.cursor;
		match self.highlight {
			Highlight::None => {}
			Highlight::Normal => {
				event_handler.add(GUIEvent::SetSmileyState(SmileyState::Suspense));

				event_handler.add(GUIEvent::HighlightTile(x, y));
			}
			Highlight::Wide => {
				event_handler.add(GUIEvent::HighlightTile(x, y));
				event_handler.add(GUIEvent::SetSmileyState(SmileyState::Suspense));

				for pos in ADJACENT_WITHOUT_CENTER.iter() {
					let x = pos.0 + x as isize;
					let y = pos.1 + y as isize;
					if x < 0 || y < 0 || (interface.width as isize) <= x || (interface.height as isize) <= y {
						continue;
					}
					let x = x as usize;
					let y = y as usize;
					event_handler.add(GUIEvent::HighlightTile(x, y));
				}
			}
		}

		self.move_highlight(&interface, event_handler);
	}

	fn move_highlight(&mut self, interface: &UIState, event_handler: &mut Events<GUIEvent>) {
		if let Some((old_x, old_y)) = self.cursor_old {
			match self.highlight {
				Highlight::None => (),
				Highlight::Normal => {
					event_handler.add(GUIEvent::UnHighlightTile(old_x, old_y));
				}
				Highlight::Wide => {
					let (new_x, new_y) = interface.cursor;
					let mut old_highlighted_non_overlap = ADJACENT_WITH_CENTER.to_vec();
					// Retain all old highlighted points which do not overlap with new highlighted points
					old_highlighted_non_overlap.retain_mut(|pos: &mut (isize, isize)| {
						let x = pos.0 + old_x as isize;
						let y = pos.1 + old_y as isize;
						// Loop through old highlighted points to check if overlapping
						for p in ADJACENT_WITH_CENTER.iter() {
							let nx = p.0 + new_x as isize;
							let ny = p.1 + new_y as isize;
							if x == nx && y == ny {
								// Do not retain point if point at same location
								// found within new highlighted area
								return false;
							};
						}
						// Update x and y value of `old_highlighted_non_overlap` as they currently are the
						// initial values from the SCAN constant
						pos.0 = x;
						pos.1 = y;
						true
					});
					for pos in old_highlighted_non_overlap.iter() {
						let x = pos.0;
						let y = pos.1;
						if x >= 0 && y >= 0 && x < interface.width as isize && y < interface.height as isize {
							let x = x as usize;
							let y = y as usize;
							event_handler.add(GUIEvent::UnHighlightTile(x, y));
						}
					}
				}
			}
		}

		self.cursor_old = Some(interface.cursor);
	}

	fn reset_highlight(&mut self, interface: &UIState, event_handler: &mut Events<GUIEvent>) {
		if let Some((x, y)) = self.cursor_old {
			match self.highlight {
				Highlight::None => (),
				Highlight::Normal => {
					event_handler.add(GUIEvent::UnHighlightTile(x, y));
				}
				Highlight::Wide => {
					event_handler.add(GUIEvent::UnHighlightTile(x, y));

					for pos in ADJACENT_WITHOUT_CENTER.iter() {
						let x = pos.0 + x as isize;
						let y = pos.1 + y as isize;
						if x >= 0 && y >= 0 && x < interface.width as isize && y < interface.height as isize {
							let x = x as usize;
							let y = y as usize;
							event_handler.add(GUIEvent::UnHighlightTile(x, y));
						}
					}
				}
			}
		}

		self.highlight = Highlight::None;
		self.cursor_old = None;
	}
}
