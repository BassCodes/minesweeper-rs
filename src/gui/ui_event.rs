use crate::logic::game_board::ModifyMode;

use super::{top_menu::smile::SmileyState, Language};

pub enum GUIEvent {
	ClickReset,
	OpenSettings,
	CloseSettings,
	SwitchLanguage(Language),
	ClickTile(usize, usize),
	ModifyTile(usize, usize),
	HighlightTile(usize, usize),
	UnHighlightTile(usize, usize),
	CreateNewGame(usize, usize, usize),
	SetQuestionMode(ModifyMode),
	SetSmileyState(SmileyState),
}
#[derive(Default)]
pub struct GUIEvents {
	events: Vec<GUIEvent>,
}

impl GUIEvents {
	pub fn add(&mut self, event: GUIEvent) {
		self.events.push(event);
	}
	pub fn next(&mut self) -> Option<GUIEvent> {
		if self.events.len() > 0 {
			self.events.pop()
		} else {
			None
		}
	}
	pub fn clear(&mut self) {
		self.events.clear();
	}
}
