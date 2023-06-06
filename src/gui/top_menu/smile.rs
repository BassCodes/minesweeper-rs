use macroquad::{
	prelude::*,
	ui::{widgets, Ui},
};

use crate::{
	gui::{texture_store::TextureStore, GUIEvent},
	util::Events,
};

use super::UIState;
#[derive(Clone, Copy)]
pub enum SmileyState {
	Chillin,
	PressedChillin,
	Suspense,
	Victory,
	Dead,
}
pub struct GUISmile {
	reset_pressed: bool,
	face: SmileyState,
}
impl Default for GUISmile {
	fn default() -> Self {
		Self {
			reset_pressed: false,
			face: SmileyState::Chillin,
		}
	}
}

const WIDTH: usize = 70;
const HEIGHT: usize = 70;

impl GUISmile {
	pub fn render(&mut self, ui_state: &UIState, ui: &mut Ui, event_handler: &mut Events<GUIEvent>, textures: &TextureStore) {
		let top_height = ui_state.top_offset;
		let top_width = ui_state.width * ui_state.tile_size;
		let pos_x = (top_width - HEIGHT) / 2;
		let pos_y = (top_height - HEIGHT) / 2;
		let (pos_x, pos_y) = ui_state.pixel_screen_offset(pos_x, pos_y);
		let (scaled_width, scaled_height) = ui_state.pixel_screen_scale(WIDTH, HEIGHT);
		if widgets::Button::new(textures.smilies[self.face as usize])
			.size(vec2(scaled_width, scaled_height))
			.position(vec2(pos_x, pos_y))
			.ui(ui)
		{
			self.reset_pressed = true;
		}
		if self.reset_pressed {
			self.set_smile(SmileyState::PressedChillin);

			if is_mouse_button_released(MouseButton::Left) {
				self.reset_pressed = false;
				self.set_smile(SmileyState::Chillin);

				event_handler.add(GUIEvent::ClickReset);
			}
		}
	}
	pub fn set_smile(&mut self, face: SmileyState) {
		self.face = face;
	}
}
