#[cfg(feature = "compiler")]
use crate::compiler::ExecutionOutput;
#[cfg(feature = "compiler")]
use regex::Regex;
use serde::{Deserialize, Serialize};

// ============================================================================
// Legacy Success Criteria (for backward compatibility)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SuccessCriteria {
    /// Output must match exactly
    ExactMatch { expected_stdout: String },

    /// Output must match regex pattern
    RegexMatch { regex: String },

    /// Count occurrences of a token in output
    OutputCount { token: String, count: usize },

    /// Code must compile without errors (no output check)
    CompileOnly,

    /// Multiple criteria must all pass
    All { criteria: Vec<SuccessCriteria> },

    /// At least one criterion must pass
    Any { criteria: Vec<SuccessCriteria> },
}

#[cfg(feature = "compiler")]
impl SuccessCriteria {
    /// Validate execution output against criteria (only available with compiler feature)
    pub fn validate(&self, output: &ExecutionOutput) -> bool {
        match self {
            SuccessCriteria::ExactMatch { expected_stdout } => {
                output.stdout.trim() == expected_stdout.trim()
            }

            SuccessCriteria::RegexMatch { regex } => Regex::new(regex)
                .map(|re| re.is_match(&output.stdout))
                .unwrap_or(false),

            SuccessCriteria::OutputCount { token, count } => {
                output.stdout.matches(token).count() == *count
            }

            SuccessCriteria::CompileOnly => output.compile_success(),

            SuccessCriteria::All { criteria } => criteria.iter().all(|c| c.validate(output)),

            SuccessCriteria::Any { criteria } => criteria.iter().any(|c| c.validate(output)),
        }
    }
}

// ============================================================================
// New Test Suite Results (for function-based challenges)
// ============================================================================

/// Result of running a single test case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseResult {
    pub input: Vec<serde_json::Value>,
    pub expected: String,
    pub actual: String,
    pub passed: bool,
}

/// Result of running all (or sample) test cases for a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteResult {
    /// Whether all tests passed
    pub passed: bool,
    /// Total number of tests run
    pub total: usize,
    /// Number of tests that passed
    pub passed_count: usize,
    /// Individual test case results
    pub results: Vec<TestCaseResult>,
    /// Compilation error if any
    pub compilation_error: Option<String>,
}
