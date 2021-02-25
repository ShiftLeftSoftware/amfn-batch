/// The batch enums.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Batch action type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ActionType {
    /// Balance the cashflow.
    Balance = 1,
    /// Calculate the selected event's value.
    CalcValue = 2,
    /// Calculate the selected event's periods.
    CalcPeriods = 3,
    /// Calculate the overall yield.
    CalcYield = 4,
    /// Combine two cashflows.
    Combine = 5,
    /// Merge two cashflows.
    Merge = 6,
    /// Split a cashflow.
    Split = 7,
    /// Transform a cashflow.
    Transform = 8,
    /// Create a cashflow from a template group.
    TemplateCashflow = 9,
}

/// Batch action test type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TestType {
    /// No test.
    None = 0,
    /// Test the balance.
    Balance = 1,
    /// Test the yield.
    Yield = 2,
}

/// Batch io type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum IOType {
    /// Read or write from a file.
    File = 1,
}
