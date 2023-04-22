use macroquad::{
	prelude::*,
	ui::{widgets, Ui},
};

use crate::gui::texture_store::TextureStore;

use super::UIState;

pub struct GUIFlagCounter {
	old_count: isize,
	digits: Vec<usize>,
}
impl Default for GUIFlagCounter {
	fn default() -> Self {
		Self {
			old_count: 0,
			digits: vec![0, 0, 0, 0],
		}
	}
}

impl GUIFlagCounter {
	pub fn render(&mut self, ui_state: &UIState, remaining: isize, ui: &mut Ui, textures: &TextureStore) {
		if self.old_count != remaining {
			let remaining_string = remaining.abs().to_string();
			let remaining_string = format!("{:0>2}", remaining_string);
			let digits: Vec<usize> = remaining_string.chars().map(|i| (i.to_digit(10u32).unwrap_or(0)) as usize).collect();
			let sign: usize = if remaining.signum() == -1 { 10 } else { 0 };
			let mut sign = vec![sign];
			sign.extend(digits);
			self.digits = sign;
			self.old_count = remaining;
		}

		let top = ui_state.top_offset;
		const WIDTH: usize = 13 * 2;
		const HEIGHT: usize = 23 * 2;
		let (scaled_width, scaled_height) = ui_state.pixel_screen_scale(WIDTH, HEIGHT);

		// let length = self.digits.len() as f32;
		for (x, digit) in self.digits.iter().enumerate() {
			let (pos_x, pos_y) = ui_state.pixel_screen_offset((x + 2) * WIDTH, (top - HEIGHT) / 2);
			widgets::Texture::new(textures.numbers[*digit])
				.size(scaled_width, scaled_height)
				.position(vec2(pos_x, pos_y))
				.ui(ui);
		}
	}
}
