#![allow(clippy::missing_safety_doc)]

use emf_rs::macros::plugin;
use emf_rs::safer_ffi::prelude::char_p;

use emf_rs::once_cell::sync::Lazy;

use log::error;

use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

use discord_rpc_client::Client as DiscordRPC;

// Only really needed when messing with memory stuff. We don't need it (right now)
#[plugin(id = "com.sunglasses.exrpc")]
mod plugin {}

// Create the DiscordRPC instance.
static mut EXRPC: Lazy<Arc<Mutex<DiscordRPC>>> =
	Lazy::new(|| Arc::new(Mutex::new(DiscordRPC::new(1263575994686640140))));

static mut ACTIVITY: String = String::new();
static mut ICON: String = String::new();

// Setup or init function
unsafe fn exrpc_setup() {
	std::thread::spawn(|| {
		println!("EXRPC: Reading config.toml for last used value for activity.");
		println!("EXRPC: Reading config.toml for last used value for icon.");

		let mut plugin = plugin::get();
		// Read the strings from the config.toml file (../config.toml OR exanima_dir/mods/exrpc/config.toml))
		ACTIVITY = plugin.read_setting_string("exrpc_activity").unwrap();
		ICON = plugin.read_setting_string("exrpc_icon").unwrap();

		println!("EXRPC: Set activity to {}", ACTIVITY);
		println!("EXRPC: Set icon to {}", ICON);

		let mut exrpc = EXRPC.lock().unwrap();

		exrpc.on_error(|_ctx| {
			error!("EXRPC: Error ! {:?}", _ctx.borrow());
		});

		// Start the DiscordRPC
		exrpc.start();

		println!("EXRPC: Started");
	
		// Start the update loop. 
		exrpc_update();
		println!("EXRPC: Update loop started");
	});
}

unsafe fn exrpc_update() {
	std::thread::spawn(|| {		
		loop {
			let mut exrpc = EXRPC.lock().unwrap();
			exrpc
				.set_activity(|act| {
					act.state(ACTIVITY.to_string())
						.assets(|ass| ass.large_image(ICON.to_string()))
				})
				.expect("failed to update activity");
		}
	});
}

#[no_mangle]
pub unsafe extern "C" fn enable() {
	exrpc_setup();
}

#[no_mangle]
pub unsafe extern "C" fn disable() {
	
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_string(name: char_p::Box, value: char_p::Box) {
	if value.to_string().is_empty() == false {
		if &name.to_string() == "exrpc_activity" {
			ACTIVITY = value.to_string();
			println!("EXRPC: Activity string set to {}", ACTIVITY);
		}
		if &name.to_string() == "exrpc_icon" {
			ICON = value.to_string();
			println!("EXRPC: Icon string set to {}", ICON);
		}
	}
}
