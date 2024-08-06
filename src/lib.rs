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
static mut DETAILS: String = String::new();
static mut ICON: String = String::new();
static mut STATE: String = String::new();
static mut STATEENABLED: bool = false;

// Setup or init function
unsafe fn exrpc_setup() {
	let mut exrpc = EXRPC.lock().unwrap();
	// Handle error to prevent mutex being poisoned
	if let Err(e) = exrpc.connect() {
		return error!("{}", e);
	};

	let mut plugin = plugin::get();
	// Read the strings from the config.toml file (../config.toml OR exanima_dir/mods/exrpc/config.toml)
	DETAILS = plugin.read_setting_string("exrpc_details").unwrap();
	ICON = plugin.read_setting_string("exrpc_icon").unwrap();
	STATE = plugin.read_setting_string("exrpc_state").unwrap();
	STATEENABLED = plugin.read_setting_bool("exrpc_stateenabled").unwrap();

	exrpc_update();
}

unsafe fn exrpc_update() {
	std::thread::spawn(|| {
		let mut exrpc = EXRPC.lock().unwrap();
		info!("Updating presence");
		if STATEENABLED == true {
			// Handle error to prevent mutex being poisoned
			if let Err(e) = exrpc.set_activity(
				Activity::new()
					.details(&DETAILS.clone()).state(&STATE.clone())
					.assets(Assets::new().large_image(&ICON.clone())),
			) {
				return error!("{}", e);
			};
		}
		else {
			// Handle error to prevent mutex being poisoned
			if let Err(e) = exrpc.set_activity(
				Activity::new()
					.details(&DETAILS.clone())
					.assets(Assets::new().large_image(&ICON.clone())),
			) {
				return error!("{}", e);
			};
		}
		info!("Rich presence has been updated")
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
pub unsafe extern "C" fn setting_changed_bool(name: char_p::Box, value: bool) {
	if &name.to_string() == "exrpc_stateenabled" {
		STATEENABLED = value;
		info!("State Checkbox set to {}", STATEENABLED);
	}

	exrpc_update();
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_string(name: char_p::Box, value: char_p::Box) {
	if !value.to_string().is_empty() {
		if &name.to_string() == "exrpc_details" {
			DETAILS = value.to_string();
			info!("Details string set to {}", DETAILS);
		}
		if &name.to_string() == "exrpc_icon" {
			ICON = value.to_string();
			info!("Icon string set to {}", ICON);
		}

		if &name.to_string() == "exrpc_state" {
			STATE = value.to_string();
			info!("State string set to {}", STATE);
		}

		exrpc_update();
	}
}
