//! The batch process batch definition.
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

#![doc(html_no_source)]
#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use rust_decimal::prelude::*;
use colored::*;

use amfnengine::*;
use amfnengine::core::*;
use amfnengine::engine::*;

use batch::{AdapterFactory, BatchUtility, JsonDeserialize, 
  ListBatch, ElemBatchAction, ElemBatchIO, ElemBatchResult};

include!("batch_constants.rs");
include!("batch_enums.rs");
include!("batch_traits.rs");

pub mod batch;

/// The batch process batch structure.

pub struct ProcessBatch {
}

/// The batch process batch implementation.

impl ProcessBatch {

  /// Process batches for the AmFn batch processing.
  /// 
  /// # Arguments
  ///
  /// * `config_names` - Configuration names.
  /// * `show_updates` - Show update messages from the engine.
  /// 
  /// # Return
  ///
  /// * The batch result or an error code.

  pub fn process_batches(config_names: Vec<&str>, show_updates: bool) -> 
    Result<ElemBatchResult, crate::ErrorType> {

    let mut total_batches: usize = 0;
    let mut total_successes: usize = 0;
    let mut total_failures: usize = 0;

    for config_name in config_names {

      BatchUtility::println(format!("Processing configuration: {}", config_name), Color::White);

      let adapter = AdapterFactory::get_adapter(crate::IOType::File);

      let json_config: String;
      match adapter.read(config_name) {
        Err(e) => { return Err(e); }
        Ok(o) => { json_config = o; }
      }
      
      let list_batch: ListBatch;
      match JsonDeserialize::deserialize(json_config) {
        Err(e) => { return Err(e); }
        Ok(o) => {list_batch = o; }
      }
  
      let mut threads = list_batch.threads();
      let mut index: usize = 0;
  
      if threads <= 0 {
        BatchUtility::println(String::from("Threads must be greater than zero"), Color::Red);
        return Err(crate::ErrorType::Index); 
      }

      let mut exec_batch: VecDeque<(String, VecDeque<(usize, bool)>)> = VecDeque::new();
      
      loop {
        if !list_batch.get_element(index as usize) { break; }

        if list_batch.enabled() { 
          let mut is_queued = false;

          if list_batch.group().len() > 0 {
            for elem in exec_batch.iter_mut() {
              let (group, vec) = elem;

              if group.len() > 0 && group == list_batch.group() {
                vec.push_back((index, false));
                is_queued = true;
                break;
              }
            }
          }

          if !is_queued {
            let mut batch: VecDeque<(usize, bool)> = VecDeque::new();

            batch.push_back((index, true)); // Initial index for a group

            exec_batch.push_back((String::from(list_batch.group()), batch));
          }
        }

        index += 1;
      }

      let batch_completions: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
      let action_successes = Arc::new(Mutex::new(0));
      let action_failures = Arc::new(Mutex::new(0));
      let mut execute_batches: usize = 0;
      
      while threads > 0 { // Fire up initial threads

        let index = ProcessBatch::dequeue_next_batch(&mut exec_batch, "");
        if index == usize::MAX { break; }

        if !ProcessBatch::spawn_batch(&list_batch, index, &batch_completions,
          &action_successes, &action_failures, show_updates) { break; }

        execute_batches += 1;          
        threads -= 1;
      }

      loop { // Monitor completions

        let mut group = String::from("");
        let mut completions: usize = 0;
        match batch_completions.lock() {
          Err(_e) => { }
          Ok(mut o) => { 
            completions = o.len(); 
            
            if completions > 0 {
              match o.pop_front() {
                None => { }
                Some(o2) => { group = o2; }
              }
            }
          }
        }

        if completions > 0 {

          let index = ProcessBatch::dequeue_next_batch(&mut exec_batch, group.as_str());
          if index == usize::MAX { break; }
  
          if !ProcessBatch::spawn_batch(&list_batch, index, &batch_completions,
            &action_successes, &action_failures, show_updates) { break; }  
  
          execute_batches += 1;          
        }

        std::thread::sleep(std::time::Duration::from_millis(crate::THREAD_SLEEP_MILLIS));
      }

      total_batches += execute_batches;

      let mut execute_successes: usize = 0;
      match action_successes.lock() {
        Err(_e) => { }
        Ok(o) => { 
            execute_successes = *o;
            total_successes += execute_successes; 
        }
      }

      let mut execute_failures: usize = 0;
      match action_failures.lock() {
        Err(_e) => { }
        Ok(o) => { 
            execute_failures = *o;
            total_failures += execute_failures; 
        }
      }

      BatchUtility::println( 
        format!("End configuration: Batches = {}, Successes = {}, Failures = {}", 
          execute_batches, execute_successes, execute_failures), Color::White);
    }

    return Ok(ElemBatchResult::new(total_batches, total_successes, total_failures));
  }

  /// Execute a batch.
  /// 
  /// # Arguments
  ///
  /// * `name` - The batch name.
  /// * `group` - Optional group name.
  /// * `locale` - User locale of the batch.
  /// * `actions` - The vector of batch actions.
  /// * `inputs` - The vector of batch inputs.
  /// * `outputs` - The vector of batch outputs.
  /// * `show_updates` - Show updates.
  /// 
  /// # Return
  ///
  /// * An success (Ok) or an error code.

  fn execute_batch(name: String, group: String, locale: String, 
    actions: Vec<ElemBatchAction>, inputs: Vec<ElemBatchIO>, outputs: Vec<ElemBatchIO>, 
    show_updates: bool) -> Result<ElemBatchResult, crate::ErrorType> {

    if group.len() > 0 {
      BatchUtility::println(format!("Processing: {} ({})", name, group), Color::White);
    } else {
      BatchUtility::println(format!("Processing: {}", name), Color::White);
    }

    let engine = CalcEngine::new(UpdateListener::new(move |s| {
      if show_updates { 
        BatchUtility::println(String::from(s), Color::White);
      }
    }));

    let json = CalcJsonDeserialize::new(engine.calc_manager());

    let empty_string = String::from("");
    let mut action_success: usize = 0;
    let mut action_failure: usize = 0;

    for input in inputs {
      let adapter = AdapterFactory::get_adapter(input.io_type());

      let json_input: String;
      match adapter.read(input.location()) {
        Err(e) => {
          BatchUtility::println(format!("Error: {:?}", e), Color::Red);
          return Err(e); 
        }
        Ok(o) => { json_input = o; }
      }

      match json.deserialize(json_input) {
        Err(e) => {
          BatchUtility::println(format!("Error: {:?}", e), Color::Red);
          return Err(e); 
        }
        Ok(_o) => { }
      }
    }

    engine.init_engine(locale.as_str());

    for action in actions {

      if action.action() != crate::ActionType::TemplateCashflow {
          
        if action.cashflow_name().len() > 0 {
          if !engine.calc_reg().list_cashflow().get_element_by_name(action.cashflow_name(), true) {
            BatchUtility::println(String::from("Error: Cannot find cashflow"), Color::Red);
            return Err(crate::ErrorType::Json); 
          }
        } else { // Position to the first cashflow
          if !engine.calc_reg().list_cashflow().get_element(0) { 
            BatchUtility::println(String::from("Error: Cannot find cashflow"), Color::Red);
            return Err(crate::ErrorType::Json); 
          }
        }

        match engine.calc_reg().list_cashflow().list_event() {
          None => { 
            BatchUtility::println(String::from("Error: Cannot find cashflow list event"), Color::Red);
            return Err(crate::ErrorType::Json); 
          }
          Some(o) => {
            let list_event = o; // Position to the selected event
            if action.event_name().len() > 0 {
              if !list_event.get_element_by_name(action.event_name(), true) {
                BatchUtility::println(String::from("Error: Cannot find event"), Color::Red);
                return Err(crate::ErrorType::Json); 
              }
            } else { // Position to the selected event
              if !list_event.get_element_by_type(action.select(), action.iteration()) {
                BatchUtility::println(String::from("Error: Cannot find event"), Color::Red);
                return Err(crate::ErrorType::Json); 
              }
            }
          }
        }
      }

      match action.action() {
        crate::ActionType::CalcValue => {
          match engine.calculate_value(dec!(0.0)) {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        crate::ActionType::CalcPeriods => {
          match engine.calculate_periods(dec!(0.0)) {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        crate::ActionType::CalcYield => {
          match engine.calculate_yield(dec!(0.0)) {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        crate::ActionType::Combine => {
          match engine.combine_cashflow(action.cashflow_name2(), action.cashflow_new(), "") {
              Err(e) => {
                BatchUtility::println(format!("Error: {:?}", e), Color::Red);
                action_failure += 1; 
              }
              Ok(o) => {
                if ProcessBatch::check_action_results(&engine, &action, &o) { 
                  action_success += 1; 
                } else { 
                  action_failure += 1; 
                }
              }
          }
        }
        crate::ActionType::Merge => {
          let opts = BatchUtility::parse_options(action.cashflow_options());
          let interest_event_action_str = opts.get("int-action").unwrap_or(&empty_string);
          let interest_event_action = CoreUtility::get_merge_type(interest_event_action_str);

          match engine.merge_cashflow(action.cashflow_name2(), action.cashflow_new(), "", interest_event_action) {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        crate::ActionType::Split => {
          let opts = BatchUtility::parse_options(action.cashflow_options());
          let all_events_str = opts.get("all-events").unwrap_or(&empty_string);
          let all_events = all_events_str == "true";

          match engine.split_cashflow(all_events) {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        crate::ActionType::Transform => {
          let opts = BatchUtility::parse_options(action.cashflow_options());
          let after_pv_str = opts.get("after-pv").unwrap_or(&empty_string);
          let after_pv = after_pv_str == "true";
          let omit_int_str = opts.get("omit-int").unwrap_or(&empty_string);
          let omit_int = omit_int_str == "true";

          match engine.transform_cashflow(action.cashflow_new(), "", after_pv, omit_int) {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        crate::ActionType::TemplateCashflow => {
          match engine.create_cashflow_from_template_group(action.template_group(), action.cashflow_new(), "") {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
        _ => {
          match engine.balance_cashflow() {
            Err(e) => {
              BatchUtility::println(format!("Error: {:?}", e), Color::Red);
              action_failure += 1; 
            }
            Ok(o) => {
              if ProcessBatch::check_action_results(&engine, &action, &o) { 
                action_success += 1; 
              } else { 
                action_failure += 1; 
              }
            }
          }
        }
      }

    }

    let json = CalcJsonSerialize::new(engine.calc_manager());

    for output in outputs {
      let json_output = json.serialize(output.options());
      
      let adapter = AdapterFactory::get_adapter(output.io_type());
      
      match adapter.write(output.location(), json_output.as_str()) {
        Err(e) => {
          BatchUtility::println(format!("Error: {:?}", e), Color::Red);
          return Err(e); 
        }
        Ok(_o) => { }
      }
    }

    return Ok(ElemBatchResult::new(1, action_success, action_failure));
  }

  /// Report on the action results.
  /// 
  /// # Arguments
  ///
  /// * `engine` - AmFn engine.
  /// * `action` - Batch action.
  /// * `balance_result` - Balance results.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn check_action_results(
    engine: &CalcEngine, action: &ElemBatchAction, balance_result: &ElemBalanceResult) -> bool {

    let mut is_success = true;

    match action.test_type() {
      crate::TestType::Balance => { 
        if engine.round_currency(balance_result.balance()) == action.test_value() {
          BatchUtility::println(
            format!("Test Success: Balance = {}", engine.format_currency(balance_result.balance())), Color::Green);
        } else {
          BatchUtility::println(
            format!("Test Failure: Balance = {}", engine.format_currency(balance_result.balance())), Color::Red);
          is_success = false;
        }
      }
      crate::TestType::Yield => { 
        if engine.round_decimal(balance_result.last_yield()) == action.test_value() {
          BatchUtility::println(
            format!("Test Success: Balance = {}", engine.format_decimal(balance_result.last_yield())), Color::Green);
        } else {
          BatchUtility::println(
            format!("Test Failure: Balance = {}", engine.format_decimal(balance_result.last_yield())), Color::Red);
          is_success = false;
        }
      }
      _ => { 
        if action.test_str().len() == 0 {
          BatchUtility::println(
            format!("Success: Balance = {}", engine.format_currency(balance_result.balance())), Color::Green);
        }
      }
    }

    let stats = engine.create_cashflow_stats();
    if action.test_str().len() > 0 {
      if BatchUtility::equal_stats(&stats, action.test_str()) {
        BatchUtility::println(
          format!("Test Success: Stats = {}",  BatchUtility::format_stats(&stats)), Color::Green);
      } else {
        BatchUtility::println(
          format!("Test Failure: Stats = {}",  BatchUtility::format_stats(&stats)), Color::Red);
        is_success = false;
      }
    }

    return is_success;
  }
    
  /// Dequeue the next batch.
  /// 
  /// # Arguments
  ///
  /// * `exec_batch` - Execution batches.
  /// * `group_param` - Group to retrieve.
  /// 
  /// # Return
  ///
  /// * The index of the batch, otherwise usize::MAX.

  fn dequeue_next_batch(exec_batch: &mut VecDeque<(String, VecDeque<(usize, bool)>)>, group_param: &str) -> usize {

    if group_param.len() > 0 { // Continue with a group sequence once started
      for elem in exec_batch.iter_mut() {
        let (group, vec) = elem;

        if group != group_param || vec.len() == 0 { continue; }

        match vec.pop_front() {
          None => { }
          Some(o) => { 
            let (index, _initial) = o;
            return index;
          }
        }
      }
    }
    
    for elem in exec_batch.iter_mut() { // Look for the next initial batch
      let (_group, vec) = elem;

      if vec.len() == 0 { continue; }

      match vec.pop_front() {
        None => { }
        Some(o) => { 
          let (index, initial) = o;

          if !initial { continue; } 

          return index;
        }
      }
    }

    return usize::MAX;
  }

  /// Print line with a specific color.
  /// 
  /// # Arguments
  ///
  /// * `text` - Text to print.
  /// * `color` - Color to print text.

  pub fn println(text: String, color: Color) -> () {
  
    BatchUtility::println(text, color);
  }
  
  /// Spawn the batch at the passed index.
  /// 
  /// # Arguments
  ///
  /// * `list_batch` - List of batches.
  /// * `index` - Index of batch.
  /// * `batch_completions` - Batch completions.
  /// * `action_successes` - Action successes.
  /// * `action_failures` - Action failures.
  /// * `show_updates` - Show update messages.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn spawn_batch(list_batch: &ListBatch, index: usize, batch_completions: &Arc<Mutex<VecDeque<String>>>,
    action_successes: &Arc<Mutex<usize>>, action_failures: &Arc<Mutex<usize>>,
    show_updates: bool) -> bool {

    if !list_batch.get_element(index) { return false; }

    let name = String::from(list_batch.name());
    let group = String::from(list_batch.group());
    let locale = String::from(list_batch.locale());
    let actions = list_batch.copy_actions();
    let inputs = list_batch.copy_inputs();
    let outputs = list_batch.copy_outputs();

    let tbatch_completions = batch_completions.clone();
    let taction_successes = action_successes.clone();
    let taction_failures = action_failures.clone();

    std::thread::spawn(move || { 

      let tgroup = String::from(group.as_str());

      match ProcessBatch::execute_batch(
        name, group, locale, actions, inputs, outputs, show_updates) {
        Err(e) => { 
          BatchUtility::println(format!("Error: {:?}", e), Color::Red);
        }
        Ok(o) => { 
          match tbatch_completions.lock() {
            Err(_e) => { }
            Ok(mut o2) => { o2.push_back(tgroup); }
          }

          match taction_successes.lock() {
            Err(_e) => { }
            Ok(mut o2) => { *o2 += o.successes(); }
          }

          match taction_failures.lock() {
            Err(_e) => { }
            Ok(mut o2) => { *o2 += o.failures(); }
          }
        }
      }

    });

    return true;
  }

}