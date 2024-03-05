"""Qoqo AQT tutorial example."""
from qoqo import Circuit
from qoqo import operations as ops
from qoqo_aqt import devices, Backend

print(
    "To use this example a valid AQT access token\
      is required in the AQT_ACCESS_TOKEN environmental variable"
)


circuit = Circuit()
circuit += ops.DefinitionBit(
    "readout", length=2, is_output=True
)  # Classical register for readout
circuit += ops.MolmerSorensenXX(control=0, target=1)  # Quantum operations
circuit += ops.PragmaRepeatedMeasurement(
    readout="readout", number_measurements=100
)  # Measuring qubits


device = devices.SimulatorDevice(number_qubits=2)
backend = Backend(device=device, access_token=None)

(bit_registers, float_registers, complex_registers) = backend.run_circuit(circuit)
print(bit_registers["readout"])
