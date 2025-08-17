```rust
//src/shared/gatings/gate_scores.rs
#[derive(Clone, Debug)]
pub struct GateScores {
    // scores/softmax par expert_id
    pub logits: Vec<(ExpertId, f32)>,
}
```

```rust
//src/shared/gatings/gating_policy.rs
/// Politique de gating standard (softmax stable, tie-break, epsilon-greedy)
#[derive(Clone, Debug)]
pub struct GatingPolicy {
    pub temperature: f32,  // >0
    pub epsilon: f32,      // 0..1, proba d’exploration min
    pub seed: u64,         // pour tie-break stable
}

pub fn softmax_stable(xs: &[f32], temp: f32) -> Vec<f32> {
    let invt = 1.0_f32 / temp.max(1e-6);
    let m = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = xs.iter().map(|&x| ((x - m) * invt).exp()).collect();
    let sum = exps.iter().sum::<f32>().max(1e-12);
    exps.into_iter().map(|e| e / sum).collect()
}

/// Applique softmax + epsilon, puis tri avec tie-break stable.
pub fn gate_with_policy(
    scores: &[(ExpertId, f32)],
    pol: &GatingPolicy
) -> Vec<(ExpertId, f32)> {
    let ws = softmax_stable(&scores.iter().map(|(_, s)| *s).collect::<Vec<_>>(), pol.temperature);
    let mut v: Vec<(ExpertId, f32)> = scores.iter().zip(ws).map(|((id,_), w)| (*id, (1.0 - pol.epsilon)*w + pol.epsilon/(scores.len() as f32))).collect();
    // tri stable par poids puis par hash(id, seed)
    v.sort_by(|(a,wa), (b,wb)| {
        wb.partial_cmp(wa).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                let ha = a.0 ^ pol.seed;
                let hb = b.0 ^ pol.seed;
                ha.cmp(&hb)
            })
    });
    v
}
```