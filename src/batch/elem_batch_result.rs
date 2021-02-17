//! The batch result element definition.
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

/// The batch result element.

pub struct ElemBatchResult {

  /// Number of batches processed. 
  batches: usize,

  /// Number of processed successes. 
  successes: usize,

  /// Number of processed failures. 
  failures: usize

}

/// The batch result element implementation.

impl ElemBatchResult {

  /// Create a new object.
  /// 
  /// # Arguments
  ///
  /// * `batches` - Number of batches processed.
  /// * `successes` - Number of processed successes.
  /// * `failures` - Number of processed failures.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(batches: usize, successes: usize, failures: usize) -> ElemBatchResult {
    
    return ElemBatchResult {
      batches: batches,
      successes: successes,
      failures: failures
    }
  }

  /// Return the number of batches processed.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn batches(self: &Self) -> usize {

    return self.batches;
  }

  /// Return the number of processed successes.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn successes(self: &Self) -> usize {

    return self.successes;
  }

  /// Return the number of processed failures.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn failures(self: &Self) -> usize {

    return self.failures;
  }

}