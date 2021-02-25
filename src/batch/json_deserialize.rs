//! The batch deserialize json element.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use colored::*;
use json::JsonValue;
use rust_decimal::prelude::*;

use amfnengine::core::CoreUtility;
use amfnengine::dec;

use super::{BatchUtility, ElemBatchAction, ElemBatchIO, ListBatch};

/// The batch deserialize json.

pub struct JsonDeserialize {}

/// The batch deserialize json implementation.

impl JsonDeserialize {
    /// Deserialize and ingest serialized Json.
    ///
    /// # Arguments
    ///
    /// * `input_param` - Input containing serialized Json.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    pub fn deserialize(input_param: String) -> Result<ListBatch, crate::ErrorType> {
        let data: JsonValue;
        match json::parse(input_param.as_str()) {
            Err(_e) => {
                return Err(crate::ErrorType::Json);
            }
            Ok(o) => {
                data = o;
            }
        }

        if data["threads"].is_null() || data["batches"].is_null() {
            return Err(crate::ErrorType::Json);
        }

        let threads: usize;
        match data["threads"].as_usize() {
            None => {
                return Err(crate::ErrorType::Json);
            }
            Some(n) => {
                threads = n;
            }
        }

        let mut list_batches = ListBatch::new(threads);
        let result = JsonDeserialize::deserialize_batches(&data["batches"], &mut list_batches);
        match result {
            Err(e) => Err(e),
            Ok(_o) => Ok(list_batches),
        }
    }

    /// Deserialize and ingest Json batches.
    ///
    /// # Arguments
    ///
    /// * `json_batches` - Json value for batches.
    /// * `batches` - List of batches.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    fn deserialize_batches(
        json_batches: &JsonValue,
        batches: &mut ListBatch,
    ) -> Result<(), crate::ErrorType> {
        let mut index: usize = 0;

        loop {
            let batch = &json_batches[index];
            if batch.is_null() {
                break;
            }

            let name: &str;
            match batch["name"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    name = o;
                }
            }

            let group: &str;
            match batch["group"].as_str() {
                None => {
                    group = "";
                } // Optional
                Some(o) => {
                    group = o;
                }
            }

            let locale: &str;
            match batch["locale"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    locale = o;
                }
            }

            let enabled: bool;
            match batch["enabled"].as_bool() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    enabled = o;
                }
            }
            let actions: Vec<ElemBatchAction>;
            if batch["actions"].is_null() {
                actions = Vec::new();
            } else {
                let result = JsonDeserialize::deserialize_actions(&batch["actions"]);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(o) => {
                        actions = o;
                    }
                }
            }

            let inputs: Vec<ElemBatchIO>;
            if batch["inputs"].is_null() {
                inputs = Vec::new();
            } else {
                let result = JsonDeserialize::deserialize_io(&batch["inputs"]);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(o) => {
                        inputs = o;
                    }
                }
            }

            let outputs: Vec<ElemBatchIO>;
            if batch["outputs"].is_null() {
                outputs = Vec::new();
            } else {
                let result = JsonDeserialize::deserialize_io(&batch["outputs"]);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(o) => {
                        outputs = o;
                    }
                }
            }

            batches.add_batch(name, group, locale, enabled, actions, inputs, outputs);

            index += 1;
        }

        Ok(())
    }

    /// Deserialize and ingest batch actions.
    ///
    /// # Arguments
    ///
    /// * `json_actions` - Json value for batch actions.
    ///
    /// # Return
    ///
    /// * List of batch actions if successful, otherwise error code.

    fn deserialize_actions(
        json_actions: &JsonValue,
    ) -> Result<Vec<ElemBatchAction>, crate::ErrorType> {
        let mut actions: Vec<ElemBatchAction> = Vec::new();
        let mut index: usize = 0;

        loop {
            let act = &json_actions[index];
            if act.is_null() {
                break;
            }

            let action: crate::ActionType;
            match act["action"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    action = JsonDeserialize::get_action(o);
                }
            }

            let mut template_group = String::from("");
            if !act["template-group"].is_null() {
                match act["template-group"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        template_group = String::from(o);
                    }
                }
            }

            let mut cashflow_name = String::from("");
            if !act["cashflow-name"].is_null() {
                match act["cashflow-name"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        cashflow_name = String::from(o);
                    }
                }
            }

            let mut cashflow_name2 = String::from("");
            if !act["cashflow-name2"].is_null() {
                match act["cashflow-name2"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        cashflow_name2 = String::from(o);
                    }
                }
            }

            let mut cashflow_new = String::from("");
            if !act["cashflow-new"].is_null() {
                match act["cashflow-new"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        cashflow_new = String::from(o);
                    }
                }
            }

            let mut cashflow_options = String::from("");
            if !act["cashflow-options"].is_null() {
                match act["cashflow-options"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        cashflow_options = String::from(o);
                    }
                }
            }

            let mut event_name = String::from("");
            if !act["event-name"].is_null() {
                match act["event-name"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        event_name = String::from(o);
                    }
                }
            }

            let mut select = amfnengine::ExtensionType::PrincipalChange;
            if !act["select"].is_null() {
                match act["select"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        select = CoreUtility::get_event_type(o);
                    }
                }
            }

            let mut iteration: usize = 1;
            if !act["iteration"].is_null() {
                match act["iteration"].as_usize() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(n) => {
                        iteration = n;
                    }
                }
            }

            let mut test_type = crate::TestType::None;
            if !act["test-type"].is_null() {
                match act["test-type"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(n) => {
                        test_type = JsonDeserialize::get_test_type(n);
                    }
                }
            }

            let mut test_value = dec!("0.0");
            if !act["test-value"].is_null() {
                match act["test-value"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(n) => {
                        test_value = dec!(n);
                    }
                }
            }

            let mut test_str = "";
            if !act["test-str"].is_null() {
                match act["test-str"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        test_str = o;
                    }
                }
            }

            actions.push(ElemBatchAction::new(
                action,
                template_group.as_str(),
                cashflow_name.as_str(),
                cashflow_name2.as_str(),
                cashflow_new.as_str(),
                cashflow_options.as_str(),
                event_name.as_str(),
                select,
                iteration,
                test_type,
                test_value,
                test_str,
            ));

            index += 1;
        }

        Ok(actions)
    }

    /// Deserialize and ingest batch ios.
    ///
    /// # Arguments
    ///
    /// * `json_ios` - Json value for batch ios.
    ///
    /// # Return
    ///
    /// * List of batch ios if successful, otherwise error code.

    fn deserialize_io(json_ios: &JsonValue) -> Result<Vec<ElemBatchIO>, crate::ErrorType> {
        let mut ios: Vec<ElemBatchIO> = Vec::new();
        let mut index: usize = 0;

        loop {
            let io = &json_ios[index];
            if io.is_null() {
                break;
            }

            let io_type: &str;
            match io["io-type"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    io_type = o;
                }
            }

            let location: &str;
            match io["location"].as_str() {
                None => {
                    return Err(crate::ErrorType::Json);
                }
                Some(o) => {
                    location = o;
                }
            }

            let mut options: usize = 0;
            if !io["options"].is_null() {
                let options_str: &str;
                match io["options"].as_str() {
                    None => {
                        return Err(crate::ErrorType::Json);
                    }
                    Some(o) => {
                        options_str = o;
                    }
                }

                options = JsonDeserialize::get_options(options_str);
            }

            ios.push(ElemBatchIO::new(
                JsonDeserialize::get_io_type(io_type),
                location,
                options,
            ));

            index += 1;
        }

        Ok(ios)
    }

    /// Parse and return an action type.
    ///
    /// # Arguments
    ///
    /// * `action_param` - Action type string.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_action(action_param: &str) -> crate::ActionType {
        match action_param.to_lowercase().as_str() {
            "calculate-value" => crate::ActionType::CalcValue,
            "calculate-periods" => crate::ActionType::CalcPeriods,
            "calculate-yield" => crate::ActionType::CalcYield,
            "combine-cashflow" => crate::ActionType::Combine,
            "merge-cashflow" => crate::ActionType::Merge,
            "split-cashflow" => crate::ActionType::Split,
            "transform-cashflow" => crate::ActionType::Transform,
            "template-cashflow" => crate::ActionType::TemplateCashflow,
            _ => crate::ActionType::Balance, // "balance-cashflow"
        }
    }

    /// Parse and return an io type.
    ///
    /// # Arguments
    ///
    /// * `io_type_param` - An io type string.
    ///
    /// # Return
    ///
    /// * See description.
    #[allow(clippy::match_single_binding)]

    fn get_io_type(io_type_param: &str) -> crate::IOType {
        match io_type_param.to_lowercase().as_str() {
            _ => crate::IOType::File,
        }
    }

    /// Parse and return the options.
    ///
    /// # Arguments
    ///
    /// * `options_param` - Options string.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_options(options_param: &str) -> usize {
        let mut options: usize = 0;

        for option in options_param.split('|') {
            let opt = option.trim().to_lowercase();

            match opt.as_str() {
                "preferences" => {
                    options += crate::JSON_SERIALIZE_PREFERENCES;
                }
                "templates" => {
                    options += crate::JSON_SERIALIZE_TEMPLATES;
                }
                "exchange-rates" => {
                    options += crate::JSON_SERIALIZE_EXCHANGE_RATES;
                }
                "cashflow-preferences" => {
                    options += crate::JSON_SERIALIZE_CASHFLOW_PREFERENCES;
                }
                "event-list" => {
                    options += crate::JSON_SERIALIZE_EVENT_LIST;
                }
                "amortization-list" => {
                    options += crate::JSON_SERIALIZE_AMORTIZATION_LIST;
                }
                "amortization-rollups" => {
                    options += crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS;
                }
                "amortization-details" => {
                    options += crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS;
                }
                _ => {
                    BatchUtility::println(format!("Unrecognized option: {:?}", option), Color::Red);
                }
            }
        }

        options
    }

    /// Parse and return the test type.
    ///
    /// # Arguments
    ///
    /// * `test_type_param` - Test type.
    ///
    /// # Return
    ///
    /// * See description.

    fn get_test_type(test_type_param: &str) -> crate::TestType {
        let mut test_type = crate::TestType::None;

        match test_type_param {
            "balance" => {
                test_type = crate::TestType::Balance;
            }
            "yield" => {
                test_type = crate::TestType::Yield;
            }
            _ => {
                BatchUtility::println(
                    format!("Unrecognized test type: {:?}", test_type_param),
                    Color::Red,
                );
            }
        }

        test_type
    }
}
