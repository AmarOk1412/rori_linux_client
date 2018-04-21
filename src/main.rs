/**
 * Copyright (c) 2018, Sébastien Blin <sebastien.blin@enconn.fr>
 * All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright
 *  notice, this list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright
 *  notice, this list of conditions and the following disclaimer in the
 *  documentation and/or other materials provided with the distribution.
 * * Neither the name of the University of California, Berkeley nor the
 *  names of its contributors may be used to endorse or promote products
 *  derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE REGENTS AND CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 **/

extern crate dbus;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate qmlrs;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;

pub mod rori;
pub mod sharedprop;

use rori::account::Account;
use rori::endpoint::Endpoint;
use serde_json::{Value, from_str};
use sharedprop::SharedProp;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;

/**
 * ConfigFile structure
 * TBD
 */
#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    ring_id: String,
    rori_server: String,
    username: String,
}

/**
 * Generate a config file
 * NOTE: will move into the UI ASAP
 */
fn create_config_file() {
    let accounts = Endpoint::get_account_list();
    let mut chosen_acc = Account::null();
    for account in accounts {
        if account.alias == "rori_linux_client" {
            chosen_acc = account;
        }
    }
    if chosen_acc.id == "" {
        // Create ring account
        Endpoint::add_account("rori_linux_client", "", false);
        // Let some time for the daemon
        let three_secs = Duration::from_millis(3000);
        thread::sleep(three_secs);
        let accounts = Endpoint::get_account_list();
        for account in accounts {
            if account.alias == "rori_linux_client" {
                chosen_acc = account;
            }
        }
    }

    if chosen_acc.id == "" {
        return;
    }

    let config = ConfigFile {
        ring_id: chosen_acc.id,
        rori_server: String::from("127.0.0.1:1412"), // TODO
        username: String::new(),
    };
    let config = serde_json::to_string_pretty(&config).unwrap_or(String::new());
    let mut file = File::create("config.json").ok().expect("config.json found.");
    let _ = file.write_all(config.as_bytes());
}

fn main() {
    // Init logging
    env_logger::init();

    // if not config, create it
    if !Path::new("config.json").exists() {
        create_config_file();
    }

    if !Path::new("config.json").exists() {
        error!("No config file found");
        return;
    }

    // This script load config from config.json
    let mut file = File::open("config.json").ok()
        .expect("Config file not found");
    let mut config = String::new();
    file.read_to_string(&mut config).ok()
        .expect("failed to read!");
    let config: Value = from_str(&*config).ok()
                        .expect("Incorrect config file. Please check config.json");

    let shared_prop = SharedProp {
        rori_text: Arc::new(Mutex::new(String::new())),
        user_text: Arc::new(Mutex::new(String::new()))
    };
    let rori_text = shared_prop.rori_text.clone();
    let user_text = shared_prop.user_text.clone();
    // Useless arc?
    let shared_endpoint : Arc<Mutex<Endpoint>> = Arc::new(Mutex::new(
        Endpoint::init(config["ring_id"].as_str().unwrap_or(""))
        .ok().expect("Can't initialize ConfigurationEndpoint"))
    );
    let shared_endpoint_cloned = shared_endpoint.clone();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_cloned = stop.clone();
    let handle_signals = thread::spawn(move || {
        Endpoint::handle_signals(shared_endpoint_cloned, stop_cloned, rori_text, user_text);
    });
    let mut engine = qmlrs::Engine::new();
    engine.load_local_file("ui/rori.qml");
    engine.set_property("sharedprop", shared_prop);
    engine.exec();
    stop.store(false, Ordering::SeqCst);
    let _ = handle_signals.join();
}
