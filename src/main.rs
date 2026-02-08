use std::fs::File;
use std::time::Duration;
use std::io::prelude::*;
use std::fmt::Write;

use camino::Utf8PathBuf;

use clap::Parser;

use emupico::vm::VM;
use emupico::rom::{RomSectionType, RomSection};

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureAccess;


#[derive(Parser)]
#[command(name = "emupico")]
struct Args {
	file: Utf8PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let args = Args::parse(); 

	let mut rom_file = File::open(&args.file)?;

	let mut rom_data = String::new();
	rom_file.read_to_string(&mut rom_data)?;

	let mut current_section = RomSectionType::Header;

	let mut lua_section = RomSection::new(RomSectionType::Lua);
	let mut gfx_section = RomSection::new(RomSectionType::Gfx);
	let mut label_section = RomSection::new(RomSectionType::Label);
	let mut sfx_section = RomSection::new(RomSectionType::Sfx);
	let mut music_section = RomSection::new(RomSectionType::Music);

	for line in rom_data.lines() {
		match line {
			"__lua__" => current_section = RomSectionType::Lua,
			"__gfx__" => current_section = RomSectionType::Gfx,
			"__label__" => current_section = RomSectionType::Label,
			"__sfx__" => current_section = RomSectionType::Sfx,
			"__music__" => current_section = RomSectionType::Music,

			_ => {
				match current_section {
					RomSectionType::Header => {},
					RomSectionType::Lua => writeln!(lua_section.data, "{}", line)?,
					RomSectionType::Gfx => writeln!(gfx_section.data, "{}", line)?,
					RomSectionType::Label => writeln!(label_section.data, "{}", line)?,
					RomSectionType::Sfx => writeln!(sfx_section.data, "{}", line)?,
					RomSectionType::Music => writeln!(music_section.data, "{}", line)?
				}
			}
		}
	}

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("emupico", 512, 512)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	let mut vm = VM::new();

	let creator = canvas.texture_creator();

	let mut screen_texture =  match creator.create_texture(
		PixelFormatEnum::RGBA8888, TextureAccess::Streaming, 128, 128) {
			Ok(texture) => texture,
			Err(why) => panic!("failed to create screen texture: {}", why)
	};

	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	'running: loop{
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running Ok(())
				},
				_ => {}
			}
		}
		
		vm.draw_pixel(0, 0, 0x45b6f7ff);

		screen_texture.update(None, vm.screen_raw(), 128*4 as usize)?;

		canvas.copy(&screen_texture, None, None)?;

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
