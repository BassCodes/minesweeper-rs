use crate::logic::tile::{Tile, TileModifier, TileState};

pub enum TileIndex {
	Hidden,
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
	pub fn render(self, show_all: bool) -> TileIndex {
		if self.swept && self.state == TileState::Mine {
			return TileIndex::Explosion;
		}
		if show_all {
			if let Some(modifier) = self.modifier {
				if modifier == TileModifier::Flagged {
					if self.state == TileState::Mine {
						return TileIndex::Flag;
					} else {
						return TileIndex::FalseFlagMine;
					}
				}
			}
			if self.state == TileState::Mine {
				return TileIndex::RevealedMine;
			}
		}
		if self.swept {
			if self.state == TileState::Mine {
				TileIndex::Explosion
			} else {
				match self.adjacent {
					0 => TileIndex::Revealed,
					1 => TileIndex::One,
					2 => TileIndex::Two,
					3 => TileIndex::Three,
					4 => TileIndex::Four,
					5 => TileIndex::Five,
					6 => TileIndex::Six,
					7 => TileIndex::Seven,
					8 => TileIndex::Eight,
					_ => TileIndex::RevealedQuestion,
				}
			}
		} else {
			if let Some(modif) = self.modifier {
				match modif {
					TileModifier::Flagged => TileIndex::Flag,
					TileModifier::Unsure => TileIndex::Question,
				}
			} else if self.highlighted {
				TileIndex::Revealed
			} else {
				TileIndex::Hidden
			}
		}
	}
}
