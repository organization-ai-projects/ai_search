```rust
//src/shared/synthesizers/synthesis_metadata.rs
#[derive(Clone, Debug)]
pub struct SynthesisMetadata {
    pub entropy: f32,
    pub topk: usize,
    pub lat_total_ms: u64,
    pub drop_count: usize,
    pub util_by_expert: Vec<(ExpertId, f32)>, // pour load-balance
}
```
```rust
//src/shared/synthesizers/synthesis_result.rs
#[derive(Clone, Debug)]
pub struct SynthesisResult {
    /// Sortie unique après synthèse locale du routeur
    pub value: Value,
    /// Métadonnées de synthèse / télémétrie
    pub synthesis_metadata: SynthesisMetadata,
}
```

### 2.5 Stratégie de synthèse indépendante du router (injection de politique)
```rust
//src/shared/synthesizers/synthesizer_trait.rs
pub trait Synthesizer<C, S>: Send + Sync {
    fn id(&self) -> &'static str;
    fn synthesize(
        &self,
        calls: &[C],
        scores: &GateScores,
    ) -> MoeResult<S>;
}
```