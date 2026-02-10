use mlua::{FromLua, Lua, Result, UserData, UserDataMethods, Value};

#[derive(Clone)]
pub struct VM {
	pub screen: Vec<u32>,
	pub time: f32
}

impl VM {
	pub fn new() -> VM {
		VM {
			screen: vec![0; (128*128) as usize],
			time: 0.0
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
			methods.add_method("time", |_, vm, ()| {
				Ok(vm.time)
			});
		}
	}
