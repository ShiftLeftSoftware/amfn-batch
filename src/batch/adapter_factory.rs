//! Batch adapter factory definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::AdapterFile;
/// Batch adapter factory.

pub struct AdapterFactory {}
/// Batch adapter factory implementation.

impl AdapterFactory {
    /// Create and return an adapter.
    ///
    /// # Arguments
    ///
    /// * `io_type` - Adapter type.
    ///
    /// # Return
    ///
    /// * See description.
    #[allow(clippy::match_single_binding)]

    pub fn get_adapter(io_type: crate::IOType) -> Box<dyn crate::IOAdapterTrait> {
        match io_type {
            _ => Box::new(AdapterFile::new()),
        }
    }
}
