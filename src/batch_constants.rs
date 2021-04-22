/// The batch constants.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Version message. 
pub const APP_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

/// Thread sleep delay in millis.
pub const THREAD_SLEEP_MILLIS: u64 = 200;