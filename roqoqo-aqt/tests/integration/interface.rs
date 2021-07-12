// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use roqoqo::operations;
use roqoqo::Circuit;
use roqoqo::RoqoqoBackendError;
use roqoqo_aqt::{call_circuit, call_operation, AqtInstruction};
use std::collections::HashMap;
use test_case::test_case;

/// Test SingleQubitGate alpha, beta, global phase
#[test_case(
    operations::RotateZ::new(0,0.0.into()).into(),
    AqtInstruction::SingleParameterInstruction((
        "Z".to_string(),
        0.0,
        vec![0],
    ));
    "RotateZ")]
#[test_case(
        operations::RotateX::new(0,0.0.into()).into(),
        AqtInstruction::SingleParameterInstruction((
            "X".to_string(),
            0.0,
            vec![0],
        ));
        "RotateX")]
#[test_case(
            operations::RotateY::new(0,0.0.into()).into(),
            AqtInstruction::SingleParameterInstruction((
                "Y".to_string(),
                0.0,
                vec![0],
            ));
            "RotateY")]
#[test_case(
            operations::PauliZ::new(0).into(),
            AqtInstruction::SingleParameterInstruction((
                "Z".to_string(),
                1.0,
                vec![0],
            ));
            "PauliZ")]
#[test_case(
            operations::PauliX::new(0).into(),
            AqtInstruction::SingleParameterInstruction((
                "X".to_string(),
                1.0,
                vec![0],
            ));
            "PauliX")]
#[test_case(
            operations::PauliY::new(0).into(),
            AqtInstruction::SingleParameterInstruction((
                "Y".to_string(),
                1.0,
                vec![0],
            ));
            "PauliY")]
#[test_case(
            operations::MolmerSorensenXX::new(0,1).into(),
            AqtInstruction::SingleParameterInstruction((
                "MS".to_string(),
                0.5,
                vec![0,1],
            ));
            "MS")]
#[test_case(
            operations::VariableMSXX::new(0,1, 0.0.into()).into(),
            AqtInstruction::SingleParameterInstruction((
                "MS".to_string(),
                0.0,
                vec![0,1],
            ));
            "VariableMS")]
fn test_passing_interface(operation: operations::Operation, instruction: AqtInstruction) {
    let called = call_operation(&operation).unwrap().unwrap();
    assert_eq!(instruction, called);
}

#[test_case(operations::PragmaSetNumberOfMeasurements::new(1,"ro".to_string()).into(); "PragmaSetNumberOfMeasurements")]
#[test_case(operations::PragmaBoostNoise::new(2.0.into()).into(); "PragmaBoostNoise")]
#[test_case(operations::PragmaStopParallelBlock::new(vec![0, 1],1.0.into()).into(); "PragmaStopParallelBlock")]
#[test_case(operations::PragmaStopDecompositionBlock::new(vec![0, 1]).into(); "PragmaStopDecompositionBlock")]
#[test_case(operations::PragmaStartDecompositionBlock::new(vec![0, 1], HashMap::new()).into(); "PragmaStartDecompositionBlock")]
#[test_case(operations::PragmaGlobalPhase::new(1.0.into()).into(); "PragmaGlobalPhase")]
#[test_case(operations::DefinitionBit::new("test".to_string(),1,false).into(); "DefinitionBit")]
#[test_case(operations::DefinitionFloat::new("test".to_string(),1,false).into(); "DefinitionFloat")]
#[test_case(operations::DefinitionComplex::new("test".to_string(),1,false).into(); "DefinitionComplex")]
#[test_case(operations::InputSymbolic::new("test".to_string(),1.0.into()).into(); "InputSymbolic")]
fn test_passing_withou_error(operation: operations::Operation) {
    let called = call_operation(&operation).unwrap();
    assert_eq!(None, called);
}

#[test_case(operations::CNOT::new(0,1).into(); "CNOT")]
fn test_failure(operation: operations::Operation) {
    let called = call_operation(&operation);
    match called {
        Err(RoqoqoBackendError::OperationNotInBackend { .. }) => {}
        _ => panic!("Not the right error"),
    }
}

#[test]
fn test_call_circuit() {
    let mut circuit = Circuit::new();
    circuit += operations::MolmerSorensenXX::new(0, 1);
    let res = call_circuit(&circuit).unwrap();
    let res_comp = vec![AqtInstruction::SingleParameterInstruction((
        "MS".to_string(),
        0.5,
        vec![0, 1],
    ))];
    assert_eq!(res, res_comp)
}
