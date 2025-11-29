#[cfg(feature = "compiler")]
use crate::compiler::ExecutionOutput;
#[cfg(feature = "compiler")]
use regex::Regex;
use serde::{Deserialize, Serialize};

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
