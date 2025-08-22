// --- OnlineLearner ciblé par domaine ---
use brainkit::DomainTargetingLearner;
// src/main.rs — runner minimal pour le crate "brainkit" (voir brainkit.rs)
// Build: cargo run --release

mod brainkit; // placez le fichier brainkit.rs dans src/ au même niveau
mod print_utils;

use anyhow::Result;
use brainkit::*;
use ndarray::{Array1, Array2};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

fn rand_mat(rng: &mut StdRng, out: usize, inp: usize, scale: f32) -> Array2<f32> {
    Array2::from_shape_fn((out, inp), |_| (rng.gen::<f32>() - 0.5) * 2.0 * scale)
}

fn main() -> Result<()> {
    // 1) Spéc réseau (MLP 2 couches)
    let spec = MlpSpec {
        d_in: 16,
        d_hidden: 24,
        d_out: 8,
    };

    // 2) Cerveau initial (genesis commit)
    let (mut repo, c0) = bootstrap_minimal_repo(&spec, 123);

    // 3) Crée 2 commits dérivés (domaines A et B), sans écraser C0
    let mut rng = StdRng::seed_from_u64(999);

    // 3a) Delta low‑rank sur layer0 pour domaine A
    let r = 2usize;
    let u0 = rand_mat(&mut rng, spec.d_hidden, r, 0.08);
    let v0 = rand_mat(&mut rng, spec.d_in + 1, r, 0.08);
    let d0 = Delta::LowRank {
        r,
        scale: 1.0,
        u: u0.iter().copied().collect(),
        v: v0.iter().copied().collect(),
        shape: Shape2D {
            out: spec.d_hidden,
            inp: spec.d_in + 1,
        },
    };
    let c1 = repo.derive_with_delta(
        &c0,
        &ParamId("layer0.Wb".into()),
        d0,
        0.2,
        "domain A (layer0)",
    )?;

    // 3b) Delta sparse sur layer1 pour domaine B
    let mut idx = Vec::new();
    for _ in 0..12 {
        // 12 entrées modifiées
        let i = rng.gen_range(0..spec.d_out);
        let j = rng.gen_range(0..spec.d_hidden + 1);
        let val = (rng.gen::<f32>() - 0.5) * 0.5;
        idx.push((i, j, val));
    }
    let d1 = Delta::Sparse {
        idx,
        shape: Shape2D {
            out: spec.d_out,
            inp: spec.d_hidden + 1,
        },
    };
    let c2 = repo.derive_with_delta(
        &c0,
        &ParamId("layer1.Wb".into()),
        d1,
        0.2,
        "domain B (layer1)",
    )?;

    // 4) Pool de branches/scénarios
    let mut pool = vec![c0.clone(), c1.clone(), c2.clone()];

    // 5) Deux requêtes avec contextes différents et cibles (pour scoring supervisé)
    let x_a = Array1::from_vec(
        (0..spec.d_in)
            .map(|k| (k as f32) / spec.d_in as f32)
            .collect(),
    );
    let target_a = Array1::from_elem(spec.d_out, 0.5); // cible arbitraire
    let req_a = Request {
        x: x_a,
        ctx: Context {
            modality: "code".into(),
            domain: "rust".into(),
            constraints: vec!["no_unsafe".into()],
            features: vec![1.0, 0.0, 0.0],
        },
        target: Some(target_a),
    };

    let x_b = Array1::from_vec(
        (0..spec.d_in)
            .map(|k| ((spec.d_in - k) as f32) / spec.d_in as f32)
            .collect(),
    );
    let target_b = Array1::from_elem(spec.d_out, -0.5); // autre cible arbitraire
    let req_b = Request {
        x: x_b,
        ctx: Context {
            modality: "code".into(),
            domain: "python".into(),
            constraints: vec!["vectorize".into()],
            features: vec![0.0, 1.0, 0.0],
        },
        target: Some(target_b),
    };

    // --- Boucle d'entraînement temps réel ---
    let learner = DomainTargetingLearner;
    let max_branches = 4; // Limite dynamique du nombre de branches explorées
                          // Définition des ancres pour les domaines "A" et "B"
    let mut anchors = HashMap::new();
    anchors.insert("rust".to_string(), Array1::from_elem(spec.d_out, 1.0));
    anchors.insert("python".to_string(), Array1::from_elem(spec.d_out, -1.0));

    for step in 0..10 {
        let gater = DiversityGater {
            max_candidates: max_branches,
        };
        // Verifier sensible au domaine
        let verifier = AnchorVerifier {
            anchors: anchors.clone(),
        };
        let runner = Runner {
            repo: &repo,
            spec: spec.clone(),
            gater: Box::new(gater),
            verifier: Box::new(verifier),
        };
        // Alterne entre deux contextes
        let (req, label) = if step % 2 == 0 {
            (&req_a, "A")
        } else {
            (&req_b, "B")
        };
        let resp = runner.run(req, &pool)?;
        println!(
            "Step {step} | {label} commit_used={} score={:.3} y[0]={:.3}",
            (resp.commit_used).0,
            resp.score,
            resp.y[0]
        );
        if let Some((pid, delta)) = learner.propose_delta(&repo, &resp.commit_used, req, &resp) {
            let commit_result = repo.derive_with_delta(
                &resp.commit_used,
                &pid,
                delta.clone(),
                0.2,
                &format!("online step {step}"),
            );
            let new_commit = match commit_result {
                Ok(cid) => cid,
                Err(e) => {
                    if e.to_string().contains("DMAX reached") {
                        println!(
                            "[Pack] Compactage du paramètre {} sur commit {}",
                            pid.0, resp.commit_used.0
                        );
                        repo.pack_param(&resp.commit_used, &pid)?;
                        repo.derive_with_delta(
                            &resp.commit_used,
                            &pid,
                            delta,
                            0.2,
                            &format!("online step {step} (après pack)"),
                        )?
                    } else {
                        return Err(e);
                    }
                }
            };
            pool.push(new_commit);
            if pool.len() > max_branches {
                pool.remove(0);
            }
        }
        if step > 0 && step % 3 == 0 {
            for cid in &pool {
                for pid in [ParamId("layer0.Wb".into()), ParamId("layer1.Wb".into())].iter() {
                    let _ = repo.pack_param(cid, pid);
                }
            }
        }
    }
    // Affichage de l'historique des commits et des deltas
    print_utils::print_commit_history(&repo);
    Ok(())
}
