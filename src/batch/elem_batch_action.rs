//! The batch action element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    test_str: String,
}

/// The batch action element implementation.

impl ElemBatchAction {
    /// Create a new object.
    ///
    /// # Arguments
    ///
    /// * `action_param` - Action of the batch action element.
    /// * `template_group_param` - Selection of the batch action template group element.
    /// * `cashflow_name_param` - Selection of the batch action cashflow element.
    /// * `cashflow_name2_param` - Selection of the batch action cashflow element 2.
    /// * `cashflow_new_param` - New cashflow element name.
    /// * `cashflow_options_param` - Batch action cashflow element options.
    /// * `event_name_param` - Selection of the batch action event element by name.
    /// * `select_param` - Selection of the batch action event element by type.
    /// * `iteration_param` - Iteration of the batch action event element by type.
    /// * `test_type_param` - Test type.
    /// * `test_value_param` - Test value.
    /// * `test_str_param` - Test string.
    ///
    /// # Return
    ///
    /// * See description.
    #[allow(clippy::too_many_arguments)]

    pub fn new(
        action_param: crate::ActionType,
        template_group_param: &str,
        cashflow_name_param: &str,
        cashflow_name2_param: &str,
        cashflow_new_param: &str,
        cashflow_options_param: &str,
        event_name_param: &str,
        select_param: amfnengine::ExtensionType,
        iteration_param: usize,
        test_type_param: crate::TestType,
        test_value_param: Decimal,
        test_str_param: &str,
    ) -> ElemBatchAction {
        ElemBatchAction {
            action: action_param,
            template_group: String::from(template_group_param),
            cashflow_name: String::from(cashflow_name_param),
            cashflow_name2: String::from(cashflow_name2_param),
            cashflow_new: String::from(cashflow_new_param),
            cashflow_options: String::from(cashflow_options_param),
            event_name: String::from(event_name_param),
            select: select_param,
            iteration: iteration_param,
            test_type: test_type_param,
            test_value: test_value_param,
            test_str: String::from(test_str_param),
        }
    }

    /// Get the action type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn action(&self) -> crate::ActionType {
        self.action
    }

    /// Get the name of the batch action template group element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn template_group(&self) -> &str {
        self.template_group.as_str()
    }

    /// Get the name of the batch action cashflow element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_name(&self) -> &str {
        self.cashflow_name.as_str()
    }

    /// Get the name of the batch action cashflow element 2.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_name2(&self) -> &str {
        self.cashflow_name2.as_str()
    }

    /// Get the new cashflow element name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_new(&self) -> &str {
        self.cashflow_new.as_str()
    }

    /// Get the batch action cashflow element options.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_options(&self) -> &str {
        self.cashflow_options.as_str()
    }

    /// Get the name of the batch action event element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_name(&self) -> &str {
        self.event_name.as_str()
    }

    /// Get the event select type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn select(&self) -> amfnengine::ExtensionType {
        self.select
    }

    /// Get the iteration of the event select by type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn iteration(&self) -> usize {
        self.iteration
    }

    /// Get the test type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn test_type(&self) -> crate::TestType {
        self.test_type
    }

    /// Get the test value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn test_value(&self) -> Decimal {
        self.test_value
    }

    /// Get the test string.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn test_str(&self) -> &str {
        self.test_str.as_str()
    }
}
