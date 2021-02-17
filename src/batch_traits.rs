/// The batch traits.
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

  fn read(self: &Self, location: &str) ->  Result<String, amfnengine::ErrorType>;
    
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

  fn write(self: &Self, location: &str, text: &str) -> Result<(), amfnengine::ErrorType>;

}