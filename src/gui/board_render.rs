use crate::logic::game_board::GameBoard;
use macroquad::prelude::*;

use super::{texture_store::TextureStore, ui_event::GUIEvents, UIState};
impl GameBoard {
	pub fn render(&self, textures: &TextureStore, settings: &UIState) {
		// dbg!(&settings.top_offset, &settings.render_scale);
		let tile_size = settings.tile_size;
		let (scaled_tile, _) = settings.pixel_screen_scale(tile_size, 0);
		for (x, col) in self.tiles.iter().enumerate() {
			for (y, tile) in col.iter().enumerate() {
				let (x, y) = settings.pixel_screen_offset(x * tile_size, y * tile_size + settings.top_offset);
				draw_texture_ex(
					textures.get_tiles()[tile.render(settings.reveal_all) as usize],
					x,
					y,
					WHITE,
					DrawTextureParams {
						dest_size: Some(vec2(scaled_tile, scaled_tile)),
						source: Some(Rect {
							x: 0.0,
							y: 0.0,
							w: 32.0,
							h: 32.0,
						}),
						rotation: 0.0,
						flip_x: false,
						flip_y: false,
						pivot: None,
					},
				);
			}
		}
	}
	pub fn events(&self, settings: &UIState, event_handler: &mut GUIEvents) {
		if settings.mouse_in_minefield && !settings.frozen {
			if is_mouse_button_released(MouseButton::Left) {
				event_handler.add(super::ui_event::GUIEvent::ClickTile(settings.cursor.0, settings.cursor.1))
			}
			if is_mouse_button_released(MouseButton::Right) {
				event_handler.add(super::ui_event::GUIEvent::ModifyTile(settings.cursor.0, settings.cursor.1))
			}
		}
	}
}
