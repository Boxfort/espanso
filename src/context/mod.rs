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

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
pub(crate) mod macos;

use std::sync::mpsc::Sender;
use crate::event::Event;
use std::path::PathBuf;
use std::fs::create_dir_all;

pub trait Context {
    fn eventloop(&self);
}

// MAC IMPLEMENTATION
#[cfg(target_os = "macos")]
pub fn new(send_channel: Sender<Event>) -> Box<dyn Context> {
    macos::MacContext::new(send_channel)
}

// LINUX IMPLEMENTATION
#[cfg(target_os = "linux")]
pub fn new(send_channel: Sender<Event>) -> Box<dyn Context> {
    linux::LinuxContext::new(send_channel)
}

// WINDOWS IMPLEMENTATION
#[cfg(target_os = "windows")]
pub fn new(send_channel: Sender<Event>) -> Box<dyn Context> {
    windows::WindowsContext::new(send_channel)
}

// espanso directories

pub fn get_data_dir() -> PathBuf {
    let data_dir = dirs::data_local_dir().expect("Can't obtain data_local_dir(), terminating.");
    let espanso_dir = data_dir.join("espanso");
    create_dir_all(&espanso_dir).expect("Error creating espanso data directory");
    espanso_dir
}

pub fn get_config_dir() -> PathBuf {
    // Portable mode check
    // Get the espanso executable path
    let espanso_exe_path = std::env::current_exe().expect("Could not get espanso executable path");
    let exe_dir = espanso_exe_path.parent();
    if let Some(parent) = exe_dir {
        let config_dir = parent.join(".espanso");
        if config_dir.exists() {
            println!("PORTABLE MODE, using config folder: '{}'", config_dir.to_string_lossy());
            return config_dir;
        }
    }

    // For compatibility purposes, check if the $HOME/.espanso directory is available
    let home_dir = dirs::home_dir().expect("Can't obtain the user home directory, terminating.");
    let legacy_espanso_dir = home_dir.join(".espanso");
    if legacy_espanso_dir.exists() {
        eprintln!("WARNING: using legacy espanso config location in $HOME/.espanso is DEPRECATED");
        eprintln!("Starting from espanso v0.3.0, espanso config location is changed.");
        eprintln!("Please check out the documentation to find out more: https://espanso.org/docs/configuration/");

        return legacy_espanso_dir;
    }

    // New config location, from version v0.3.0
    // Refer to issue #73 for more information: https://github.com/federico-terzi/espanso/issues/73
    let config_dir = dirs::config_dir().expect("Can't obtain config_dir(), terminating.");
    let espanso_dir = config_dir.join("espanso");
    create_dir_all(&espanso_dir).expect("Error creating espanso config directory");
    espanso_dir
}

const PACKAGES_FOLDER_NAME : &str = "packages";

pub fn get_package_dir() -> PathBuf {
    // Deprecated $HOME/.espanso/packages directory compatibility check
    let config_dir = get_config_dir();
    let legacy_package_dir = config_dir.join(PACKAGES_FOLDER_NAME);
    if legacy_package_dir.exists() {
        return legacy_package_dir;
    }

    // New package location, starting from version v0.3.0
    let data_dir = get_data_dir();
    let package_dir = data_dir.join(PACKAGES_FOLDER_NAME);
    create_dir_all(&package_dir).expect("Error creating espanso packages directory");
    package_dir
}