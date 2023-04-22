use macroquad::{
	hash,
	ui::{root_ui, widgets},
};

pub mod flag_counter;
pub mod smile;
pub mod timer;

use crate::logic::Minesweeper;

use self::{flag_counter::GUIFlagCounter, smile::GUISmile, timer::GUITimer};

use super::{
	texture_store::TextureStore,
	ui_event::{GUIEvent, GUIEvents},
	UIState,
};
use macroquad::prelude::*;
#[derive(Default)]
pub struct GUITop {
	pub flag_counter: GUIFlagCounter,
	pub timer: GUITimer,
	pub smile: GUISmile,
}

impl GUITop {
	pub fn render(
		&mut self,
		ui_state: &UIState,
		game_logic: &Minesweeper,
		event_handler: &mut GUIEvents,
		textures: &TextureStore,
	) {
		let background_color = Color::from_rgba(192, 192, 192, 255);
		let border_color = Color::from_rgba(123, 123, 123, 255);
		const BORDER_MARGIN: f32 = 3.0;
		let top_offset = ui_state.top_offset as f32 * ui_state.scale + ui_state.letterbox.1;
		let (x, y) = ui_state.pixel_screen_offset(0, 0);
		let board_width = ui_state.width * ui_state.tile_size;
		let (scaled_top_width, scaled_top_offset) = ui_state.pixel_screen_scale(board_width, ui_state.top_offset);

		root_ui().window(hash!(), vec2(0., 0.), vec2(screen_width(), top_offset), |ui| {
			// Macroquad does not support window border colors so I hacked it in.
			draw_rectangle(x, y, scaled_top_width, scaled_top_offset, border_color);
			draw_rectangle(
				BORDER_MARGIN + x,
				BORDER_MARGIN + y,
				scaled_top_width - BORDER_MARGIN * 2.0,
				scaled_top_offset - BORDER_MARGIN * 2.0,
				background_color,
			);
			// done with the hackey bits.

			//Settings button. (didn't have enough logic to be broken into it's own file.)
			{
				const WIDTH: usize = 35;
				const HEIGHT: usize = 35;
				let pos_y = (ui_state.top_offset - HEIGHT) / 2;
				let pos_x = (13 * 2 * 2 - WIDTH) / 2;
				let (scaled_width, scaled_height) = ui_state.pixel_screen_scale(WIDTH as usize, HEIGHT);
				let (pos_x, pos_y) = ui_state.pixel_screen_offset(pos_x, pos_y);
				if widgets::Button::new(textures.cog)
					.size(vec2(scaled_width, scaled_height))
					.position(vec2(pos_x, pos_y))
					.ui(ui)
				{
					if ui_state.settings_open {
						event_handler.add(GUIEvent::CloseSettings)
					} else {
						event_handler.add(GUIEvent::OpenSettings)
					}
				}
			}

			self.timer.render(&ui_state, game_logic.get_time(), ui, &textures);
			self.smile.render(&ui_state, ui, event_handler, &textures);
			self.flag_counter.render(&ui_state, game_logic.board.remaining_flags(), ui, &textures);
		});
	}
}
