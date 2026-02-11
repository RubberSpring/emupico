pub mod dummy;

use mlua::{Function, Lua, Result};

use crate::vm::VM;

pub fn cls(lua: &Lua, _color: Option<i32>) -> Result<()>{
	let globals = lua.globals();
	let mut vm = globals.get::<VM>("EMUPICO_VM")?;
	vm.clear_screen();
	globals.set("EMUPICO_VM", vm)?;
	Ok(())
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
	let mut vm = globals.get::<VM>("EMUPICO_VM")?;
	vm.pal(color1, color2, palette);
	globals.set("EMUPICO_VM", vm)?;
	Ok(())
}

pub fn spr(lua: &Lua, (sprite_number, x, y, w, h, flip_x, flip_y):(i32, i32, i32, Option<i32>, Option<i32>, Option<bool>, Option<bool>)) -> Result<()>{
	let globals = lua.globals();
	let mut vm = globals.get::<VM>("EMUPICO_VM")?;
	vm.spr(sprite_number, x, y, w, h, flip_x, flip_y);
	globals.set("EMUPICO_VM", vm)?;
	Ok(())
}