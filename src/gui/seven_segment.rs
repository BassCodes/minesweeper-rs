use macroquad::{
	prelude::vec2,
	ui::{widgets, Ui},
};
pub const WIDTH: usize = 13 * 2;
pub const HEIGHT: usize = 23 * 2;

use super::{texture_store::TextureStore, UIState};

pub fn draw_seven_segment(ui_state: &UIState, ui: &mut Ui, textures: &TextureStore, val: &[usize], x: usize, y: usize) {
	for (n, digit) in val.iter().enumerate() {
		let (scaled_width, scaled_height) = ui_state.pixel_screen_scale(WIDTH, HEIGHT);
		let (pos_x, pos_y) = ui_state.pixel_screen_offset(n * WIDTH + x, y);

		widgets::Texture::new(textures.numbers[*digit])
			.size(scaled_width, scaled_height)
			.position(vec2(pos_x, pos_y))
			.ui(ui);
	}
}

pub fn draw_seven_segment_unscaled(ui: &mut Ui, textures: &TextureStore, val: &[usize], x: usize, y: usize) {
	for (n, digit) in val.iter().enumerate() {
		let (pos_x, pos_y) = ((n * WIDTH + x) as f32, y as f32);

		widgets::Texture::new(textures.numbers[*digit])
			.size(WIDTH as f32, HEIGHT as f32)
			.position(vec2(pos_x, pos_y))
			.ui(ui);
	}
}
