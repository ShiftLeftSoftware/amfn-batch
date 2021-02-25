//! The batch io element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The batch io element.

pub struct ElemBatchIO {
    /// Type of the IO.
    io_type: crate::IOType,

    /// Location of the IO.
    location: String,

    /// Serialize options.
    options: usize,
}

/// The batch io element implementation.

impl ElemBatchIO {
    /// Create a new object.
    ///
    /// # Arguments
    ///
    /// * `io_type_param` - IO type of the batch io element.
    /// * `location_param` - Location of the batch io element.
    /// * `options_param` - Options of the batch io output element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        io_type_param: crate::IOType,
        location_param: &str,
        options_param: usize,
    ) -> ElemBatchIO {
        ElemBatchIO {
            io_type: io_type_param,
            location: String::from(location_param),
            options: options_param,
        }
    }

    /// Get the type of the IO.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn io_type(&self) -> crate::IOType {
        self.io_type
    }

    /// Get the location of the IO.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn location(&self) -> &str {
        self.location.as_str()
    }

    /// Get the options of the IO.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn options(&self) -> usize {
        self.options
    }
}
