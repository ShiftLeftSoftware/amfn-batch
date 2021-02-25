/// The batch traits.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Common trait for all I/O adapters.

pub trait IOAdapterTrait {
    /// Read input from a location.
    ///
    /// # Arguments
    ///
    /// * `location` - The location to read.
    ///
    /// # Return
    ///
    /// * The input string or an error code.

    fn read(&self, location: &str) -> Result<String, amfnengine::ErrorType>;

    /// Write output to a location.
    ///
    /// # Arguments
    ///
    /// * `location` - The location to write.
    /// * `text` - The text to write.
    ///
    /// # Return
    ///
    /// * ERROR_NONE or an error code.

    fn write(&self, location: &str, text: &str) -> Result<(), amfnengine::ErrorType>;
}
