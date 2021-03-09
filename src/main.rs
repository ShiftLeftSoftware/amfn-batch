//! AmFn engine batch processing.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![doc(html_no_source)]
#![allow(dead_code)]

use clap::{App, Arg};
use colored::*;
use std::io::{self, BufRead};

use amfnbatchlib::ProcessBatch;

pub fn main() {
    let matches = App::new("AmFn Batch Process")
        .version("1.0.0")
        .author("ShiftLeft Software")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("pause")
                .short("p")
                .long("pause")
                .help("Pause prior to exit")
                .takes_value(false),
        )
        .get_matches();

    let pause_at_exit = matches.is_present("pause");

    let mut config_names: Vec<&str> = Vec::new();

    match matches.values_of("config") {
        None => {
            ProcessBatch::println(
                String::from("At least one config option must be specified"),
                Color::Red,
            );
            return;
        }
        Some(o) => {
            for config_name in o {
                config_names.push(config_name);
            }
        }
    }

    match ProcessBatch::process_batches(config_names) {
        Err(e) => {
            ProcessBatch::println(format!("Error: {:?}", e), Color::Red);
        }
        Ok(o) => {
            ProcessBatch::println(
                format!(
                    "Total processing: Batches = {}, Successes = {}, Failures = {}",
                    o.batches(),
                    o.successes(),
                    o.failures()
                ),
                Color::White,
            );
        }
    }

    if pause_at_exit {
        ProcessBatch::println(
            String::from("Press the Enter key to continue:"),
            Color::White,
        );
        io::stdin().lock().lines().next();
    }
}
