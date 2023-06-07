use macroquad::ui::Ui;

use crate::gui::{
	seven_segment::{self, draw_seven_segment},
	texture_store::TextureStore,
};

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

		draw_seven_segment(
			ui_state,
			ui,
			textures,
			&self.digits,
			seven_segment::WIDTH * 2,
			(ui_state.top_offset - seven_segment::HEIGHT) / 2,
		);
	}
}
