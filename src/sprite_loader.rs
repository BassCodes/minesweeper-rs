use std::error::Error;

use image::{load_from_memory, EncodableLayout};
use macroquad::texture::{FilterMode, Texture2D};
pub fn load_sprites(bytes: &[u8], tile_size: (u32, u32), rows: usize, columns: usize) -> Result<Vec<Texture2D>, Box<dyn Error>> {
	let sprite_sheet = load_from_memory(bytes)?.to_rgba8();

	let mut sprite_list: Vec<Texture2D> = vec![];
	let (tile_width, tile_height) = tile_size;

	for i in 0..(rows * columns) {
		let x = (i % columns) as u32;
		let y = (i / columns) as u32;
		let tile = image::imageops::crop_imm(&sprite_sheet, x * tile_width, y * tile_height, tile_width, tile_height).to_image();

		tile.as_bytes();
		let tile_width = tile_width as u16;
		let tile_height = tile_height as u16;
		let tex = Texture2D::from_rgba8(tile_width, tile_height, &tile);
		tex.set_filter(FilterMode::Nearest);

		sprite_list.push(tex);
	}
	Ok(sprite_list)
}
