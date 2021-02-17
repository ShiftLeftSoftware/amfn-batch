//! Batch adapter factory definition.
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

use super::AdapterFile;
  
/// Batch adapter factory.

pub struct AdapterFactory {
}
  
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

  pub fn get_adapter(io_type: crate::IOType) -> Box<dyn crate::IOAdapterTrait> {

    match io_type {
      _ => { return Box::new(AdapterFile::new()); }
    }
  }
}  