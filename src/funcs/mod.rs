pub mod dummy;

use mlua::{Function, Lua, Result};

use crate::vm::VM;

pub fn cls(lua: &Lua, _color: Option<i32>) -> Result<()>{
	lua.load("EMUPICO_VM:cls()").set_name("cls").exec()
}

pub fn time(lua: &Lua, ():()) -> Result<f32>{
	let globals = lua.globals();
	Ok(globals.get::<VM>("EMUPICO_VM")?.time)
}

pub fn cos(_: &Lua, x: f32) -> Result<f32> {
	Ok(x.cos())
}

pub fn pal(lua: &Lua, (color1, color2, palette):(i8, i8, Option<i8>)) -> Result<()>{
	let globals = lua.globals();
	lua.load("EMUPICO_PAL = EMUPICO_VM:pal()").set_name("pal").exec()?;
	let pal = globals.get::<Function>("EMUPICO_PAL")?;
	pal.call::<()>((color1, color2, palette))?;
	globals.raw_remove("EMUPICO_PAL")?;
	Ok(())
}