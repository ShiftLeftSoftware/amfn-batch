//! Batch adapter file definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use colored::*;
use std::fs;

use super::BatchUtility;
/// Batch IOAdapterTrait adapter file.

pub struct AdapterFile {}
/// Batch IOAdapterTrait adapter file implementation.
impl crate::IOAdapterTrait for AdapterFile {
    /// Read input from a file.
    ///
    /// # Arguments
    ///
    /// * `location` - The file path to read.
    ///
    /// # Return
    ///
    /// * The input string or an error code.

    fn read(&self, location: &str) -> Result<String, amfnengine::ErrorType> {
        match fs::read_to_string(location) {
            Err(e) => {
                BatchUtility::println(format!("Error: {:?}", e), Color::Red);
                Err(amfnengine::ErrorType::Json)
            }
            Ok(o) => Ok(o),
        }
    }

    /// Write output to a file.
    ///
    /// # Arguments
    ///
    /// * `location` - The file path to write.
    /// * `text` - The text to write.
    ///
    /// # Return
    ///
    /// * ERROR_NONE or an error code.

    fn write(&self, location: &str, text: &str) -> Result<(), amfnengine::ErrorType> {
        let path = std::path::Path::new(location);
        match path.parent() {
            None => {}
            Some(o) => match fs::create_dir_all(o) {
                Err(e) => {
                    BatchUtility::println(format!("Error: {:?}", e), Color::Red);
                    return Err(amfnengine::ErrorType::Json);
                }
                Ok(_o) => {}
            },
        }

        match fs::write(location, text) {
            Err(e) => {
                BatchUtility::println(format!("Error: {:?}", e), Color::Red);
                Err(amfnengine::ErrorType::Json)
            }
            Ok(_o) => Ok(()),
        }
    }
}

/// Batch adapter file implementation.

impl AdapterFile {
    /// Read input from a file.
    ///
    /// # Arguments
    ///
    /// * `location` - The file path to read.
    ///
    /// # Return
    ///
    /// * The input string or an error code.

    pub fn new() -> AdapterFile {
        AdapterFile {}
    }
}
