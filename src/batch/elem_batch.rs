//! The batch element definition.
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

use std::vec::Vec;

use super::{ElemBatchAction, ElemBatchIO};

/// The batch structure.

pub struct ElemBatch {

  /// Name of the batch. 
  name: String,

  /// Optional group of the batch. 
  group: String,

  /// User locale of the batch 
  locale: String,

  /// Enabled for execution 
  enabled: bool,

  /// Action of the batch. 
  actions: Vec<ElemBatchAction>,

  /// Inputs of the batch. 
  inputs: Vec<ElemBatchIO>,

  /// Outputs of the batch. 
  outputs: Vec<ElemBatchIO>

}

/// The batch implementation.

impl ElemBatch {

  /// Create a new object.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - Name of the batch.
  /// * `group_param` - Optional group of the batch.
  /// * `locale_param` - User locale of the batch.
  /// * `enabled_param` - Enabled for execution.
  /// * `actions_param` - Actions of the batch.
  /// * `inputs_param` - Inputs of the batch.
  /// * `outputs_param` - Outputs of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(name_param: &str, group_param: &str, locale_param: &str, enabled_param: bool, 
      actions_param: Vec<ElemBatchAction>, inputs_param: Vec<ElemBatchIO>, 
      outputs_param: Vec<ElemBatchIO>) -> ElemBatch {
    
    return ElemBatch {
      name: String::from(name_param),
      group: String::from(group_param),
      locale: String::from(locale_param),
      enabled: enabled_param,
      actions: actions_param,
      inputs: inputs_param,
      outputs: outputs_param
    }
  }

  /// Get the name of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the optional group of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn group(self: &Self) -> &str {

    return self.group.as_str();
  }

  /// Get the user locale of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn locale(self: &Self) -> &str {

    return self.locale.as_str();
  }

  /// Get the enabled for execution.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn enabled(self: &Self) -> bool {

    return self.enabled;
  }

  /// Get the actions of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn actions(self: &Self) -> &Vec<ElemBatchAction> {

    return &self.actions;
  }

  /// Get the inputs of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn inputs(self: &Self) -> &Vec<ElemBatchIO> {

    return &self.inputs;
  }

  /// Get the outputs of the batch.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn outputs(self: &Self) -> &Vec<ElemBatchIO> {

    return &self.outputs;
  }

}