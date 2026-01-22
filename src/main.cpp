#include <cstdio>

#include <iostream>
#include <fstream>
#include <filesystem>

#include <SDL2/SDL.h>

#define SOL_ALL_SAFETIES_ON 1
#include <sol/sol.hpp>

namespace fs = std::filesystem;

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 540;

bool init();

void close();

SDL_Window* gWindow = NULL;

SDL_Renderer* gRenderer = NULL;

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
		
			uint32_t* screenBuffer = new uint32_t[128 * 128];
			SDL_Texture* screen = SDL_CreateTexture(gRenderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, 128, 128);

			fs::path romPath("default.p8");

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

			lua.script(romCode);

			while (!quit) {
			
				while (SDL_PollEvent( &e ) != 0) {
					if (e.type == SDL_QUIT) {
						quit = true;
					}
				}
		
				SDL_SetRenderDrawColor(gRenderer, 0x00, 0x00, 0x00, 0x00);
                SDL_RenderClear(gRenderer);
				
				lua["_draw"]();
				
				int nx = 128;
  				int ny = 128;
				
				for (int x = 0; x < nx; x++) {
				  for (int y = 0; y < ny; y++) {
				    float r = float(x) / float(nx);
				    float g = float(y) / float(ny);
				    float b = 0.2;
				    int ir = int(255.99 * r);
				    int ig = int(255.99 * g);
				    int ib = int(255.99 * b);

					// Alpha, Red, Blue, Green
				    screenBuffer[(y*nx) + x] = 0xFF000000 | (ir<<16) | (ib<<8) | ig;
				  }
				}

				SDL_UpdateTexture(screen , NULL, screenBuffer, nx * sizeof(uint32_t));
				SDL_RenderCopy(gRenderer, screen, NULL, NULL);

                SDL_RenderPresent(gRenderer);
		}
	}

	close();

	return 0;
}
