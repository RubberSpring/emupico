pub struct VM {
	pub screen: Vec<u32>
}

impl VM {
	pub fn new() -> VM {
		VM {
			screen: vec![0; (128*128) as usize]
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