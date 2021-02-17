/// The batch enums.
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

/// Batch action type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ActionType {
  /// Balance the cashflow. 
  Balance = 1,
  /// Calculate the selected event's value. 
  CalcValue = 2,
  /// Calculate the selected event's periods. 
  CalcPeriods = 3,
  /// Calculate the overall yield. 
  CalcYield = 4,
  /// Combine two cashflows. 
  Combine = 5,
  /// Merge two cashflows. 
  Merge = 6,
  /// Split a cashflow. 
  Split = 7,
  /// Transform a cashflow. 
  Transform = 8,
  /// Create a cashflow from a template group. 
  TemplateCashflow = 9
}

/// Batch action test type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TestType {
  /// No test. 
  None = 0,
  /// Test the balance. 
  Balance = 1,
  /// Test the yield. 
  Yield = 2
}

/// Batch io type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IOType {
  /// Read or write from a file. 
  File = 1
}