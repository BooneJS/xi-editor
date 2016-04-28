/// Copyright 2016 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate serde_json;
extern crate time;

use std::io;
use std::io::{Read, Write};
use serde_json::Value;

#[macro_use]
mod macros;

mod editor;
mod view;
mod linewrap;

use editor::Editor;

extern crate xi_rope;
extern crate xi_unicode;

// TODO: should provide result
pub fn send(v: &Value) {
    let mut s = serde_json::to_string(v).unwrap();
    s.push('\n');
    //print_err!("from core: {}", s);
    let size = s.len();
    let mut sizebuf = [0; 8];
    for (i, item) in sizebuf.iter_mut().enumerate() {
        *item = (((size as u64) >> (i * 8)) & 0xff) as u8;
    }
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let _ = stdout_handle.write_all(&sizebuf);
    let _ = stdout_handle.write_all(s.as_bytes());
    // flush is not needed because of the LineWriter on stdout
    //let _ = stdout_handle.flush();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    let mut sizebuf = [0; 8];
    let mut editor = Editor::new();
    while stdin_handle.read_exact(&mut sizebuf).is_ok() {
        // byteorder would be more direct
        let size = sizebuf.iter().enumerate().fold(0, |s, (i, &b)| s + ((b as u64) << (i * 8)));
        let mut buf = vec![0; size as usize];
        if stdin_handle.read_exact(&mut buf).is_ok() {
            if let Ok(data) = serde_json::from_slice::<Value>(&buf) {
                print_err!("to core: {:?}", data);
                if let Some(array) = data.as_array() {
                    if let Some(cmd) = array[0].as_string() {
                        editor.do_cmd(cmd, &array[1]);
                    }
                }
            }
        }
    }
}
