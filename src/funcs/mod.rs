pub mod dummy;

use mlua::{Lua, Result};

use crate::vm::VM;

pub fn cls(lua: &Lua, _color: Option<i32>) -> Result<()>{
	lua.load("EMUPICO_VM:cls()").set_name("cls").exec()
}

pub fn time(lua: &Lua, _color: Option<i32>) -> Result<f32>{
	let globals = lua.globals();
	Ok(globals.get::<VM>("EMUPICO_VM")?.time)
}