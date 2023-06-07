use crate::logic::tile::{Tile, TileModifier, TileState};

#[repr(usize)]
pub enum TileIndex {
	Unknown,
	Revealed,
	Flag,
	Question,
	RevealedQuestion, //UNUSED. I'm not sure how this can come to be given that the win condition is for all empty tiles to be revealed.
	RevealedMine,
	Explosion,
	FalseFlagMine,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
}

impl Tile {
	pub fn render(self, game_over: bool) -> TileIndex {
		// Behold: the match statement from hell!
		match (
			self.state,
			self.modifier,
			self.adjacent,
			game_over,
			self.swept,
			self.highlighted,
		) {
			// Has mine, clicked mine: BOOM!
			(TileState::Mine, _, _, _, true, _) => TileIndex::Explosion,
			// Has mine, has flag, and game is over: True Flag
			(TileState::Mine, Some(TileModifier::Flagged), _, true, _, _) => TileIndex::Flag,
			// Has flag, is not Mine, and game is over: False flag
			(TileState::Empty, Some(TileModifier::Flagged), _, true, _, _) => TileIndex::FalseFlagMine,
			// Revealed mine after game is over
			(TileState::Mine, _, _, true, _, _) => TileIndex::RevealedMine,
			// Revealed tiles with adjacent tile count
			(TileState::Empty, _, 0, _, true, _) => TileIndex::Revealed,
			(TileState::Empty, _, 1, _, true, _) => TileIndex::One,
			(TileState::Empty, _, 2, _, true, _) => TileIndex::Two,
			(TileState::Empty, _, 3, _, true, _) => TileIndex::Three,
			(TileState::Empty, _, 4, _, true, _) => TileIndex::Four,
			(TileState::Empty, _, 5, _, true, _) => TileIndex::Five,
			(TileState::Empty, _, 6, _, true, _) => TileIndex::Six,
			(TileState::Empty, _, 7, _, true, _) => TileIndex::Seven,
			(TileState::Empty, _, 8, _, true, _) => TileIndex::Eight,
			// Flag modifier
			(_, Some(TileModifier::Flagged), _, _, _, _) => TileIndex::Flag,
			// Question mark modifier
			(_, Some(TileModifier::Unsure), _, _, _, _) => TileIndex::Question,
			// No modifier, not swept, but highlighted
			(_, None, _, _, false, true) => TileIndex::Revealed,
			// No modifier, Not swept, and not highlighted: Unknown tile
			(_, None, _, _, false, false) => TileIndex::Unknown,
			// unsigned 8 bit integer has too much range for the adjacent tiles count, creating an invalid state
			// from 9 onward. This last clause is to catch if somehow this invalid state occurs, and display
			// the invalid tile in that case.
			(TileState::Empty, _, 9..=u8::MAX, _, true, _) => TileIndex::RevealedQuestion,
		}
	}
}
