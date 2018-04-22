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

use dbus::{Connection, ConnectionItem, BusType, Message};
use dbus::arg::{Array, Dict};
use reqwest;
use rori::account::Account;
use rori::interaction::Interaction;
use serde_json::{Value, from_str};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use time;

/**
 * This class is used to load RORI accounts and handle signals from Ring.
 * Should be one unique instance of this and is used to access the RORI server
 */
pub struct Endpoint {
    pub account: Account,

    rori_server: String,
    rori_ring_id: String,
    ring_dbus: &'static str,
    configuration_path: &'static str,
    configuration_iface: &'static str,
}

impl Endpoint {
    /**
     * Init the RORI server, the database and retrieve the RING account linked
     * @param ring_id to retrieve
     * @return a Manager if success, else an error
     */
    pub fn init(ring_id: &str, rori_server: &str, rori_ring_id: &str) -> Result<Endpoint, &'static str> {
        let mut manager = Endpoint {
            account: Account::null(),

            rori_server: String::from(rori_server),
            rori_ring_id: String::from(rori_ring_id),
            ring_dbus: "cx.ring.Ring",
            configuration_path: "/cx/ring/Ring/ConfigurationManager",
            configuration_iface: "cx.ring.Ring.ConfigurationManager",
        };
        manager.account = Endpoint::build_account(ring_id);
        if !manager.account.enabled {
            info!("{} was not enabled. Enable it", ring_id);
            manager.enable_account();
        }
        debug!("Get: {}", manager.account.ring_id);
        if manager.account.ring_id == "" {
            return Err("Cannot build RORI account, please check configuration");
        }
        info!("{}: Account loaded", manager.account.id);
        Ok(manager)
    }

    pub fn login(manager: Arc<Mutex<Endpoint>>, user_logged: &Arc<Mutex<bool>>) {
        // TODO 1. get if ring_id already match to username (=logged)
        // 2. if not, get if username already registered
        let rori_server = manager.lock().unwrap().rori_server.clone();
        let acc_linked = manager.lock().unwrap().account.clone();
        let username_registered = Endpoint::get_ring_id(&rori_server, &acc_linked.alias) != "";
        if username_registered {
            // 3. TODO if already registered, /link
            manager.lock().unwrap().send_text_interaction_to_rori(&*format!("/link {}", acc_linked.alias));
        } else {
            // 4. else /register
            manager.lock().unwrap().send_text_interaction_to_rori(&*format!("/register {}", acc_linked.alias));
        }
    }

    /**
     * Listen from interresting signals from dbus and call handlers
     * @param self
     */
    pub fn handle_signals(manager: Arc<Mutex<Endpoint>>, stop: Arc<AtomicBool>, rori_text: Arc<Mutex<String>>, user_text: Arc<Mutex<String>>, user_logged: Arc<Mutex<bool>>) {
        // Use another dbus connection to listen signals.
        let dbus_listener = Connection::get_private(BusType::Session).unwrap();
        dbus_listener.add_match("interface=cx.ring.Ring.ConfigurationManager,member=incomingAccountMessage").unwrap();
        dbus_listener.add_match("interface=cx.ring.Ring.ConfigurationManager,member=incomingTrustRequest").unwrap();
        dbus_listener.add_match("interface=cx.ring.Ring.ConfigurationManager,member=accountsChanged").unwrap();
        dbus_listener.add_match("interface=cx.ring.Ring.ConfigurationManager,member=registrationStateChanged").unwrap();
        let rori_ring_id = manager.lock().unwrap().rori_ring_id.clone();
        // For each signals, call handlers.
        for i in dbus_listener.iter(100) {
            let mut m = manager.lock().unwrap();
            m.handle_accounts_signals(&i);
            m.handle_registration_changed(&i);
            if let Some((account_id, interaction)) = m.handle_interactions(&i) {
                info!("New interation for {}: {}", account_id, interaction);
                if account_id == m.account.id {
                    if interaction.author_ring_id == rori_ring_id && interaction.body != "" {
                        match from_str(&interaction.body) {
                            Ok(j) => {
                                // Only if rori order
                                let j: Value = j;
                                if j["registered"].to_string() == "true" {
                                    *user_logged.lock().unwrap() = true;
                                    *rori_text.lock().unwrap() = String::new();
                                }
                            },
                            _ => {
                                *rori_text.lock().unwrap() = String::from(interaction.body);
                            }
                        };

                    }
                }
            };
            if let Some((account_id, from)) = m.handle_requests(&i) {
                if account_id == m.account.id {
                    info!("New request from {}", from);
                    // TODO
                }
            };
            let utext = user_text.lock().unwrap().clone();
            if utext != "" {
                *user_text.lock().unwrap() = String::new();
                m.send_text_interaction_to_rori(&*utext);
            }
            if stop.load(Ordering::SeqCst) {
                break;
            }
        }
    }

    pub fn get_ring_id(nameserver: &String, name: &String) -> String {
        // NOTE: this will not work for now if certificate is self signed
        // See: https://github.com/seanmonstar/reqwest/pull/198
        let mut ns = nameserver.clone();
        if ns.find("http") != Some(0) {
            ns = String::from("https://") + &*ns;
        }
        let mut conn = match reqwest::get(&*format!("{}/name/{}", ns, name)) {
            Ok(conn) => conn,
            _ => {
                return String::from("")
            }
        };
        match conn.text() {
            Ok(body) => {
                let v: Value = from_str(&body).unwrap();
                return v["addr"].to_string();
            },
            _ => {
                return String::from("");
            }
        }
    }

    // Helpers

    /**
     * Add a RING account
     * @param main_info path or alias
     * @param password
     * @param from_archive if main_info is a path
     */
    pub fn add_account(main_info: &str, password: &str, from_archive: bool) {
        let mut details: HashMap<&str, &str> = HashMap::new();
        if from_archive {
            details.insert("Account.archivePath", main_info);
        } else {
            details.insert("Account.alias", main_info);
        }
        details.insert("Account.type", "RING");
        details.insert("Account.archivePassword", password);
        let details = Dict::new(details.iter());
        let dbus_msg = Message::new_method_call("cx.ring.Ring", "/cx/ring/Ring/ConfigurationManager",
                                                "cx.ring.Ring.ConfigurationManager",
                                                "addAccount");
        if !dbus_msg.is_ok() {
            error!("addAccount fails. Please verify daemon's API.");
            return;
        }
        let conn = Connection::get_private(BusType::Session);
        if !conn.is_ok() {
            return;
        }
        let dbus = conn.unwrap();
        let response = dbus.send_with_reply_and_block(dbus_msg.unwrap()
                                                                .append1(details), 2000).unwrap();
        // addAccount returns one argument, which is a string.
        let account_added: &str  = match response.get1() {
            Some(account) => account,
            None => ""
        };
        info!("New account: {:?}", account_added);
    }

    /**
     * Get current ring accounts
     * @return current accounts
     */
    pub fn get_account_list() -> Vec<Account> {
        let mut account_list: Vec<Account> = Vec::new();
        let dbus_msg = Message::new_method_call("cx.ring.Ring", "/cx/ring/Ring/ConfigurationManager",
                                                "cx.ring.Ring.ConfigurationManager",
                                                "getAccountList");
        if !dbus_msg.is_ok() {
            error!("getAccountList fails. Please verify daemon's API.");
            return account_list;
        }
        let conn = Connection::get_private(BusType::Session);
        if !conn.is_ok() {
            return account_list;
        }
        let dbus = conn.unwrap();
        let response = dbus.send_with_reply_and_block(dbus_msg.unwrap(), 2000).unwrap();
        // getAccountList returns one argument, which is an array of strings.
        let accounts: Array<&str, _>  = match response.get1() {
            Some(array) => array,
            None => return account_list
        };
        for account in accounts {
            account_list.push(Endpoint::build_account(account));
        }
        account_list
    }

// Private stuff
    /**
     * Build a new account with an id from the daemon
     * @param id the account id to build
     * @return the account retrieven
     */
    fn build_account(id: &str) -> Account {
        let dbus_msg = Message::new_method_call("cx.ring.Ring", "/cx/ring/Ring/ConfigurationManager",
                                                "cx.ring.Ring.ConfigurationManager",
                                                "getAccountDetails");
        if !dbus_msg.is_ok() {
            error!("getAccountDetails fails. Please verify daemon's API.");
            return Account::null();
        }
        let conn = Connection::get_private(BusType::Session);
        if !conn.is_ok() {
            error!("connection not ok.");
            return Account::null();
        }
        let dbus = conn.unwrap();
        let response = dbus.send_with_reply_and_block(
                                           dbus_msg.unwrap().append1(id), 2000
                                       ).ok().expect("Is the ring-daemon launched?");
        let details: Dict<&str, &str, _> = match response.get1() {
            Some(details) => details,
            None => {
                return Account::null();
            }
        };

        let mut account = Account::null();
        account.id = id.to_owned();
        for detail in details {
            match detail {
                (key, value) => {
                    if key == "Account.enable" {
                        account.enabled = value == "true";
                    }
                    if key == "Account.alias" {
                        account.alias = String::from(value);
                    }
                    if key == "Account.username" {
                        account.ring_id = String::from(value);
                    }
                }
            }
        }
        account
    }

    /**
     * Enable a Ring account
     * @param self
     */
    pub fn enable_account(&self) {
        let dbus_msg = Message::new_method_call(self.ring_dbus, self.configuration_path,
                                                self.configuration_iface,
                                                "sendRegister");
        if !dbus_msg.is_ok() {
            error!("sendRegister call fails. Please verify daemon's API.");
            return;
        }
        let conn = Connection::get_private(BusType::Session);
        if !conn.is_ok() {
            return;
        }
        let dbus = conn.unwrap();
        let _ = dbus.send_with_reply_and_block(
            dbus_msg.unwrap().append2(self.account.id.clone(), true), 2000);
    }

    /**
     * Update current RORI account by handling accountsChanged signals from daemon.
     * @param self
     * @param ci
     */
    fn handle_accounts_signals(&mut self, ci: &ConnectionItem) {
        // Check signal
        let msg = if let &ConnectionItem::Signal(ref signal) = ci { signal } else { return };
        if &*msg.interface().unwrap() != "cx.ring.Ring.ConfigurationManager" { return };
        if &*msg.member().unwrap() != "accountsChanged" { return };
        // TODO test if RORI accounts is still exists
    }

    /**
    * Handle new interactions signals
    * @param self
    * @param ci
    * @return (accountId, interaction)
    */
    fn handle_interactions(&self, ci: &ConnectionItem) -> Option<(String, Interaction)> {
        // Check signal
        let msg = if let &ConnectionItem::Signal(ref signal) = ci { signal } else { return None };
        if &*msg.interface().unwrap() != "cx.ring.Ring.ConfigurationManager" { return None };
        if &*msg.member().unwrap() != "incomingAccountMessage" { return None };
        // incomingAccountMessage return three arguments
        let (account_id, author_ring_id, payloads) = msg.get3::<&str, &str, Dict<&str, &str, _>>();
        let author_ring_id = author_ring_id.unwrap().to_string();
        let mut body = String::new();
        for detail in payloads.unwrap() {
            // TODO handle other interactions
            match detail {
                (key, value) => {
                    if key == "text/plain" {
                        body = value.to_string();
                    }
                }
            }
        };
        let interaction = Interaction {
            author_ring_id: author_ring_id,
            body: body,
            time: time::now()
        };
        Some((account_id.unwrap().to_string(), interaction))
    }

    /**
     * Update current RORI account by handling accountsChanged signals from daemon
     * @param self
     * @param ci
     */
    fn handle_registration_changed(&self, ci: &ConnectionItem) {
        // Check signal
        let msg = if let &ConnectionItem::Signal(ref signal) = ci { signal } else { return };
        if &*msg.interface().unwrap() != "cx.ring.Ring.ConfigurationManager" { return };
        if &*msg.member().unwrap() != "registrationStateChanged" { return };
        // let (account_id, registration_state, _, _) = msg.get4::<&str, &str, u64, &str>();
        // TODO the account can be disabled. Inform UI
    }

    /**
     * Handle new pending requests signals
     * @param self
     * @param ci
     * @return (accountId, from)
     */
    fn handle_requests(&self, ci: &ConnectionItem) -> Option<(String, String)> {
        // Check signal
        let msg = if let &ConnectionItem::Signal(ref signal) = ci { signal } else { return None };
        if &*msg.interface().unwrap() != "cx.ring.Ring.ConfigurationManager" { return None };
        if &*msg.member().unwrap() != "incomingTrustRequest" { return None };
        // incomingTrustRequest return three arguments
        let (account_id, from, _, _) = msg.get4::<&str, &str, Dict<&str, &str, _>, u64>();
        Some((account_id.unwrap().to_string(), from.unwrap().to_string()))
    }



    /**
     * Send a new text message to rori
     * @param self
     * @param body text to send
     * @return the interaction id if success. TODO, watch message status (if received)
     */
    fn send_text_interaction_to_rori(&self, body: &str) -> u64 {
        let mut payloads: HashMap<&str, &str> = HashMap::new();
        payloads.insert("text/plain", body);
        let payloads = Dict::new(payloads.iter());

        let dbus_msg = Message::new_method_call(self.ring_dbus, self.configuration_path, self.configuration_iface,
                                                "sendTextMessage");
        if !dbus_msg.is_ok() {
            error!("sendTextMessage fails. Please verify daemon's API.");
            return 0;
        }
        let conn = Connection::get_private(BusType::Session);
        if !conn.is_ok() {
            return 0;
        }
        let dbus = conn.unwrap();
        // TODO
        let response = dbus.send_with_reply_and_block(dbus_msg.unwrap().append3(&*self.account.id,
            self.rori_ring_id.clone(), payloads), 2000).unwrap();
        // sendTextMessage returns one argument, which is a u64.
        let interaction_id: u64  = match response.get1() {
            Some(interaction_id) => interaction_id,
            None => 0
        };
        interaction_id
    }
}
