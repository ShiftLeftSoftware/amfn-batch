#![doc(html_no_source)]
#![allow(dead_code)]

//! AmFn engine batch processing variable integration tests.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use amfnbatchlib::ProcessBatch;

pub const TEST_FILE: &str = "testsets/test_batch_variable.json";

#[test]
fn batch_variable() {
    let mut config_names: Vec<&str> = Vec::new();
    config_names.push(TEST_FILE);

    match ProcessBatch::process_batches(config_names) {
        Err(e) => {
            assert!(false, "{:?}", e);
        }
        Ok(o) => {
            assert!(
                o.failures() == 0,
                "Total processing: Batches = {}, Successes = {}, Failures = {}",
                o.batches(),
                o.successes(),
                o.failures()
            );
        }
    }
}
