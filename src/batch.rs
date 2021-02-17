//! The batch modules.
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

mod adapter_file;
use adapter_file::AdapterFile;

pub mod batch_utility;
pub use batch_utility::BatchUtility;

pub mod elem_batch_action;
pub use elem_batch_action::ElemBatchAction;

pub mod elem_batch_io;
pub use elem_batch_io::ElemBatchIO;

pub mod elem_batch;
pub use elem_batch::ElemBatch;

pub mod elem_batch_result;
pub use elem_batch_result::ElemBatchResult;

pub mod adapter_factory;
pub use adapter_factory::AdapterFactory;

pub mod json_deserialize;
pub use json_deserialize::JsonDeserialize;

pub mod list_batch;
pub use list_batch::ListBatch;