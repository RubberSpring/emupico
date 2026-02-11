use crate::palette::{code_to_color, color_to_hex};

pub enum RomSectionType {
	Header,
	Lua,
	Gfx,
	Label,
	Sfx,
	Music
}

pub struct RomSection {
	pub data: String,
	pub section_type: RomSectionType
}

impl RomSection {
	pub fn new(section_type: RomSectionType) -> RomSection {
		RomSection { 
			data: String::new(),
			section_type
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
	pub pixels: [[u32; 8]; 8]
}

impl Sprite {
	pub fn new() -> Self {
		Sprite {
			pixels: [[0x000000ff; 8]; 8]
		}
	}
}

#[derive(Clone)]
pub struct GfxSection {
	pub sprites: Vec<Sprite>
}

impl GfxSection {
	pub fn parse(data: &str) -> Self {
		let lines: Vec<&str> = data.lines()
			.filter(|line| !line.trim().is_empty())
			.collect();

		let mut sprites = vec![Sprite::new(); 64];
		
		for (line_idx, line) in lines.iter().enumerate() {
			let padded_line = if line.len() < 128 {
				format!("{}{}", line, "0".repeat(128 - line.len()))
			} else {
				line.to_string()
			};

			for sprite_x in 0..16 {
				let char_start = sprite_x * 8;
				let char_end = char_start + 8;
				
				if char_end > padded_line.len() {
					break;
				}

				let sprite_row = line_idx % 8;
				let sprite_sheet_row = line_idx / 8;
				let sprite_idx = sprite_sheet_row * 16 + sprite_x;

				if sprite_idx >= 64 {
					break;
				}

				let row_data = &padded_line[char_start..char_end];
				for (pixel_x, ch) in row_data.chars().enumerate() {
					if ch.to_digit(16).is_some() {
						let color = code_to_color(ch);
						let rgba = color_to_hex(color);
						sprites[sprite_idx].pixels[sprite_row][pixel_x] = rgba;
					}
				}
			}
		}

		GfxSection { sprites }
	}

	pub fn get_sprite(&self, index: usize) -> Option<&Sprite> {
		self.sprites.get(index)
	}

	pub fn get_sprite_mut(&mut self, index: usize) -> Option<&mut Sprite> {
		self.sprites.get_mut(index)
	}
}