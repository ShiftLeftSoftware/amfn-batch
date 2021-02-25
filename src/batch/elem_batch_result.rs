//! The batch result element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The batch result element.

pub struct ElemBatchResult {
    /// Number of batches processed.
    batches: usize,

    /// Number of processed successes.
    successes: usize,

    /// Number of processed failures.
    failures: usize,
}

/// The batch result element implementation.

impl ElemBatchResult {
    /// Create a new object.
    ///
    /// # Arguments
    ///
    /// * `batches_param` - Number of batches processed.
    /// * `successes_param` - Number of processed successes.
    /// * `failures_param` - Number of processed failures.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        batches_param: usize,
        successes_param: usize,
        failures_param: usize,
    ) -> ElemBatchResult {
        ElemBatchResult {
            batches: batches_param,
            successes: successes_param,
            failures: failures_param,
        }
    }

    /// Return the number of batches processed.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn batches(&self) -> usize {
        self.batches
    }

    /// Return the number of processed successes.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn successes(&self) -> usize {
        self.successes
    }

    /// Return the number of processed failures.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn failures(&self) -> usize {
        self.failures
    }
}
