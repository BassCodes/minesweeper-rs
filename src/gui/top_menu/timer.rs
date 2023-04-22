use macroquad::{
	prelude::*,
	ui::{widgets, Ui},
};

use crate::gui::texture_store::TextureStore;

use super::UIState;

pub struct GUITimer {
	old_time: u64,
	digits: Vec<usize>,
}
impl Default for GUITimer {
	fn default() -> Self {
		Self {
			old_time: 0u64,
			digits: vec![0, 0, 0],
		}
	}
}

impl GUITimer {
	pub fn render(&mut self, ui_state: &UIState, time: Option<f64>, ui: &mut Ui, textures: &TextureStore) {
		let time = time.unwrap_or_default() as u64;
		// Only update digits if time is different
		if self.old_time != time {
			self.old_time = time;
			let time_1 = time.to_string();
			let time_1 = format!("{:0>3}", time_1);
			let digits: Vec<usize> = time_1.chars().map(|i| (i.to_digit(10u32).unwrap_or(0)) as usize).collect();
			self.digits = digits;
		}
		let top = ui_state.top_offset;
		const WIDTH: usize = 13 * 2;
		const HEIGHT: usize = 23 * 2;
		let (scaled_width, scaled_height) = ui_state.pixel_screen_scale(WIDTH as usize, HEIGHT);
		let board_width = ui_state.width * ui_state.tile_size;
		let length = self.digits.len();
		for (x, digit) in self.digits.iter().enumerate() {
			let (pos_x, pos_y) = ui_state.pixel_screen_offset(WIDTH * x + board_width - WIDTH * (length + 2), (top - HEIGHT) / 2);
			widgets::Texture::new(textures.numbers[*digit])
				.size(scaled_width, scaled_height)
				.position(vec2(pos_x, pos_y))
				.ui(ui);
		}
	}
}
