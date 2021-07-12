from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import ClassicalRegister
from qoqo_aqt.devices import SimulatorDevice
from qoqo_aqt import Backend
import numpy as np

print("To use this example a valid AQT access token is required in the AQT_ACCESS_TOKEN evironmental variable")

circuit = Circuit()
circuit += ops.DefinitionBit("readout", length=5, is_output=True)
circuit += ops.MolmerSorensenXX(control=0, target=1)  # ["MS", 0.5, [0,1]]
circuit += ops.RotateX(qubit=0, theta=np.pi / 2)  # ["X", 0.5, [0]]
circuit += ops.RotateY(qubit=4, theta=np.pi)  # ["Y", 1.0, [4]]]
circuit += ops.PragmaRepeatedMeasurement(readout="readout", number_measurements=100)

device = SimulatorDevice(number_qubits=5)

backend = Backend(device=device, access_token=None)
# Access token is read from AQT_ACCESS_TOKEN environmental variable

measurement = ClassicalRegister(constant_circuit=None, circuits=[circuit])

(bit_registers, float_registers, complex_registers) = backend.run_circuit(circuit)

print(bit_registers["readout"])