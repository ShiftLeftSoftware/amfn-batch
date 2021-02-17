//! The batch io element definition.
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

/// The batch io element.

pub struct ElemBatchIO {

  /// Type of the IO. 
  io_type: crate::IOType,

  /// Location of the IO. 
  location: String,

  /// Serialize options. 
  options: usize

}

/// The batch io element implementation.

impl ElemBatchIO {

  /// Create a new object.
  /// 
  /// # Arguments
  ///
  /// * `io_type` - IO type of the batch io element.
  /// * `location` - Location of the batch io element.
  /// * `options` - Options of the batch io output element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(io_type: crate::IOType, location: &str, options: usize) -> ElemBatchIO {
    
    return ElemBatchIO {
      io_type: io_type,
      location: String::from(location),
      options: options
    }
  }

  /// Get the type of the IO.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn io_type(self: &Self) -> crate::IOType {

    return self.io_type;
  }

  /// Get the location of the IO.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn location(self: &Self) -> &str {

    return self.location.as_str();
  }

  /// Get the options of the IO.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn options(self: &Self) -> usize {

    return self.options;
  }

}