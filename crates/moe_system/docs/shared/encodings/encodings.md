```rust
//src/shared/encodings/encoded.rs
pub struct Encoded(pub Vec<f32>); // alias simple, change plus tard si besoin
```
```rust
//src/shared/encodings/encoder_trait.rs
pub trait Encoder: Send + Sync {
    fn encode(&self, x: &InputData) -> MoeResult<Encoded>;
}
```