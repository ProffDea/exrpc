#![allow(clippy::missing_safety_doc)]

use log::*;

use emf_rs::macros::plugin;
use emf_rs::safer_ffi::prelude::char_p;

use emf_rs::once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use std::{thread, time};

use discord_rpc_client::Client as DiscordRPC;

#[plugin(id = "com.sunglasses.exrpc")]
mod plugin {

}

static mut EXRPC: Lazy<Arc<Mutex<DiscordRPC>>> = Lazy::new(|| Arc::new(Mutex::new(DiscordRPC::new(1263575994686640140))));
static mut activity: String = String::new();
static mut details: String = String::new();
static mut icon: String = String::new();

unsafe fn exrpc_setup() {
    std::thread::spawn(|| {
        let mut exrpc = EXRPC.lock().unwrap();
        exrpc.start();
        exrpc
            .set_activity(|act| {
                act.state("Running around...")
                    .assets(|ass| ass.large_image("exanima"))
            })
            .expect("failed to set activity");
    });
}

unsafe fn exrpc_update() {
	//let mut exrpc = EXRPC.lock().unwrap();
    std::thread::spawn(|| {
		let mut exrpc = EXRPC.lock().unwrap();
		dbg!(activity.to_string());
		dbg!(details.to_string());
		dbg!(icon.to_string());
		exrpc
			.set_activity(|act| {
				act.state(activity.to_string()).details(details.to_string())
					.assets(|ass| ass.large_image(icon.to_string()))
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
	
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_int(name: char_p::Box, value: i32) {
	
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_float(name: char_p::Box, value: f32) {
	
}

#[no_mangle]
pub unsafe extern "C" fn setting_changed_string(name: char_p::Box, value: char_p::Box) {
	if &name.to_string() == "exrpc_activity" { activity = value.to_string(); dbg!(activity.to_string()); }
	if &name.to_string() == "exrpc_details" { details = value.to_string(); dbg!(details.to_string()); }
	if &name.to_string() == "exrpc_icon" { icon = value.to_string(); dbg!(icon.to_string()); }

	exrpc_update();
}
