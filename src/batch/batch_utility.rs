//! The batch process utility definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

use colored::*;
use std::collections::HashMap;

use amfnengine::engine::*;

/// The batch process utility.

pub struct BatchUtility {}

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
        if tokens.len() != 4 {
            return false;
        }

        match tokens[0].trim().parse::<usize>() {
            Err(_e) => {}
            Ok(n) => {
                if n != cashflow_stats.current_values() {
                    return false;
                }
            }
        }

        match tokens[1].trim().parse::<usize>() {
            Err(_e) => {}
            Ok(n) => {
                if n != cashflow_stats.interest_changes() {
                    return false;
                }
            }
        }

        match tokens[2].trim().parse::<usize>() {
            Err(_e) => {}
            Ok(n) => {
                if n != cashflow_stats.principal_changes() {
                    return false;
                }
            }
        }

        match tokens[3].trim().parse::<usize>() {
            Err(_e) => {}
            Ok(n) => {
                if n != cashflow_stats.statistic_values() {
                    return false;
                }
            }
        }
        true
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
        return format!(
            "{},{},{},{}",
            cashflow_stats.current_values(),
            cashflow_stats.interest_changes(),
            cashflow_stats.principal_changes(),
            cashflow_stats.statistic_values()
        );
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
            if opt.len() != 2 {
                continue;
            }

            map.insert(opt[0].trim().to_lowercase(), opt[1].trim().to_lowercase());
        }

        map
    }

    /// Print line colored.
    ///
    /// # Arguments
    ///
    /// * `text` - Text to print.
    /// * `color` - Color to write text.

    pub fn println(text: String, color: Color) {
        println!("{}", text.color(color).bold());
    }
}
