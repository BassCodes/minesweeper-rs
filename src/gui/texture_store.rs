use image::ImageFormat;
use macroquad::texture::Texture2D;

use crate::sprite_loader::load_sprites;

use super::Language;

pub struct TextureStore {
	english_tiles: Vec<Texture2D>,
	japanese_tiles: Vec<Texture2D>,
	pub numbers: Vec<Texture2D>,
	pub smilies: Vec<Texture2D>,
	pub cog: Texture2D,
	pub lang: Language,
}
impl Default for TextureStore {
	fn default() -> Self {
		Self::new()
	}
}

impl TextureStore {
	pub fn new() -> Self {
		Self {
			numbers: load_sprites(include_bytes!("../../assets/numbers.png"), (26, 46), 1, 10).expect("Could not load sprites"),
			english_tiles: load_sprites(include_bytes!("../../assets/english_32x.png"), (32, 32), 2, 8)
				.expect("Could not load Tile Sprites"),
			japanese_tiles: load_sprites(include_bytes!("../../assets/japanese_32x.png"), (32, 32), 2, 8)
				.expect("Could not load Tile Sprites"),
			smilies: load_sprites(include_bytes!("../../assets/faces.png"), (48, 48), 1, 5).expect("Could not load face sprites"),
			cog: Texture2D::from_file_with_format(include_bytes!("../../assets/cog.png"), Some(ImageFormat::Png)),
			lang: Language::English,
		}
	}
	pub fn get_tiles(&self) -> &Vec<Texture2D> {
		return match self.lang {
			Language::English => &self.english_tiles,
			Language::Japanese => &self.japanese_tiles,
		};
	}
}
