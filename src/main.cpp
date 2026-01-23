#include <cstdio>

#include <iostream>
#include <fstream>
#include <filesystem>

#include <SDL2/SDL.h>

#include <CLI11/CLI11.hpp>

#define SOL_ALL_SAFETIES_ON 1
#include <dummy.h>

namespace fs = std::filesystem;

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 540;

bool init();

void close();

SDL_Window* gWindow = NULL;

SDL_Renderer* gRenderer = NULL;

struct screenBox {
	int width;
	int height;

	uint32_t* screenBuffer;

	screenBox() {
		width = 128;
		height = 128;
		screenBuffer = new uint32_t[width * height];
	}

	void cls(sol::optional<int> color) {
		if (color.has_value()) {
			printf("UNIMPLEMENTED: cls(%s)", color);
			std::abort();
		} else {
			for (int x = 0; x < width; x++) {
				  for (int y = 0; y < height; y++) {
					// Alpha, Red, Blue, Green
				    screenBuffer[(y*width) + x] = 0xFF000000 | (0<<16) | (0<<8) | 0	;
				  }
			}
		}
	}
};

bool init() {
	bool success = true;

	if (SDL_Init(SDL_INIT_VIDEO) < 0) {
		printf("SDL could not initialize! SDL_Error: %s\n", SDL_GetError());
		success = false;
	} else {
		gWindow = SDL_CreateWindow("emupico", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, SCREEN_WIDTH, SCREEN_HEIGHT, SDL_WINDOW_SHOWN);
		if (gWindow == NULL) {
			printf("Window could not be created! SDL_Error: %s\n", SDL_GetError());
			success = false;
		} else {
            gRenderer = SDL_CreateRenderer( gWindow, -1, SDL_RENDERER_ACCELERATED );
            if(gRenderer == NULL) {
                printf("Renderer could not be created! SDL Error: %s\n", SDL_GetError());
                success = false;
            } else {
                SDL_SetRenderDrawColor( gRenderer, 0x00, 0x00, 0x00, 0x00 );
            }
		}
	}

	return success;
}

void close() {
	SDL_DestroyWindow(gWindow);
	gWindow = NULL;

	SDL_Quit();
}

int main(int argc, char* args[]) {

	if (!init()) {
		printf("Failed to initialize!\n");
	} else {				
			bool quit = false;
		
			SDL_Event e;
		
			CLI::App app{"Another PICO-8 emulator"};
    		args = app.ensure_utf8(args);
		
			std::string path = "default.p8";
			app.add_option("file", path, "The file to be executed.")
				->required();

			bool noMusic = false;
			app.add_flag("-m,--no-music", noMusic, "Disable music, useful for \"music()\" related crashes.");

			bool noSFX = false;
			app.add_flag("-s,--no-sfx", noMusic, "Disable SFX, useful for \"sfx()\" related crashes.");

			CLI11_PARSE(app, argc, args);			

			SDL_Texture* screen = SDL_CreateTexture(gRenderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, 128, 128);

			fs::path romPath(path);

			if (!fs::exists(romPath)) {
				printf("ROM file %s is missing.\n", fs::absolute(romPath).string().c_str());
				return 1;
			}

			std::string romData;
			std::ifstream romFile(romPath, std::ios::binary);
			std::stringstream romBuffer;

			romBuffer << romFile.rdbuf();
			romFile.close();
			
			std::string romLine;
			std::string romCode;
			std::string romGFX;
			std::string romSFX;
			std::string romMusic;
			std::string romLabel;

			bool isCode = false;
			bool isGFX = false;
			bool isSFX = false;
			bool isMusic = false;
			bool isLabel = false;

			while (std::getline(romBuffer, romLine)) {
				// Trim trailing whitespace (carriage returns, etc.)
				while (!romLine.empty() && std::isspace(romLine.back())) {
					romLine.pop_back();
				}
				
				// Skip empty lines
				if (romLine.empty()) {
					continue;
				}
				
				// Section check
        		if (romLine == "__lua__") {
					isCode = true;
					isGFX = false;
					isSFX = false;
					isMusic = false;
					continue;
				}
				if (romLine == "__gfx__") {
					isCode = false;
					isGFX = true;
					isSFX = false;
					isMusic = false;
					continue;
				}
				if (romLine == "__label__") {
					isCode = false;
					isGFX = false;
					isLabel = true;
					isSFX = false;
					isMusic = false;
					continue;
				}
				if (romLine == "__sfx__") {
					isCode = false;
					isGFX = false;
					isLabel = false;
					isSFX = true;
					isMusic = false;
					continue;
				}
				if (romLine == "__music__") {
					isCode = false;
					isGFX = false;
					isLabel = false;
					isSFX = false;
					isMusic = true;
					continue;
				}

				// Place ROM data in apropriate places
				if (isCode) {
					romLine += "\n";
					romCode.append(romLine);
				}
				if (isGFX) {
					romLine += "\n";
					romGFX.append(romLine);
				}
				if (isLabel) {
					romLine += "\n";
					romLabel.append(romLine);
				}
				if (isSFX) {
					romLine += "\n";
					romSFX.append(romLine);
				}
				if (isMusic) {
					romLine += "\n";
					romMusic.append(romLine);
				}
			}
			
			sol::state lua;
			lua.open_libraries(sol::lib::base);

			if (noMusic) {
				lua.set_function("music", dummy_music);
			}
			if (noSFX) {
				lua.set_function("sfx", dummy_sfx);
			}

			lua.set_function("cls", &screenBox::cls, screenBox());

			lua.script(romCode);

			auto init = lua["_init"];
			auto update = lua["_update"];
			auto draw = lua["_draw"];

			bool hasUpdate = false;
			bool hasDraw = false;

			if (init.valid()) {
				init();
			}

			if (update.valid()) {
				hasUpdate = true;
			}

			if (draw.valid()) {
				hasDraw = true;
			}

			screenBox screenBoxInst;

			for (int x = 0; x < screenBoxInst.width; x++) {
				  for (int y = 0; y < screenBoxInst.height; y++) {
					// Alpha, Red, Blue, Green
				    screenBoxInst.screenBuffer[(y*screenBoxInst.width) + x] = 0xFF000000 | (0<<16) | (0<<8) | 0	;
				  }
			}

			while (!quit) {
			
				while (SDL_PollEvent( &e ) != 0) {
					if (e.type == SDL_QUIT) {
						quit = true;
					}
				}
		
				SDL_SetRenderDrawColor(gRenderer, 0x00, 0x00, 0x00, 0x00);
				
				if (hasUpdate) {
					update();
				}

				if (hasDraw) {
					draw();
				}

				SDL_UpdateTexture(screen , NULL, screenBoxInst.screenBuffer, screenBoxInst.width * sizeof(uint32_t));
				SDL_RenderCopy(gRenderer, screen, NULL, NULL);

                SDL_RenderPresent(gRenderer);
		}
	}

	close();

	return 0;
}
