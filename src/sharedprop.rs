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

use std::sync::{Arc, Mutex};
use qmlrs;


 pub struct SharedProp {
     pub rori_text: Arc<Mutex<String>>,
     pub user_text: Arc<Mutex<String>>,
     pub api_text: Arc<Mutex<String>>,
     pub is_listening: Arc<Mutex<bool>>,
     pub logged: Arc<Mutex<bool>>
 }

 impl SharedProp {
     fn set_api_text(&self, text: String) {
         *self.api_text.lock().unwrap() = text;
     }

     fn set_user_text(&self, text: String) {
         *self.user_text.lock().unwrap() = text;
     }

     fn get_api_text(&self) -> String {
         self.api_text.lock().unwrap().clone()
     }

     fn get_rori_text(&self) -> String {
         self.rori_text.lock().unwrap().clone()
     }

     fn get_logged(&self) -> bool {
         self.logged.lock().unwrap().clone()
     }

     fn get_is_listening(&self) -> bool {
         self.is_listening.lock().unwrap().clone()
     }
}

 Q_OBJECT! { SharedProp:
     slot fn set_api_text(String);
     slot fn set_user_text(String);
     slot fn get_api_text();
     slot fn get_rori_text();
     slot fn get_logged();
     slot fn get_is_listening();
 }
