//! AmFn engine batch processing.
// Copyright (c) 2021 ShiftLeft Software
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![doc(html_no_source)]
#![allow(dead_code)]

use std::io::{self, BufRead};
use clap::{Arg, App};
use colored::*;

use amfnbatchlib::ProcessBatch;

pub fn main() {

  let matches = App::new("AmFn Batch Process")
                        .version("1.0.0")
                        .author("ShiftLeft Software")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                            .takes_value(true)
                            .multiple(true)
                            .number_of_values(1))
                        .arg(Arg::with_name("pause")
                            .short("p")
                            .long("pause")
                            .help("Pause prior to exit")
                            .takes_value(false))
                        .arg(Arg::with_name("updates")
                            .short("u")
                            .long("updates")
                            .help("Show updates")
                            .takes_value(false))
                        .get_matches();

  let pause_at_exit = matches.is_present("pause");
  let show_updates = matches.is_present("updates");

  let mut config_names: Vec<&str> = Vec::new();

  match matches.values_of("config") {
    None => { 
      ProcessBatch::println(String::from("At least one config option must be specified"), Color::Red);            
      return; 
    }
    Some(o) => { 
      for config_name in o {
          config_names.push(config_name);
      }
    }
  }

  match ProcessBatch::process_batches(config_names, show_updates) {
    Err(e) => { 
      ProcessBatch::println(format!("Error: {:?}", e), Color::Red);            
    }
    Ok(o) => { 
      ProcessBatch::println(format!("Total processing: Batches = {}, Successes = {}, Failures = {}", 
        o.batches(), o.successes(), o.failures()), Color::White);            
    }
  }

  if pause_at_exit {
    ProcessBatch::println(String::from("Press the Enter key to continue:"), Color::White);            
    io::stdin().lock().lines().next();
  }
}