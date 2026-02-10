pub enum Color {
	Black,
	DarkBlue,
	DarkPurple,
	DarkGreen,
	Brown,
	DarkGray,
	LightGray,
	White,
	Red,
	Orange,
	Yellow,
	Green,
	Blue,
	Indigo,
	Pink,
	Peach
}

impl Color {
	pub const VARIANTS: [Color; 16] = [
		Color::Black,
		Color::DarkBlue,
		Color::DarkPurple,
		Color::DarkGreen,
		Color::Brown,
		Color::DarkGray,
		Color::LightGray,
		Color::White,
		Color::Red,
		Color::Orange,
		Color::Yellow,
		Color::Green,
		Color::Blue,
		Color::Indigo,
		Color::Pink,
		Color::Peach
	];
}

pub fn color_to_code(color: Color) -> i8 {
	color as i8
}

pub fn code_to_color(code: i8) -> Color {
	if let 0..=15 = code {
		unsafe {std::mem::transmute::<i8, Color>(code)}
	} else {
		panic!("color code {} is invalid", code)
	}
}

pub fn color_to_hex(color: Color) -> u32{
	match color {
		Color::Black => 0x000000ff,
		Color::DarkBlue => 0x1D2B53ff,
		Color::DarkPurple => 0x7E2553ff,
		Color::DarkGreen => 0x008751ff,
		Color::Brown => 0xAB5236ff,
		Color::DarkGray => 0x5F574Fff,
		Color::LightGray => 0xC2C3C7ff,
		Color::White => 0xFFF1E8ff,
		Color::Red => 0xFF004Dff,
		Color::Orange => 0xFFA300ff,
		Color::Yellow => 0xFFFF27ff,
		Color::Green => 0x00E756ff,
		Color::Blue => 0x29ADFFff,
		Color::Indigo => 0x83769Cff,
		Color::Pink => 0xFF77A8ff,
		Color::Peach =>0xFFCCAAff 
	}
}

pub struct ColorList(pub [Color; 16]);