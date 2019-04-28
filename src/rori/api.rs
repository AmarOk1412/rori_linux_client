/**
 * Copyright (c) 2019 Sébastien Blin <sebastien.blin@enconn.fr>
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

// TODO replace this API by a subprocess (https://github.com/Uberi/speech_recognition/issues/411)
use iron::prelude::*;
use iron::Handler;
use iron::status;
use router::Router;
use std::sync::{Arc, Mutex};

/**
 * Publicly accessible to manipulate RORI from HTTP requests
 */
pub struct API {
    user_text: Arc<Mutex<String>>,
    is_listening: Arc<Mutex<bool>>
}

impl API {
    /**
     * Initializes the API
     * @param user_text
     * @param is_listening
     * @return an API structure
     */
    pub fn new(user_text: Arc<Mutex<String>>, is_listening: Arc<Mutex<bool>>) -> API {
        API {
            user_text,
            is_listening
        }
    }

    /**
     * Launch an API instance
     * @param self
     */
    pub fn start(&mut self) {
        let mut router = Router::new();
        // Init routes
        let say_handler = SayHandler {
            user_text: self.user_text.clone()
        };

        let start_listening_handler = StartListeningHandler {
            is_listening: self.is_listening.clone()
        };

        let stop_listening_handler = StopListeningHandler {
            is_listening: self.is_listening.clone()
        };

        router.post("/say", say_handler, "say");
        router.get("/startListen", start_listening_handler, "start");
        router.get("/stopListen", stop_listening_handler, "stop");
        // Start router
        Iron::new(router).http("localhost:3000").unwrap();
    }
}

/**
 * fill user text for interface
 */
struct SayHandler {
    user_text: Arc<Mutex<String>>
}

impl Handler for SayHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        match json_body {
            Ok(Some(json_body)) => {
                let user_say = json_body["say"].to_string();
                *self.user_text.lock().unwrap() = String::from(&user_say[1..(user_say.len()-1)]);
            },
            _ => return Ok(Response::with((status::NotFound, "Can't get body")))
        }
        info!("POST /say: {}", *self.user_text.lock().unwrap());
       
        Ok(Response::with(status::Ok))
    }
}

/**
 * Show listening status
 */
struct StartListeningHandler {
    is_listening: Arc<Mutex<bool>>
}

impl Handler for StartListeningHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        *self.is_listening.lock().unwrap() = true;       
        Ok(Response::with(status::Ok))
    }
}

/**
 * Stop showing listening status
 */
struct StopListeningHandler {
    is_listening: Arc<Mutex<bool>>
}

impl Handler for StopListeningHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        *self.is_listening.lock().unwrap() = false;       
        Ok(Response::with(status::Ok))
    }
}