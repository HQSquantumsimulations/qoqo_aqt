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
use bincode::{deserialize, serialize};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use qoqo::QoqoBackendError;
use roqoqo_aqt::devices::{AqtDevice, SimulatorDevice};

/// AQT quantum simulator device
///
/// Provides endpoint that receives instructions that are simulated and returns measurement results.
#[pyclass(name = "SimulatorDevice", module = "qoqo_aqt")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulatorDeviceWrapper {
    /// Internal storage of [roqoqo_aqt::SimulatorDevice]
    pub internal: SimulatorDevice,
}

#[pymethods]
impl SimulatorDeviceWrapper {
    /// Create new simulator device.
    ///
    /// Args:
    ///     number_qubits (int): Number of qubits that should be simulated
    #[new]
    pub fn new(number_qubits: usize) -> Self {
        Self {
            internal: SimulatorDevice::new(number_qubits),
        }
    }

    /// Return a copy of the SimulatorDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     SimulatorDevice: A deep copy of self.
    pub fn __copy__(&self) -> SimulatorDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the SimulatorDevice.
    ///
    /// Returns:
    ///     SimulatorDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> SimulatorDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the SimulatorDevice using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized SimulatorDevice (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize SimulatorDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize SimulatorDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the SimulatorDevice to a SimulatorDevice using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized SimulatorDevice (in [bincode] form).
    ///
    /// Returns:
    ///     SimulatorDevice: The deserialized SimulatorDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to SimulatorDevice.
    #[staticmethod]
    pub fn from_bincode(input: &PyAny) -> PyResult<SimulatorDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(SimulatorDeviceWrapper {
            internal: deserialize(&bytes[..]).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to SimulatorDevice")
            })?,
        })
    }

    /// Return the json representation of the SimulatorDevice.
    ///
    /// Returns:
    ///     str: The serialized form of SimulatorDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize SimulatorDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize SimulatorDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a SimulatorDevice to a SimulatorDevice.
    ///
    /// Args:
    ///     input (str): The serialized SimulatorDevice in json form.
    ///
    /// Returns:
    ///     SimulatorDevice: The deserialized SimulatorDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to SimulatorDevice.
    #[staticmethod]
    fn from_json(input: &str) -> PyResult<SimulatorDeviceWrapper> {
        Ok(SimulatorDeviceWrapper {
            internal: serde_json::from_str(input).map_err(|_| {
                PyValueError::new_err("Input cannot be deserialized to SimulatorDevice")
            })?,
        })
    }

    /// Return number of qubits simulated by Simulator.
    ///
    /// Returns:
    ///     int: The number of qubits.
    ///
    pub fn number_qubits(&self) -> usize {
        self.internal.number_qubits()
    }

    /// Return the URL of the remote host executing Circuits.
    ///
    /// Returns:
    ///     str: The URL of the remote host executing the Circuits.
    ///
    pub fn remote_host(&self) -> String {
        self.internal.remote_host().to_string()
    }

    /// Return the bincode representation of the Enum variant of the Device.
    ///
    /// Only used for internal interfacing.
    ///
    /// Returns:
    ///     ByteArray: The serialized AqtDevice (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize Device to bytes.
    pub fn _enum_to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let aqt_enum: AqtDevice = (&self.internal).into();
        let serialized = serialize(&aqt_enum)
            .map_err(|_| PyValueError::new_err("Cannot serialize SimulatorDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new(py, &serialized[..]).into()
        });
        Ok(b)
    }
}

/// Convert generic python object to [roqoqo_aqt::AqtDevice].
///
/// Fallible conversion of generic python object to [roqoqo::SimulatorDevice].
pub fn convert_into_device(input: &PyAny) -> Result<AqtDevice, QoqoBackendError> {
    // Everything that follows tries to extract the circuit when two separately
    // compiled python packages are involved
    let get_bytes = input
        .call_method0("_enum_to_bincode")
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    deserialize(&bytes[..]).map_err(|_| QoqoBackendError::CannotExtractObject)
}
