use mlua::{FromLua, Lua, Result, UserData, UserDataMethods, Value};

use crate::palette::{ColorList, Color};
use crate::rom::GfxSection;

#[derive(Clone)]
pub struct VM {
	pub screen: Vec<u32>,
	pub time: f32,
	pub palette: ColorList,
	pub gfx_data: GfxSection
}

impl VM {
	pub fn new(gfx_data: &str) -> VM {
		VM {
			screen: vec![0; (128*128) as usize],
			time: 0.0,
			palette: ColorList(Color::VARIANTS),
			gfx_data: GfxSection::parse(gfx_data)
		}
	}

	pub fn screen_raw(&self) -> &[u8] {
		unsafe {
			std::slice::from_raw_parts(
				self.screen.as_ptr() as *const u8,
				self.screen.len()*4
			)
		}
	}

	pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
		self.screen[(y*128 + x) as usize] = color;
	}

	pub fn clear_screen(&mut self) {
		self.screen = vec![0; (128*128) as usize]
	}

	pub fn spr(&mut self, sprite_number: i32, x: i32, y: i32, w: Option<i32>, h: Option<i32>, flip_x: Option<bool>, flip_y: Option<bool>) {
		let w = w.unwrap_or(1) as usize;
		let h = h.unwrap_or(1) as usize;
		let flip_x = flip_x.unwrap_or(false);
		let flip_y = flip_y.unwrap_or(false);
		
		for dy in 0..h {
			for dx in 0..w {
				let sprite_base = sprite_number as usize;
				let sprite_idx = sprite_base + dy * 16 + dx;
				
				if sprite_idx >= 64 {
					continue;
				}
				
				let sprite_pixels = if let Some(sprite) = self.gfx_data.get_sprite(sprite_idx) {
					Some(sprite.pixels)
				} else {
					None
				};
				
				if let Some(pixels) = sprite_pixels {
					let screen_x = x + (dx as i32) * 8;
					let screen_y = y + (dy as i32) * 8;
					
					for py in 0..8 {
						for px in 0..8 {
							let src_x = if flip_x { 7 - px } else { px };
							let src_y = if flip_y { 7 - py } else { py };
							
							let color = pixels[src_y][src_x];
							let dest_x = screen_x + px as i32;
							let dest_y = screen_y + py as i32;
							
							if dest_x >= 0 && dest_x < 128 && dest_y >= 0 && dest_y < 128 {
								self.draw_pixel(dest_x as u32, dest_y as u32, color);
							}
						}
					}
				}
			}
		}
	}

	pub fn pal(&mut self, color1: i8, color2: i8, palette: Option<i8>) {
		match palette {
					Some(0) | None => self.palette.0[color1 as usize] = self.palette.0[color2 as usize],
					Some(change) => panic!("Palette change type {} is not implemented", change)
		}
	}
}

impl FromLua for VM {
	fn from_lua(value: Value, _: &Lua) -> Result<Self> {
		match value {
			Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
			_ => unreachable!(),
		}
	}
}

impl UserData for VM {
	fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {}
}
