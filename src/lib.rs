#![allow(clippy::missing_safety_doc)]

use discord_rich_presence::activity::Assets;
use emf_rs::safer_ffi::prelude::char_p;
use emf_rs::{macros::plugin, once_cell::sync::OnceCell};

use emf_rs::once_cell::sync::Lazy;

use log::*;

use std::sync::{Arc, Mutex};

use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};

// Only really needed when messing with memory stuff. We don't need it (right now)
#[plugin(id = "com.sunglasses.exrpc")]
mod plugin {}

// Create the DiscordRPC instance.
static mut EXRPC: Lazy<Arc<Mutex<DiscordIpcClient>>> = Lazy::new(|| {
	Arc::new(Mutex::new(
		DiscordIpcClient::new("1263575994686640140").unwrap(),
	))
});

static FIRST_RUN: OnceCell<bool> = OnceCell::new();
static mut ACTIVITY: String = String::new();
static mut ICON: String = String::new();

// Setup or init function
unsafe fn exrpc_setup() {
	let mut exrpc = EXRPC.lock().unwrap();
	// Handle error to prevent mutex being poisoned
	if let Err(e) = exrpc.connect() {
		return error!("{}", e);
	};

	let mut plugin = plugin::get();
	// Read the strings from the config.toml file (../config.toml OR exanima_dir/mods/exrpc/config.toml)
	ACTIVITY = plugin.read_setting_string("exrpc_activity").unwrap();
	ICON = plugin.read_setting_string("exrpc_icon").unwrap();

	exrpc_update();
}

unsafe fn exrpc_update() {
	std::thread::spawn(|| {
		let mut exrpc = EXRPC.lock().unwrap();
		info!("Updating activity");
		// Handle error to prevent mutex being poisoned
		if let Err(e) = exrpc.set_activity(
			Activity::new()
				.state(&ACTIVITY.clone())
				.assets(Assets::new().large_image(&ICON.clone())),
		) {
			return error!("{}", e);
		};
		info!("Activity has been updated")
	});
}

#[no_mangle]
pub unsafe extern "C" fn enable() {
	FIRST_RUN.get_or_init(|| {
		pretty_env_logger::formatted_builder()
			.filter_level(LevelFilter::Debug)
			.init();
		true
	});

	std::thread::spawn(|| {
		info!("Running");
		exrpc_setup();
	});
}

#[no_mangle]
pub unsafe extern "C" fn disable() {
	std::thread::spawn(|| {
		let mut exrpc = EXRPC.lock().unwrap();
		// Handle error to prevent mutex being poisoned
		if let Err(e) = exrpc.close() {
			return error!("{}", e);
		};
		info!("Disabled");
	});
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_string(name: char_p::Box, value: char_p::Box) {
	if !value.to_string().is_empty() {
		if &name.to_string() == "exrpc_activity" {
			ACTIVITY = value.to_string();
			info!("Activity string set to {}", ACTIVITY);
		}
		if &name.to_string() == "exrpc_icon" {
			ICON = value.to_string();
			info!("Icon string set to {}", ICON);
		}

		exrpc_update();
	}
}
