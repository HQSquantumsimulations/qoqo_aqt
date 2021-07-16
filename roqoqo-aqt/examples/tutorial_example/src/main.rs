fn main() {
    println!("To use this example a valid AQT access token is required in the AQT_ACCESS_TOKEN environmental variable");

    use roqoqo::operations;
    use roqoqo::Circuit;

    let mut circuit = Circuit::new();
    circuit += operations::DefinitionBit::new("readout".to_string(), 2, true); // Classical register for readout
    circuit += operations::MolmerSorensenXX::new(0, 1); // Quantum operation
    circuit += operations::PragmaRepeatedMeasurement::new("readout".to_string(), None, 100); // Measuring qubits

    use roqoqo::backends::EvaluatingBackend;
    use roqoqo_aqt::{devices, Backend};

    let device = devices::SimulatorDevice::new(2); //number qubits
    let backend = Backend::new(device.into(), None).unwrap();

    let (bit_registers, _float_registers, _complex_registers) = backend
        .run_circuit(&circuit)
        .expect("Running the circuit failed");
    println!("{:?}", bit_registers["readout"]);
}
