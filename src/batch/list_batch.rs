//! List of batches element.
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
use std::cell::Cell;

use amfnengine::ListTrait;
use super::{ElemBatchIO, ElemBatchAction, ElemBatch};

/// List of batches list trait.

pub struct ListBatch {

  /// The list of batches. 
  list_batch: Vec<ElemBatch>,

  /// The index of the currently selected batch element. 
  list_index: Cell<usize>,

  /// Number of simultaneous threads to execute. 
  threads: usize

}

/// List of batches list trait implementation.

impl ListTrait for ListBatch {

  /// Clear all batches from the batch list.

  fn clear(self: &mut Self) -> () {
    
    self.list_batch.clear();
    self.list_index.set(usize::MAX);
  }

  /// Get the count of the batch list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_batch.len();
  }

  /// Get the index of the selected batch (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a batch based upon an index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - Index of the batch to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_batch.len() {
      return false;
    }

    self.set_index(index_param);

    return true;
  }

  /// Set the list index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn set_index(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_batch.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// List of batches implementation.

impl ListBatch {

  /// Create and return a new list of batches.
  /// 
  /// # Arguments
  ///
  /// * `threads_param` - Number of simultaneous threads to execute.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new(threads_param: usize) -> ListBatch {
    
    return ListBatch {
      list_batch: Vec::new(),
      list_index: Cell::new(usize::MAX),
      threads: threads_param
    }
  }

  /// Add a new batch into the batches list.
  /// Duplicate batches are allowed.
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
  /// * True if successful, otherwise false.
  
  pub fn add_batch(self: &mut Self, name_param: &str, group_param: &str, locale_param: &str, 
      enabled_param: bool, actions_param: Vec<ElemBatchAction>, 
      inputs_param: Vec<ElemBatchIO>, outputs_param: Vec<ElemBatchIO>) -> bool {

    let new_elem_batch = ElemBatch::new(
      name_param, group_param, locale_param, enabled_param, actions_param, inputs_param, outputs_param);

    self.list_batch.push(new_elem_batch);

    match self.list_batch.iter().position(|e| e.name() == name_param) {
      None => { return false; }
      Some(o) => { self.list_index.set(o); }
    }

    return true;
  }

  /// Get the name of the batch.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn name(self: &Self) -> &str {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.name(); }
    }
  }

  /// Get the optional group of the batch.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn group(self: &Self) -> &str {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.group(); }
    }
  }

  /// Get the locale of the batch.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn locale(self: &Self) -> &str {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.locale(); }
    }
  }

  /// Get the enabled execution batch.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn enabled(self: &Self) -> bool {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.enabled(); }
    }
  }

  /// Get the Number of simultaneous threads to execute.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn threads(self: &Self) -> usize {

    return self.threads;
  }

  /// Get the list of actions.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn actions(self: &Self) -> &Vec<ElemBatchAction> {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.actions(); }
    }
  }

  /// Get the inputs of the batch.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn inputs(self: &Self) -> &Vec<ElemBatchIO> {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.inputs(); }
    }
  }

  /// Get the outputs of the batch.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn outputs(self: &Self) -> &Vec<ElemBatchIO> {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { return o.outputs(); }
    }
  }

  /// Copy the actions.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn copy_actions(self: &Self) -> Vec<ElemBatchAction> {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { 
        let mut new_actions: Vec<ElemBatchAction> = Vec::new();
        for elem in o.actions() {
          new_actions.push(ElemBatchAction::new(elem.action(), 
            elem.template_group(), elem.cashflow_name(),  elem.cashflow_name2(),  elem.cashflow_new(),  
            elem.cashflow_options(), elem.event_name(), elem.select(), elem.iteration(),
            elem.test_type(), elem.test_value(), elem.test_str()));
        }
        return new_actions; 
      }
    }
  }

  /// Copy the inputs.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn copy_inputs(self: &Self) -> Vec<ElemBatchIO> {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { 
        let mut new_inputs: Vec<ElemBatchIO> = Vec::new();
        for elem in o.inputs() {
          new_inputs.push(ElemBatchIO::new(elem.io_type(), elem.location(), elem.options()));
        }
        return new_inputs; 
      }
    }
  }

  /// Copy the outputs.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn copy_outputs(self: &Self) -> Vec<ElemBatchIO> {

    match self.list_batch.iter().nth(self.list_index.get()) {
      None => { panic!("Batch list index not set"); }
      Some(o) => { 
        let mut new_outputs: Vec<ElemBatchIO> = Vec::new();
        for elem in o.outputs() {
          new_outputs.push(ElemBatchIO::new(elem.io_type(), elem.location(), elem.options()));
        }
        return new_outputs; 
      }
    }
  }

}