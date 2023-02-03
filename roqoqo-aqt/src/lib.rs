// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
#![allow(clippy::borrow_deref_ref)]
#![deny(missing_docs)]
#![warn(private_intra_doc_links)]
#![warn(missing_crate_level_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]
#![deny(missing_debug_implementations)]

//! # roqoqo-aqt
//!
//! AQT interface and backend for roqoqo quantum computing toolkit.
//!
//! roqoqo-aqt provides backends to send roqoqo quantum circuits to AQT machines

mod interface;
pub use interface::{call_circuit, call_operation, AqtInstruction};
mod backend;
pub use backend::Backend;
pub mod devices;
