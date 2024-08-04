#![allow(clippy::missing_safety_doc)]

use log::*;

use emf_rs::macros::plugin;
use emf_rs::safer_ffi::prelude::char_p;

use emf_rs::once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use discord_rpc_client::Client as DiscordRPC;

#[plugin(id = "com.sunglasses.exrpc")]
mod plugin {

}

static mut EXRPC: Lazy<Arc<Mutex<DiscordRPC>>> = Lazy::new(|| Arc::new(Mutex::new(DiscordRPC::new(1263575994686640140))));

unsafe fn exrpc_setup() {
	let mut plugin = plugin::get();
	let activity = plugin.read_setting_string("exrpc_activity");

	dbg!(activity);

    std::thread::spawn(|| {
        let mut exrpc = EXRPC.lock().unwrap();
        exrpc.start();
        exrpc
            .set_activity(|act| {
                act.state("activity")
                    .assets(|ass| ass.large_image("exanima"))
            })
            .expect("failed to set activity");
    });
}

unsafe fn exrpc_update(activity: &str) {
	let activity = activity.to_string();
    std::thread::spawn(|| {
        let mut exrpc = EXRPC.lock().unwrap();

		exrpc
			.set_activity(|act| {
				act.state(activity).details(":3")
					.assets(|ass| ass.large_image("exanima"))
			})
			.expect("failed to update activity");
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
pub unsafe extern "C" fn on_message(s: char_p::Box, m: char_p::Box) {
	plugin::get().on_message(s, m, |sender, message| {
		debug!("Received message from {}: {}", sender, message);
	});
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_bool(name: char_p::Box, value: bool) {
	// If the ID of a boolean setting starts with `patch::` or `hook::`,
	// the plugin will automatically enable/disable the corresponding patch/hook.

	// Example:
	//
	// [[setting]]
	// name = "My Setting"
	// id = "patch::my_patch"
	//
	// or
	//
	// [[setting]]
	// name = "My Setting"
	// id = "hook::my_hook"

	plugin::get().on_setting_changed_bool(name, value, |key, value| {
		// Do something with this
		debug!("Setting changed: {} = {}", key, value);
	});
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_int(name: char_p::Box, value: i32) {
	plugin::get().on_setting_changed_int(name, value, |key, value| {
		// Do something with this
		debug!("Setting changed: {} = {}", key, value);
	});
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_float(name: char_p::Box, value: f32) {
	plugin::get().on_setting_changed_float(name, value, |key, value| {
		// Do something with this
		debug!("Setting changed: {} = {}", key, value);
	});
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_string(name: char_p::Box, value: char_p::Box) {
	let name = name.to_string();
	let value = value.to_string();

	if &name == "exrpc_activity" {
		exrpc_update(&value);
	}
}
