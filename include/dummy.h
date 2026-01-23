#ifndef EMUPICO_DUMMY_H
#define EMUPICO_DUMMY_H

#include <sol/sol.hpp>

// --no-music flag
void dummy_music(int song, sol::optional<int> fadeLenth, sol::optional<int> channelMask);

// --no-sfx flag
void dummy_sfx(int sfx, sol::optional<int> channel, sol::optional<int> length);

#endif