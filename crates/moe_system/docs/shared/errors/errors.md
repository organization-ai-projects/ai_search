### 2.2 Gestion d'erreur normalisée
```rust
//src/shared/errors/moe_result.rs
pub type MoeResult<T> = Result<T, MoeError>;
```

```rust
//src/shared/errors/moe_error.rs
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
```