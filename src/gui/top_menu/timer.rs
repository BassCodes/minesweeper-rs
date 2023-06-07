use macroquad::{prelude::*, ui::Ui};

use crate::gui::{
	seven_segment::{self, draw_seven_segment},
	texture_store::TextureStore,
};

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
		let board_width = ui_state.width * ui_state.tile_size;

		draw_seven_segment(
			ui_state,
			ui,
			textures,
			&self.digits,
			board_width - seven_segment::WIDTH * (self.digits.len() + 2),
			(ui_state.top_offset - seven_segment::HEIGHT) / 2,
		);
	}
}
