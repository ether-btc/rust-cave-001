//! Custom error types for rust-cave-001 compression pipeline.
//! Provides specific, actionable error messages instead of raw PyValueError.

use std::fmt;

/// Custom error types for rust-cave-001 compression pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionError {
    /// Input is too short to be logically complete
    TooShort(String),
    /// Active voice transformation failed (regex or conjugation issue)
    #[allow(dead_code)]
    VoiceTransformFailed(String),
    /// Text is empty
    #[allow(dead_code)]
    EmptyInput,
    /// Pipeline reached inconsistent state
    PipelineError(String),
}

impl CompressionError {
    /// Convert to a PyErr (for Python-callable functions)
    pub fn into_pyerr(self) -> pyo3::PyErr {
        pyo3::exceptions::PyValueError::new_err(self.to_string())
    }
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::TooShort(s) => {
                write!(f, "Text is too short: '{}' — provide at least 2 words", s)
            }
            CompressionError::VoiceTransformFailed(s) => {
                write!(f, "Voice transform failed: {}", s)
            }
            CompressionError::EmptyInput => {
                write!(f, "Input is empty — provide at least 2 words")
            }
            CompressionError::PipelineError(s) => {
                write!(f, "Pipeline error: {}", s)
            }
        }
    }
}

impl std::error::Error for CompressionError {}

impl From<std::string::FromUtf8Error> for CompressionError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CompressionError::PipelineError(format!("UTF-8 error: {}", e))
    }
}
