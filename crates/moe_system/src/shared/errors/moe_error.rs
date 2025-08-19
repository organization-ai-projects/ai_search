#[derive(thiserror::Error, Debug)]
pub enum MoeError {
    #[error("deadline exceeded")]
    DeadlineExceeded,
    #[error("budget exceeded")]
    BudgetExceeded,
    #[error("encode error: {0}")]
    EncodeError(String),
    #[error("expert {name} failed: {cause}")]
    ExpertFailed { name: &'static str, cause: String },
    #[error("no expert selected")]
    NoExpertSelected,
    #[error("synthesis failed: {0}")]
    SynthesisFailed(String),
}
