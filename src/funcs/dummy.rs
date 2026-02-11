use mlua::{Lua, Result};

pub fn dummy_music(_: &Lua, (_song, _fade_length, _channel_mask): (i64, Option<i64>, Option<i64>)) -> Result<()> {
	println!("WARNING: music disabled: -m or --no-music flag");
	Ok(())
}

pub fn dummy_pal(_: &Lua, (color1, color2, palette):(i8, i8, Option<i8>)) -> Result<()> {
	println!("WARNING: pal({:?},{:?},{:?}) only has unstable impementations, enable with -u", color1, color2, palette);
	Ok(())
}