//! The batch process utility definition.
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

use std::collections::HashMap;
use colored::*;

use amfnengine::engine::*;

/// The batch process utility.

pub struct BatchUtility {
}

/// The batch process utility implementation.

impl BatchUtility {

  /// Compare cashflow statistics.
  /// 
  /// # Arguments
  ///
  /// * `cashflow_stats` - Cashflow statistics to compare.
  /// * `cashflow_stats_str` - Cashflow statistics string to compare.
  /// 
  /// # Return
  ///
  /// * True if equal, otherwise false.

  pub fn equal_stats(cashflow_stats: &ElemCashflowStats, cashflow_stats_str: &str) -> bool {

    let tokens: Vec<_> = cashflow_stats_str.split(',').collect();
    if tokens.len() != 4 { return false; }

    match tokens[0].trim().parse::<usize>() {
      Err(_e) => { }
      Ok(n) => { 
        if n != cashflow_stats.current_values() { return false; }
      }
    }

    match tokens[1].trim().parse::<usize>() {
      Err(_e) => { }
      Ok(n) => { 
        if n != cashflow_stats.interest_changes() { return false; }
      }
    }

    match tokens[2].trim().parse::<usize>() {
      Err(_e) => { }
      Ok(n) => { 
        if n != cashflow_stats.principal_changes() { return false; }
      }
    }

    match tokens[3].trim().parse::<usize>() {
      Err(_e) => { }
      Ok(n) => { 
        if n != cashflow_stats.statistic_values() { return false; }
      }
    }
  
    return true;
  }

  /// Format and return cashflow statistics.
  /// 
  /// # Arguments
  ///
  /// * `cashflow_stats` - Cashflow statistics to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_stats(cashflow_stats: &ElemCashflowStats) -> String {

    return format!("{},{},{},{}", cashflow_stats.current_values(), 
      cashflow_stats.interest_changes(), cashflow_stats.principal_changes(), 
      cashflow_stats.statistic_values());
  }

  /// Parse options and return a hash map.
  /// 
  /// # Arguments
  ///
  /// * `options` - Options to parse.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn parse_options(options: &str) -> HashMap<String, String> {

    let mut map: HashMap<String, String> = HashMap::new();

    for option in options.split(',') {
      let opt: Vec<_> = option.split(':').collect();
      if opt.len() != 2 { continue; }

      map.insert(opt[0].trim().to_lowercase(), opt[1].trim().to_lowercase());
    }

    return map;
  }

  /// Print line colored.
  /// 
  /// # Arguments
  ///
  /// * `text` - Text to print.
  /// * `color` - Color to write text.

  pub fn println(text: String, color: Color) -> () {

    println!("{}", text.color(color).bold());
  }

}