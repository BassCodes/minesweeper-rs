use gui::top_menu::smile::SmileyState;
use gui::{GUIEvent, GameUI, UIState};
use logic::{GameEvent, Minesweeper};
use macroquad::{
	prelude::*,
	ui::{root_ui, Skin},
	Window,
};
mod gui;
mod logic;
mod sprite_loader;
mod util;

fn main() {
	let width = (30 * 32) as i32;
	let height = (16 * 32) as i32 + 100;
	Window::from_config(
		Conf {
			sample_count: 2,
			window_title: String::from("Minesweeper"),
			high_dpi: false,
			window_width: width,
			window_height: height,
			..Default::default()
		},
		run(),
	);
}

async fn run() {
	let mut game_logic = Minesweeper::new(30, 16, 99).unwrap();

	let top_buffer = 100; //px
	let mut interface = GameUI::new(UIState::new(30, 16, 32, top_buffer));

	let skin = {
		let button_style = root_ui().style_builder().build();
		let scrollbar_handle_style = root_ui().style_builder().build();
		let window_style = root_ui().style_builder().color(Color::from_rgba(0, 0, 0, 0)).build();
		Skin {
			scroll_width: 0f32,
			scrollbar_handle_style,
			button_style,
			window_style,
			..root_ui().default_skin()
		}
	};

	let settings_skin = {
		let button_texture = Image::from_file_with_format(include_bytes!("../assets/button.png"), Some(ImageFormat::Png));
		let button_clicked_texture =
			Image::from_file_with_format(include_bytes!("../assets/button_clicked.png"), Some(ImageFormat::Png));
		let label_style = root_ui().style_builder().font_size(20).build();
		let button_style = root_ui()
			.style_builder()
			.font_size(20)
			.background(button_texture)
			.background_clicked(button_clicked_texture)
			.build();
		Skin {
			scroll_width: 0f32,
			button_style,
			label_style,
			..root_ui().default_skin()
		}
	};
	let settings_skin_exit = {
		let button_texture = Image::from_file_with_format(include_bytes!("../assets/exit_button.png"), Some(ImageFormat::Png));
		let button_hover_texture =
			Image::from_file_with_format(include_bytes!("../assets/exit_button_hover.png"), Some(ImageFormat::Png));
		let button_style = root_ui()
			.style_builder()
			.background(button_texture)
			.background_hovered(button_hover_texture)
			.build();
		Skin {
			scroll_width: 0f32,
			button_style,
			..root_ui().default_skin()
		}
	};

	let mut old_screen_size = (0.0, 0.0);
	let background_color = Color::from_rgba(123, 123, 123, 255);
	loop {
		root_ui().push_skin(&skin);
		clear_background(background_color);

		while let Some(ge) = game_logic.events.next() {
			match ge {
				GameEvent::Lose(_, _, _) => {
					interface.state.frozen = true;
					interface.event_handler.add(GUIEvent::SetSmileyState(SmileyState::Dead));
					interface.state.reveal_all = true;
				}
				GameEvent::Win => {
					interface.state.frozen = true;
					interface.event_handler.add(GUIEvent::SetSmileyState(SmileyState::Victory));
					interface.state.reveal_all = true;
				}
				GameEvent::Reset => {
					interface.clear();
				}
				_ => (),
			}
		}
		{
			let screen_width = screen_width();
			let screen_height = screen_height();
			// Only update letterboxing calculations when screen size changes
			if (screen_width, screen_height) != old_screen_size {
				interface.state.update_letterbox(screen_width, screen_height);
				old_screen_size = (screen_width, screen_height);
			}

			let (mouse_x, mouse_y) = mouse_position();
			let (min_x, min_y) = interface.state.pixel_screen_offset(10, 10 + interface.state.top_offset);
			let tile_size = interface.state.tile_size;
			let (max_x, max_y) = interface.state.pixel_screen_offset(
				interface.state.width * tile_size - 10,
				interface.state.height * tile_size + interface.state.top_offset - 10,
			);

			if mouse_x < min_x || mouse_y < min_y || mouse_x > max_x || mouse_y > max_y {
				interface.state.mouse_in_minefield = false;
			} else {
				interface.state.mouse_in_minefield = true;
				if let Some((x, y)) = interface.to_coordinate_system(
					(mouse_x - interface.state.letterbox.0) / interface.state.scale,
					(mouse_y - interface.state.letterbox.1) / interface.state.scale,
				) {
					interface.set_cursor(x, y);
				}
			}
			if interface.state.settings_open {
				interface.settings_menu.render(
					&interface.state,
					&mut interface.event_handler,
					&interface.texture_store,
					&settings_skin,
					&settings_skin_exit,
				);
			} else {
				interface.highlighter.events(&interface.state, &mut interface.event_handler, &mut game_logic.board);
				interface.highlighter.highlight(&interface.state, &mut interface.event_handler);
				game_logic.board.render(&interface.texture_store, &interface.state);
				game_logic.board.events(&interface.state, &mut interface.event_handler);
				interface.top_menu.render(
					&interface.state,
					&game_logic,
					&mut interface.event_handler,
					&interface.texture_store,
				);
			}
		}

		while let Some(ue) = interface.event_handler.next() {
			match ue {
				GUIEvent::ClickReset => {
					game_logic.reset();
					interface.state.mouse_in_minefield = false
				}
				GUIEvent::ClickTile(x, y) => {
					game_logic.reveal(x, y);
				}
				GUIEvent::ModifyTile(x, y) => game_logic.modify(x, y),
				GUIEvent::HighlightTile(x, y) => game_logic.highlight(x, y),
				GUIEvent::UnHighlightTile(x, y) => game_logic.remove_highlight(x, y),
				GUIEvent::OpenSettings => {
					interface.state.mouse_in_minefield = false;
					interface.state.frozen = true;
					interface.state.settings_open = true;
				}
				GUIEvent::CloseSettings => {
					interface.state.frozen = false;
					interface.state.settings_open = false;
				}
				GUIEvent::SwitchLanguage(lang) => {
					interface.state.language = lang;
					interface.texture_store.lang = lang;
				}
				GUIEvent::CreateNewGame(width, height, mines) => {
					interface.state.frozen = false;
					interface.state.update_dimensions(width, height);
					game_logic.update_and_reset(width, height, mines);
					interface.state.update_letterbox(screen_width(), screen_height())
				}
				GUIEvent::SetQuestionMode(mode) => game_logic.board.modify_mode = mode,
				GUIEvent::SetSmileyState(smiley_state) => interface.top_menu.smile.set_smile(smiley_state),
			}
		}

		next_frame().await;
	}
}
