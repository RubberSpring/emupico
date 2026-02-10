use mlua::{Lua, Result};

pub fn dummy_music(_: &Lua, (_song, _fade_length, _channel_mask): (i64, Option<i64>, Option<i64>)) -> Result<()> {
	println!("WARNING music disabled: -m or --no-music flag");
	Ok(())
}