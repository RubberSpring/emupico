use std::fs::File;
use std::time::Duration;
use std::io::prelude::*;

use camino::Utf8PathBuf;

use clap::Parser;

use emupico::VM;

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

fn main(){
	let args = Args::parse(); 

	let mut rom_file = match File::open(&args.file) {
		Err(why) => {
			eprintln!("couldn't open {}: {}", args.file, why);
			return;
		},
		Ok(file) => file
	};

	let mut rom_data = String::new();
	match rom_file.read_to_string(&mut rom_data) {
		Err(why) => panic!("failed to read {}: {}", args.file, why),
		Ok(_) => {}
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
	'running: loop {
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}
		
		vm.draw_pixel(0, 0, 0x45b6f7);

		match screen_texture.update(None, vm.screen_raw(), 128*4 as usize) {
			Err(why) => panic!("failed to update screen texture: {}", why),
			Ok(()) => {}
		}

		match canvas.copy(&screen_texture, None, None) {
			Err(why) => panic!("failed to copy screen texture: {}", why),
			Ok(()) => {}
		}

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
