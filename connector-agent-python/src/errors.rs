use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use thiserror::Error;

#[allow(unused)]
pub type Result<T> = std::result::Result<T, ConnectorAgentPythonError>;

/// Errors that can be raised from this library.
#[derive(Error, Debug)]
pub enum ConnectorAgentPythonError {
    /// The required type does not same as the schema defined.
    #[error("Unknown pandas data type: {0}.")]
    UnknownPandasType(String),

    #[error("Python: {0}.")]
    PythonError(String),

    #[error("Unexpected return type, expect: {0}, but get {1}.")]
    UnexpectedReturnType(String, String),

    #[error(transparent)]
    ConnectorAgentError(#[from] connector_agent::ConnectorAgentError),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ConnectorAgentPythonError> for PyErr {
    fn from(e: ConnectorAgentPythonError) -> PyErr {
        PyRuntimeError::new_err(format!("{}", e))
    }
}

impl From<PyErr> for ConnectorAgentPythonError {
    fn from(e: PyErr) -> ConnectorAgentPythonError {
        ConnectorAgentPythonError::PythonError(format!("{}", e))
    }
}
