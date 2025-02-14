/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef ESPANSO_BRIDGE_H
#define ESPANSO_BRIDGE_H

#include <stdint.h>

extern void * context_instance;

/*
 * Check if the X11 context is available
 */
extern "C" int32_t check_x11();

/*
 * Initialize the X11 context and parameters
 */
extern "C" int32_t initialize(void * context_instance);

/*
 * Start the event loop indefinitely. Blocking call.
 */
extern "C" int32_t eventloop();

/*
 * Clean all the X11 resources allocated during the initialization.
 */
extern "C" void cleanup();

/*
 * Called when a new keypress is made, the first argument is an char array,
 * while the second is the size of the array.
 */
typedef void (*KeypressCallback)(void * self, const char *buffer, int32_t len, int32_t is_modifier, int32_t key_code);

extern KeypressCallback keypress_callback;

/*
 * Register the callback that will be called when a keypress was made
 */
extern "C" void register_keypress_callback(KeypressCallback callback);

/*
 * Type the given string by simulating Key Presses
 */
extern "C" void send_string(const char * string);

/*
 * Send the backspace keypress, *count* times.
 */
extern "C" void delete_string(int32_t count);

/*
 * Trigger normal paste ( Pressing CTRL+V )
 */
extern "C" void trigger_paste();

/*
 * Trigger terminal paste ( Pressing CTRL+SHIFT+V )
 */
extern "C" void trigger_terminal_paste();


// SYSTEM MODULE

/*
 * Return the active windows's WM_NAME
 */
extern "C" int32_t get_active_window_name(char * buffer, int32_t size);

/*
 * Return the active windows's WM_CLASS
 */
extern "C" int32_t get_active_window_class(char * buffer, int32_t size);

/*
 * Return the active windows's executable path
 */
extern "C" int32_t get_active_window_executable(char * buffer, int32_t size);

/*
 * Return 1 if the current window is a terminal window, 0 otherwise.
 */
extern "C" int32_t is_current_window_terminal();

#endif //ESPANSO_BRIDGE_H
