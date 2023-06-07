use crate::{logic::game_board::ModifyMode, util::Events};

use super::{seven_segment::draw_seven_segment_unscaled, texture_store::TextureStore, GUIEvent, Language, UIState};
use macroquad::{
	hash,
	prelude::*,
	ui::{root_ui, widgets, Skin, Ui},
};

const NEW_GAME_HEIGHT: f32 = 40f32;
const NEW_GAME_WIDTH: f32 = 250f32;
const BUTTON_MENU_WIDTH: f32 = 250f32;
const BUTTON_SIZE: f32 = 100f32;
const BUTTON_MENU_Y: f32 = 400f32;
const BUTTON_MENU_LABEL_HEIGHT: f32 = 20f32;

const MIN_MINEFIELD_WIDTH: usize = 5;
const MAX_MINEFIELD_WIDTH: usize = 100;
const MIN_MINEFIELD_HEIGHT: usize = 5;
const MAX_MINEFIELD_HEIGHT: usize = 100;

pub struct SettingsMenu {
	mines: usize,
	width: usize,
	height: usize,
	board_modify_mode: ModifyMode,
}
impl Default for SettingsMenu {
	fn default() -> Self {
		Self {
			mines: 99,
			width: 30,
			height: 16,
			board_modify_mode: ModifyMode::Flag,
		}
	}
}

impl SettingsMenu {
	pub fn render(
		&mut self,
		ui_state: &UIState,
		event_handler: &mut Events<GUIEvent>,
		textures: &TextureStore,
		skin: &Skin,
		exit_button_skin: &Skin,
	) {
		let screen_width = screen_width();
		let screen_height = screen_height();
		let background_color = Color::from_rgba(192, 192, 192, 255);

		root_ui().window(hash!(), vec2(0., 0.), vec2(screen_width, screen_height), |ui| {
			draw_rectangle(0f32, 0f32, screen_width, screen_height, background_color);
			ui.push_skin(&exit_button_skin);
			if widgets::Button::new("").size(vec2(50.0, 50.0)).position(vec2(0f32, 0f32)).ui(ui) {
				event_handler.add(GUIEvent::CloseSettings)
			}
			ui.pop_skin();
			ui.push_skin(&skin);
			let half_screen_width = screen_width * 0.5;

			render_counter(
				&mut self.width,
				ui,
				&textures,
				vec2(half_screen_width, 100f32),
				"Minefield Width",
				MIN_MINEFIELD_WIDTH,
				MAX_MINEFIELD_WIDTH,
			);
			render_counter(
				&mut self.height,
				ui,
				&textures,
				vec2(half_screen_width, 200f32),
				"Minefield Height",
				MIN_MINEFIELD_HEIGHT,
				MAX_MINEFIELD_HEIGHT,
			);
			render_counter(
				&mut self.mines,
				ui,
				&textures,
				vec2(half_screen_width, 300f32),
				"Mines",
				1,
				self.width * self.height - 10,
			);
			if widgets::Button::new("New Game")
				.size(vec2(NEW_GAME_WIDTH, NEW_GAME_HEIGHT))
				.position(vec2((screen_width - NEW_GAME_WIDTH) * 0.5, 0.0))
				.ui(ui)
			{
				event_handler.add(GUIEvent::CreateNewGame(self.width, self.height, self.mines));
				event_handler.add(GUIEvent::CloseSettings);
			}
			let language_button_x = (screen_width - BUTTON_MENU_WIDTH) * 0.5;
			let question_button_x = (screen_width - BUTTON_MENU_WIDTH) * 0.5 + (BUTTON_MENU_WIDTH - BUTTON_SIZE);

			widgets::Label::new("Language")
				.position(vec2(language_button_x, BUTTON_MENU_Y - BUTTON_MENU_LABEL_HEIGHT))
				.size(vec2(BUTTON_SIZE, BUTTON_MENU_LABEL_HEIGHT))
				.ui(ui);
			widgets::Label::new("Question Marking")
				.position(vec2(question_button_x, BUTTON_MENU_Y - BUTTON_MENU_LABEL_HEIGHT))
				.size(vec2(BUTTON_SIZE, BUTTON_MENU_LABEL_HEIGHT))
				.ui(ui);
			if let Language::English = ui_state.language {
				if widgets::Button::new("English")
					.size(vec2(BUTTON_SIZE, BUTTON_SIZE))
					.position(vec2(language_button_x, BUTTON_MENU_Y))
					.ui(ui)
				{
					event_handler.add(GUIEvent::SwitchLanguage(Language::Japanese));
				}
			} else {
				if widgets::Button::new("Japanese")
					.size(vec2(BUTTON_SIZE, BUTTON_SIZE))
					.position(vec2(language_button_x, BUTTON_MENU_Y))
					.ui(ui)
				{
					event_handler.add(GUIEvent::SwitchLanguage(Language::English));
				}
			}
			if let ModifyMode::Question = self.board_modify_mode {
				if widgets::Button::new("ON")
					.size(vec2(BUTTON_SIZE, BUTTON_SIZE))
					.position(vec2(question_button_x, BUTTON_MENU_Y))
					.ui(ui)
				{
					self.board_modify_mode = ModifyMode::Flag;
					event_handler.add(GUIEvent::SetQuestionMode(ModifyMode::Flag));
				}
			} else {
				if widgets::Button::new("OFF")
					.size(vec2(BUTTON_SIZE, BUTTON_SIZE))
					.position(vec2(question_button_x, BUTTON_MENU_Y))
					.ui(ui)
				{
					self.board_modify_mode = ModifyMode::Question;
					event_handler.add(GUIEvent::SetQuestionMode(ModifyMode::Question));
				}
			}
		});
	}
}

const COUNTER_DIGIT_WIDTH: f32 = 13f32 * 2.0;
const COUNTER_DIGIT_HEIGHT: f32 = 23f32 * 2.0;
const COUNTER_BUTTON_HEIGHT: f32 = 30f32;
const COUNTER_BUTTON_MARGIN: f32 = 10f32;
const BUTTON_OFFSET_HEIGHT: f32 = (COUNTER_DIGIT_HEIGHT - COUNTER_BUTTON_HEIGHT) * 0.5;

fn render_counter(count: &mut usize, ui: &mut Ui, textures: &TextureStore, position: Vec2, title: &str, min: usize, max: usize) {
	let digits: Vec<usize> = {
		let digits = count.to_string();
		let digits = format!("{:0>3}", digits);
		digits.chars().map(|i| (i.to_digit(10u32).unwrap_or(0)) as usize).collect()
	};

	let counter_width = digits.len() as f32 * COUNTER_DIGIT_WIDTH;
	let position = position - vec2(counter_width * 0.5, 0.0);

	draw_seven_segment_unscaled(ui, textures, &digits, position.x as usize, position.y as usize);
	if widgets::Button::new("+")
		.size(vec2(COUNTER_BUTTON_HEIGHT, COUNTER_BUTTON_HEIGHT))
		.position(position + vec2(counter_width + COUNTER_BUTTON_MARGIN, BUTTON_OFFSET_HEIGHT))
		.ui(ui)
	{
		*count += 1;
	}
	if widgets::Button::new("-")
		.size(vec2(COUNTER_BUTTON_HEIGHT, COUNTER_BUTTON_HEIGHT))
		.position(position - vec2(COUNTER_BUTTON_HEIGHT + COUNTER_BUTTON_MARGIN, -BUTTON_OFFSET_HEIGHT))
		.ui(ui)
	{
		if *count > min {
			*count -= 1;
		}
	}
	if widgets::Button::new("++")
		.size(vec2(COUNTER_BUTTON_HEIGHT, COUNTER_BUTTON_HEIGHT))
		.position(
			position
				+ vec2(
					counter_width + COUNTER_BUTTON_MARGIN * 2.0 + COUNTER_BUTTON_HEIGHT,
					BUTTON_OFFSET_HEIGHT,
				),
		)
		.ui(ui)
	{
		*count += 10;
	}
	if widgets::Button::new("--")
		.size(vec2(COUNTER_BUTTON_HEIGHT, COUNTER_BUTTON_HEIGHT))
		.position(position - vec2((COUNTER_BUTTON_HEIGHT + COUNTER_BUTTON_MARGIN) * 2.0, -BUTTON_OFFSET_HEIGHT))
		.ui(ui)
	{
		if *count as isize - 10 > min as isize {
			*count -= 10;
		} else {
			*count = min;
		}
	}
	if *count > max {
		*count = max;
	}
	if *count < min {
		*count = min;
	}
	let label_height = 20f32;
	widgets::Label::new(title)
		.position(position - vec2(0f32, label_height))
		.size(vec2(counter_width, label_height))
		.ui(ui);
}
