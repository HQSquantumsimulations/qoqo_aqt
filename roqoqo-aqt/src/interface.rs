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

use roqoqo::operations::*;
use roqoqo::Circuit;
use roqoqo::RoqoqoBackendError;

// Pragma operations that are ignored by backend and do not throw an error
const ALLOWED_OPERATIONS: &[&str; 12] = &[
    "PragmaSetNumberOfMeasurements",
    "PragmaBoostNoise",
    "PragmaStopParallelBlock",
    "PragmaGlobalPhase",
    "DefinitionBit",
    "DefinitionFloat",
    "DefinitionComplex",
    "InputSymbolic",
    "InputBit",
    "PragmaRepeatedMeasurement",
    "PragmaStartDecompositionBlock",
    "PragmaStopDecompositionBlock",
    // "PragmaLoop",                  // CHECK
    // "PhaseShiftedControlledPhase", // CHECK
];

/// Representation for AQT backend instructions serialized to Json
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AqtInstruction {
    /// Instruction involving a single parameter
    SingleParameterInstruction((String, f64, Vec<u32>)),
    /// Instruction involving two parameters
    TwoParameterInstruction((String, f64, f64, Vec<u32>)),
}

/// Instruction on AQT device that has a single float parameter
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SingleParameterInstruction((String, f64, Vec<u32>));

/// Converts all operations in a [roqoqo::Circuit] into instructions for AQT Hardware or AQT Simulators
///
/// # Arguments
///
/// `circuit` - The [roqoqo::Circuit] that is converted
///
/// # Returns
///
/// `Vec<AqtInstruction>` - List of converted instructions
/// `RoqoqoBackendError::OperationNotInBackend` - Error when [roqoqo::operations::Operation] can not be converted
pub fn call_circuit(circuit: &Circuit) -> Result<Vec<AqtInstruction>, RoqoqoBackendError> {
    let mut circuit_vec: Vec<AqtInstruction> = Vec::new();
    for op in circuit.iter() {
        if let Some(instruction) = call_operation(op)? {
            circuit_vec.push(instruction);
        }
    }
    Ok(circuit_vec)
}

/// Converts a [roqoqo::operations::Operation] into an instruction for AQT Hardware or AQT Simulators
///
/// # Arguments
///
/// `operation` - The [roqoqo::operations::Operation] that is converted
///
/// # Returns
///
/// `AqtInstruction` - Converted instruction
/// `RoqoqoBackendError::OperationNotInBackend` - Error when [roqoqo::operations::Operation] can not be converted
pub fn call_operation(operation: &Operation) -> Result<Option<AqtInstruction>, RoqoqoBackendError> {
    match operation {
        Operation::RotateZ(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "Z".to_string(),
            *op.theta().float()? / std::f64::consts::PI,
            vec![*op.qubit() as u32],
        )))),
        Operation::RotateX(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "X".to_string(),
            *op.theta().float()? / std::f64::consts::PI,
            vec![*op.qubit() as u32],
        )))),
        Operation::RotateY(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "Y".to_string(),
            *op.theta().float()? / std::f64::consts::PI,
            vec![*op.qubit() as u32],
        )))),
        Operation::PauliZ(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "Z".to_string(),
            1.0,
            vec![*op.qubit() as u32],
        )))),
        Operation::PauliX(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "X".to_string(),
            1.0,
            vec![*op.qubit() as u32],
        )))),
        Operation::PauliY(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "Y".to_string(),
            1.0,
            vec![*op.qubit() as u32],
        )))),
        Operation::VariableMSXX(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "MS".to_string(),
            *op.theta().float()? / std::f64::consts::PI,
            vec![*op.control() as u32, *op.target() as u32],
        )))),
        Operation::MolmerSorensenXX(op) => Ok(Some(AqtInstruction::SingleParameterInstruction((
            "MS".to_string(),
            0.5,
            vec![*op.control() as u32, *op.target() as u32],
        )))),
        _ => {
            if ALLOWED_OPERATIONS.contains(&operation.hqslang()) {
                Ok(None)
            } else {
                Err(RoqoqoBackendError::OperationNotInBackend {
                    backend: "AQT",
                    hqslang: operation.hqslang(),
                })
            }
        }
    }
}
