#include <dummy.h>
#include <cstdio>

// dummy.h functions
void dummy_music(int song, sol::optional<int> fadeLenth, sol::optional<int> channelMask) {
    printf("WARNING: music disabled (-m --no-music)\n");
}

void dummy_sfx(int sfx, sol::optional<int> channel, sol::optional<int> length) {
    printf("WARNING: sfx disabled (-s --no-sfx)\n");
}