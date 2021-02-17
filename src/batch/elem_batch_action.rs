//! The batch action element definition.
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

use rust_decimal::prelude::*;

/// The batch action element.

pub struct ElemBatchAction {

  /// Action type. 
  action: crate::ActionType,

  /// Select template group
  template_group: String,

  /// Select cashflow name
  cashflow_name: String,

  /// Select cashflow name2
  cashflow_name2: String,

  /// New cashflow name
  cashflow_new: String,

  /// Cashflow options
  cashflow_options: String,

  /// Select event name
  event_name: String,

  /// Event select type. 
  select: amfnengine::ExtensionType,

  /// Iteration of the select by type. 
  iteration: usize,

  /// Test type. 
  test_type: crate::TestType,

  /// Test value. 
  test_value: Decimal,

  /// Test string. 
  test_str: String

}

/// The batch action element implementation.

impl ElemBatchAction {

  /// Create a new object.
  /// 
  /// # Arguments
  ///
  /// * `action` - Action of the batch action element.
  /// * `template_group` - Selection of the batch action template group element.
  /// * `cashflow_name` - Selection of the batch action cashflow element.
  /// * `cashflow_name2` - Selection of the batch action cashflow element 2.
  /// * `cashflow_new` - New cashflow element name.
  /// * `cashflow_options` - Batch action cashflow element options.
  /// * `event_name` - Selection of the batch action event element by name.
  /// * `select` - Selection of the batch action event element by type.
  /// * `iteration` - Iteration of the batch action event element by type.
  /// * `test_type` - Test type.
  /// * `test_value` - Test value.
  /// * `test_str` - Test string.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(action: crate::ActionType, template_group: &str, cashflow_name: &str, cashflow_name2: &str, 
      cashflow_new: &str, cashflow_options: &str, event_name: &str,
      select: amfnengine::ExtensionType, iteration: usize,
      test_type: crate::TestType, test_value: Decimal, test_str: &str) -> ElemBatchAction {
    
    return ElemBatchAction {
      action: action,
      template_group: String::from(template_group),
      cashflow_name: String::from(cashflow_name),
      cashflow_name2: String::from(cashflow_name2),
      cashflow_new: String::from(cashflow_new),
      cashflow_options: String::from(cashflow_options),
      event_name: String::from(event_name),
      select: select,
      iteration: iteration,
      test_type: test_type,
      test_value: test_value,
      test_str: String::from(test_str)
    }
  }

  /// Get the action type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn action(self: &Self) -> crate::ActionType {

    return self.action;
  }

  /// Get the name of the batch action template group element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn template_group(self: &Self) -> &str {

    return self.template_group.as_str();
  }

  /// Get the name of the batch action cashflow element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cashflow_name(self: &Self) -> &str {

    return self.cashflow_name.as_str();
  }

  /// Get the name of the batch action cashflow element 2.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cashflow_name2(self: &Self) -> &str {

    return self.cashflow_name2.as_str();
  }

  /// Get the new cashflow element name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cashflow_new(self: &Self) -> &str {

    return self.cashflow_new.as_str();
  }

  /// Get the batch action cashflow element options.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cashflow_options(self: &Self) -> &str {

    return self.cashflow_options.as_str();
  }

  /// Get the name of the batch action event element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_name(self: &Self) -> &str {

    return self.event_name.as_str();
  }

  /// Get the event select type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn select(self: &Self) -> amfnengine::ExtensionType {

    return self.select;
  }

  /// Get the iteration of the event select by type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn iteration(self: &Self) -> usize {

    return self.iteration;
  }

  /// Get the test type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn test_type(self: &Self) -> crate::TestType {

    return self.test_type;
  }

  /// Get the test value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn test_value(self: &Self) -> Decimal {

    return self.test_value;
  }

  /// Get the test string.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn test_str(self: &Self) -> &str {

    return self.test_str.as_str();
  }

}