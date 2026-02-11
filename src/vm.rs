use mlua::{FromLua, Lua, Result, UserData, UserDataMethods, Value};

use crate::palette::{ColorList, Color};

#[derive(Clone)]
pub struct VM {
	pub screen: Vec<u32>,
	pub time: f32,
	pub palette: ColorList
}

impl VM {
	pub fn new() -> VM {
		VM {
			screen: vec![0; (128*128) as usize],
			time: 0.0,
			palette: ColorList(Color::VARIANTS)
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
		fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
			methods.add_method_mut("cls", |_, vm, ()| {
				vm.clear_screen();
				Ok(())
			});
			methods.add_method_mut("pal", |_, vm, (color1, color2, palette):(i8, i8, Option<i8>)| {
				match palette {
					Some(0) | None => vm.palette.0[color1 as usize] = vm.palette.0[color2 as usize],
					Some(change) => panic!("Palette change type {} is not implemented", change)
				}
				Ok(())
			});
		}
	}
