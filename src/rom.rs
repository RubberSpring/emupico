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