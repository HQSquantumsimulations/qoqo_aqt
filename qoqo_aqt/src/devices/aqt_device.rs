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
use roqoqo_aqt::{devices::AqtDevice, AqtApi};

/// AQT quantum simulator device
///
/// Provides endpoint that receives instructions that are simulated and returns measurement results.
#[pyclass(name = "AqtDevice", module = "qoqo_aqt")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AqtDeviceWrapper {
    /// Internal storage of [roqoqo_aqt::AqtDevice]
    pub internal: AqtDevice,
}

#[pymethods]
impl AqtDeviceWrapper {
    /// Create new simulator device.
    ///
    /// Args:
    ///     number_qubits (int): Number of qubits that should be simulated
    #[new]
    pub fn new(number_qubits: usize) -> Self {
        Self {
            internal: AqtDevice::new(number_qubits),
        }
    }

    /// Return a copy of the AqtDevice (copy here produces a deepcopy).
    ///
    /// Returns:
    ///     AqtDevice: A deep copy of self.
    pub fn __copy__(&self) -> AqtDeviceWrapper {
        self.clone()
    }

    /// Return a deep copy of the AqtDevice.
    ///
    /// Returns:
    ///     AqtDevice: A deep copy of self.
    pub fn __deepcopy__(&self, _memodict: Py<PyAny>) -> AqtDeviceWrapper {
        self.clone()
    }

    /// Return the bincode representation of the AqtDevice using the [bincode] crate.
    ///
    /// Returns:
    ///     ByteArray: The serialized AqtDevice (in [bincode] form).
    ///
    /// Raises:
    ///     ValueError: Cannot serialize AqtDevice to bytes.
    pub fn to_bincode(&self) -> PyResult<Py<PyByteArray>> {
        let serialized = serialize(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize AqtDevice to bytes"))?;
        let b: Py<PyByteArray> = Python::with_gil(|py| -> Py<PyByteArray> {
            PyByteArray::new_bound(py, &serialized[..]).into()
        });
        Ok(b)
    }

    /// Convert the bincode representation of the AqtDevice to a AqtDevice using the [bincode] crate.
    ///
    /// Args:
    ///     input (ByteArray): The serialized AqtDevice (in [bincode] form).
    ///
    /// Returns:
    ///     AqtDevice: The deserialized AqtDevice.
    ///
    /// Raises:
    ///     TypeError: Input cannot be converted to byte array.
    ///     ValueError: Input cannot be deserialized to AqtDevice.
    #[staticmethod]
    pub fn from_bincode(input: &Bound<PyAny>) -> PyResult<AqtDeviceWrapper> {
        let bytes = input
            .extract::<Vec<u8>>()
            .map_err(|_| PyTypeError::new_err("Input cannot be converted to byte array"))?;

        Ok(AqtDeviceWrapper {
            internal: deserialize(&bytes[..])
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to AqtDevice"))?,
        })
    }

    /// Return the json representation of the AqtDevice.
    ///
    /// Returns:
    ///     str: The serialized form of AqtDevice.
    ///
    /// Raises:
    ///     ValueError: Cannot serialize AqtDevice to json.
    fn to_json(&self) -> PyResult<String> {
        let serialized = serde_json::to_string(&self.internal)
            .map_err(|_| PyValueError::new_err("Cannot serialize AqtDevice to json"))?;
        Ok(serialized)
    }

    /// Convert the json representation of a AqtDevice to a AqtDevice.
    ///
    /// Args:
    ///     input (str): The serialized AqtDevice in json form.
    ///
    /// Returns:
    ///     AqtDevice: The deserialized AqtDevice.
    ///
    /// Raises:
    ///     ValueError: Input cannot be deserialized to AqtDevice.
    #[staticmethod]
    fn from_json(input: &str) -> PyResult<AqtDeviceWrapper> {
        Ok(AqtDeviceWrapper {
            internal: serde_json::from_str(input)
                .map_err(|_| PyValueError::new_err("Input cannot be deserialized to AqtDevice"))?,
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

    /// Return True or False to indicate whether the remote host URL is https.
    ///
    /// Returns:
    ///     boolean: Whether remote host URL is https or not.
    ///
    pub fn is_https(&self) -> bool {
        self.internal.is_https()
    }
}

/// Convert generic python object to [roqoqo_aqt::AqtDevice].
///
/// Fallible conversion of generic python object to [roqoqo::AqtDevice].
pub fn convert_into_device(input: &Bound<PyAny>) -> Result<AqtDevice, QoqoBackendError> {
    if let Ok(try_downcast) = input.extract::<AqtDeviceWrapper>() {
        return Ok(try_downcast.internal);
    }
    // Everything that follows tries to extract the circuit when two separately
    // compiled python packages are involved
    let get_bytes = input
        .call_method0("to_bincode")
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    let bytes = get_bytes
        .extract::<Vec<u8>>()
        .map_err(|_| QoqoBackendError::CannotExtractObject)?;
    deserialize(&bytes[..]).map_err(|_| QoqoBackendError::CannotExtractObject)
}
