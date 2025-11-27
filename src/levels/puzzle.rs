use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PuzzleState {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}
