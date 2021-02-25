//! The batch element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    outputs: Vec<ElemBatchIO>,
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

    pub fn new(
        name_param: &str,
        group_param: &str,
        locale_param: &str,
        enabled_param: bool,
        actions_param: Vec<ElemBatchAction>,
        inputs_param: Vec<ElemBatchIO>,
        outputs_param: Vec<ElemBatchIO>,
    ) -> ElemBatch {
        ElemBatch {
            name: String::from(name_param),
            group: String::from(group_param),
            locale: String::from(locale_param),
            enabled: enabled_param,
            actions: actions_param,
            inputs: inputs_param,
            outputs: outputs_param,
        }
    }

    /// Get the name of the batch.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the optional group of the batch.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn group(&self) -> &str {
        self.group.as_str()
    }

    /// Get the user locale of the batch.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn locale(&self) -> &str {
        self.locale.as_str()
    }

    /// Get the enabled for execution.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the actions of the batch.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn actions(&self) -> &Vec<ElemBatchAction> {
        &self.actions
    }

    /// Get the inputs of the batch.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn inputs(&self) -> &Vec<ElemBatchIO> {
        &self.inputs
    }

    /// Get the outputs of the batch.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn outputs(&self) -> &Vec<ElemBatchIO> {
        &self.outputs
    }
}
