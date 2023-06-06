mod board_render;
mod highlighter;
pub mod settings_menu;
pub mod texture_store;
mod tile_render;
pub mod top_menu;

use crate::{logic::game_board::ModifyMode, util::Events};

use self::{
	highlighter::Highlighter,
	settings_menu::SettingsMenu,
	texture_store::TextureStore,
	top_menu::{smile::SmileyState, GUITop},
};
use macroquad::prelude::*;
#[derive(Default, Copy, Clone, Debug)]
pub enum Language {
	#[default]
	English,
	Japanese,
}

pub enum GUIEvent {
	ClickReset,
	OpenSettings,
	CloseSettings,
	SwitchLanguage(Language),
	ClickTile(usize, usize),
	ModifyTile(usize, usize),
	HighlightTile(usize, usize),
	UnHighlightTile(usize, usize),
	CreateNewGame(usize, usize, usize),
	SetQuestionMode(ModifyMode),
	SetSmileyState(SmileyState),
}

#[derive(Debug, Default)]
pub struct UIState {
	pub width: usize,
	pub height: usize,
	pub tile_size: usize,
	pub mouse_in_minefield: bool,
	pub top_offset: usize, // Space above board to be used for other ui
	pub reveal_all: bool,
	pub letterbox: (f32, f32),
	pub scale: f32,
	pub frozen: bool,
	pub cursor: (usize, usize),
	pub settings_open: bool,
	pub language: Language,
}
impl UIState {
	pub fn new(width: usize, height: usize, tile_size: usize, top_offset: usize) -> Self {
		return Self {
			width,
			height,
			tile_size,
			top_offset,
			..Default::default()
		};
	}
	pub fn update_dimensions(&mut self, width: usize, height: usize) {
		self.width = width;
		self.height = height;
	}
	pub fn update_letterbox(&mut self, screen_width: f32, screen_height: f32) {
		let game_aspect_ratio = self.width as f32 / (self.height as f32 + self.top_offset as f32 / self.tile_size as f32);

		let screen_aspect_ratio = screen_width / screen_height;
		if game_aspect_ratio > screen_aspect_ratio {
			self.scale = screen_width / (self.width * self.tile_size) as f32;
		} else {
			self.scale = screen_height / ((self.height * self.tile_size) + self.top_offset) as f32;
		}

		let total_height = (self.height * self.tile_size + self.top_offset) as f32 * self.scale;

		let total_width = (self.width * self.tile_size) as f32 * self.scale;

		if total_height < screen_height {
			self.letterbox.0 = 0f32;
			self.letterbox.1 = (screen_height - total_height) * 0.5;
		} else {
			self.letterbox.0 = (screen_width - total_width) * 0.5;
			self.letterbox.1 = 0f32;
		}
	}
}

impl UIState {
	pub fn pixel_screen_offset(&self, x: usize, y: usize) -> (f32, f32) {
		let (x, y) = self.pixel_screen_scale(x, y);
		let x = x + self.letterbox.0;
		let y = y + self.letterbox.1;
		return (x, y);
	}
	pub fn pixel_screen_scale(&self, x: usize, y: usize) -> (f32, f32) {
		let x = x as f32;
		let y = y as f32;
		return (x * self.scale, y * self.scale);
	}
}

#[derive(Default)]
pub struct GameUI {
	pub event_handler: Events<GUIEvent>,
	pub highlighter: Highlighter,
	pub state: UIState,
	pub settings_menu: SettingsMenu,
	pub texture_store: TextureStore,
	pub top_menu: GUITop,
}

impl GameUI {
	pub fn new(settings: UIState) -> Self {
		let set = settings;
		return Self {
			state: set,
			..Default::default()
		};
	}
	pub fn is_valid_position(&self, x: usize, y: usize) -> bool {
		if x < self.state.width && y < self.state.height {
			return true;
		}
		false
	}

	pub fn set_cursor(&mut self, x: usize, y: usize) {
		self.state.cursor = (x, y);
	}

	pub fn clear(&mut self) {
		self.state.frozen = false;
		self.state.reveal_all = false;
		self.event_handler.clear();
	}

	pub fn to_coordinate_system(&self, x: f32, y: f32) -> Option<(usize, usize)> {
		let y = y - self.state.top_offset as f32;
		if x < 0.0 || y < 0.0 {
			return None;
		}
		let y = (y / self.state.tile_size as f32) as usize;
		let x = (x / self.state.tile_size as f32) as usize;
		if !self.is_valid_position(x, y) {
			return None;
		}
		return Some((x, y));
	}
}
