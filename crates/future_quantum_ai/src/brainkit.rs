/// Verifier sensible au domaine : score = cosinus(y, anchor[ctx.domain])
pub struct AnchorVerifier {
    pub anchors: HashMap<String, Array1<f32>>,
}
impl Verifier for AnchorVerifier {
    fn score(&self, _req: &Request, y: &Array1<f32>, ctx: &Context) -> f32 {
        if let Some(anchor) = self.anchors.get(&ctx.domain) {
            let dot = y.dot(anchor);
            let norm_y = y.dot(y).sqrt();
            let norm_a = anchor.dot(anchor).sqrt();
            if norm_y > 0.0 && norm_a > 0.0 {
                dot / (norm_y * norm_a)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}
// --- OnlineLearner ciblé par domaine ---
pub struct DomainTargetingLearner;
impl OnlineLearner for DomainTargetingLearner {
    fn propose_delta(
        &self,
        repo: &Repo,
        commit_id: &CommitId,
        _req: &Request,
        _resp: &Response,
    ) -> Option<(ParamId, Delta)> {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        // Découverte dynamique des paramètres et shapes du commit courant
        let commit = match repo.get_commit(commit_id) {
            Ok(c) => c,
            Err(_) => return None,
        };
        let params: Vec<(ParamId, Shape2D)> = commit
            .params
            .iter()
            .filter_map(|(pid, pv)| match repo.get_blob(&pv.blob) {
                Ok(blob) => Some((pid.clone(), blob.shape.clone())),
                Err(_) => None,
            })
            .collect();
        if params.is_empty() {
            return None;
        }
        let (param, shape) = params.iter().choose(&mut rng).unwrap().clone();
        // Génère un delta non nul (LowRank ou Sparse) pour ce paramètre
        for _ in 0..5 {
            if rng.gen_bool(0.5) {
                // LowRank
                let r = rng.gen_range(1..=3);
                let u: Vec<f32> = (0..shape.out * r)
                    .map(|_| rng.gen_range(-1.0..1.0))
                    .collect();
                let v: Vec<f32> = (0..shape.inp * r)
                    .map(|_| rng.gen_range(-1.0..1.0))
                    .collect();
                if u.iter().any(|&x| x.abs() > 1e-6) || v.iter().any(|&x| x.abs() > 1e-6) {
                    return Some((
                        param.clone(),
                        Delta::LowRank {
                            r,
                            scale: rng.gen_range(0.5..1.5),
                            u,
                            v,
                            shape: shape.clone(),
                        },
                    ));
                }
            } else {
                // Sparse
                let n = rng.gen_range(1..=4);
                let mut idx = Vec::new();
                for _ in 0..n {
                    let i = rng.gen_range(0..shape.out);
                    let j = rng.gen_range(0..shape.inp);
                    let val: f32 = rng.gen_range(-2.0..2.0);
                    if val.abs() > 1e-6 {
                        idx.push((i, j, val));
                    }
                }
                if !idx.is_empty() {
                    return Some((
                        param.clone(),
                        Delta::Sparse {
                            idx,
                            shape: shape.clone(),
                        },
                    ));
                }
            }
        }
        None
    }
}
// brainkit.rs — single-file crate skeleton
// "Cerveau unique" avec versioning git‑like des poids, MLP réel,
// parallélisme de scénarios + sélection par vérifieur, hooks
// d'apprentissage en ligne et finetune. 100% Rust, ndarray, rayon.
//
// Build: cargo build --release
// ----------------------------------

use anyhow::{anyhow, Result};
use blake3::Hasher;
use ndarray::{s, Array1, Array2, Axis};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ===================== Types & Context =====================
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParamId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlobId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitId(pub String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Shape2D {
    pub out: usize,
    pub inp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub modality: String, // ex: "text" | "code" | "vision"
    pub domain: String,   // ex: "rust" | "qa" | "cv"
    pub constraints: Vec<String>,
    pub features: Vec<f32>, // vecteur de contexte (pour gating)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub x: Array1<f32>,
    pub ctx: Context,
    pub target: Option<Array1<f32>>, // Optionnel : cible pour scoring supervisé
}
// ===================== Gating avancé : diversité & limitation =====================
pub struct DiversityGater {
    pub max_candidates: usize,
}
impl Gater for DiversityGater {
    fn candidates(&self, ctx: &Context, commits: &[CommitId]) -> Vec<CommitId> {
        // Sélectionne max_candidates commits, en maximisant la diversité (distance cosine entre blobs)
        // Pour la démo, on prend les max_candidates premiers, mais on pourrait faire mieux (clustering, etc.)
        let mut v = commits.to_vec();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v.into_iter().take(self.max_candidates).collect()
    }
}
// ===================== Verifier avancé : scoring par rapport à une cible =====================
pub struct TargetVerifier;
impl Verifier for TargetVerifier {
    fn score(&self, req: &Request, y: &Array1<f32>, _ctx: &Context) -> f32 {
        if let Some(target) = &req.target {
            // Score = -distance euclidienne à la cible (plus c'est proche, mieux c'est)
            -((y - target).mapv(|v| v * v).sum()).sqrt()
        } else {
            // Fallback : -entropy(softmax(y))
            let sm = softmax(y);
            -entropy(&sm)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub y: Array1<f32>,
    pub commit_used: CommitId,
    pub score: f32,
}

// ===================== Version Store (git-like) =====================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    pub shape: Shape2D,
    pub data: Vec<f32>,
}
impl Blob {
    pub fn from_array(w: &Array2<f32>) -> Self {
        Self {
            shape: Shape2D {
                out: w.nrows(),
                inp: w.ncols(),
            },
            data: w.iter().copied().collect(),
        }
    }
    pub fn to_array(&self) -> Array2<f32> {
        Array2::from_shape_vec((self.shape.out, self.shape.inp), self.data.clone()).unwrap()
    }
    pub fn hash(&self) -> BlobId {
        let bytes = bincode::serialize(self).unwrap();
        let mut h = Hasher::new();
        h.update(&bytes);
        BlobId(h.finalize().to_hex().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Delta {
    LowRank {
        r: usize,
        scale: f32,
        u: Vec<f32>,
        v: Vec<f32>,
        shape: Shape2D,
    },
    Sparse {
        idx: Vec<(usize, usize, f32)>,
        shape: Shape2D,
    },
}
impl Delta {
    pub fn dense(&self) -> Array2<f32> {
        match self {
            Delta::LowRank {
                r,
                scale,
                u,
                v,
                shape,
            } => {
                let u = Array2::from_shape_vec((shape.out, *r), u.clone()).unwrap();
                let v = Array2::from_shape_vec((shape.inp, *r), v.clone()).unwrap();
                u.dot(&v.t()) * *scale
            }
            Delta::Sparse { idx, shape } => {
                let mut m = Array2::<f32>::zeros((shape.out, shape.inp));
                for &(i, j, val) in idx {
                    m[(i, j)] += val;
                }
                m
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamVersion {
    pub blob: BlobId,
    pub deltas: Vec<Delta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMeta {
    pub message: String,
    pub forked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: CommitId,
    pub parents: Vec<CommitId>,
    pub params: HashMap<ParamId, ParamVersion>,
    pub meta: CommitMeta,
}

#[derive(Default)]
pub struct Repo {
    pub blobs: HashMap<BlobId, Blob>,
    pub commits: HashMap<CommitId, Commit>,
}
impl Repo {
    /// Compacte/pack un paramètre d'un commit : compose le poids, crée un nouveau blob, vide les deltas.
    pub fn pack_param(&mut self, commit_id: &CommitId, param_id: &ParamId) -> Result<()> {
        let commit = self.get_commit(commit_id)?.clone();
        let mut params = commit.params.clone();
        let pv = params
            .get_mut(param_id)
            .ok_or_else(|| anyhow!("param missing"))?;
        let w = self.compose(commit_id, param_id)?;
        let new_blob = self.put_blob(Blob::from_array(&w));
        *pv = ParamVersion {
            blob: new_blob,
            deltas: vec![],
        };
        // Remplace le commit par une version packée (même id, même meta)
        let packed_commit = Commit {
            id: commit.id.clone(),
            parents: commit.parents.clone(),
            params,
            meta: commit.meta.clone(),
        };
        self.commits.insert(commit_id.clone(), packed_commit);
        Ok(())
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn put_blob(&mut self, blob: Blob) -> BlobId {
        let id = blob.hash();
        self.blobs.insert(id.clone(), blob);
        id
    }
    pub fn get_blob(&self, id: &BlobId) -> Result<&Blob> {
        self.blobs.get(id).ok_or_else(|| anyhow!("blob not found"))
    }
    pub fn add_commit(&mut self, c: Commit) -> CommitId {
        let id = c.id.clone();
        self.commits.insert(id.clone(), c);
        id
    }
    pub fn get_commit(&self, id: &CommitId) -> Result<&Commit> {
        self.commits
            .get(id)
            .ok_or_else(|| anyhow!("commit not found"))
    }
    pub fn compose(&self, cid: &CommitId, pid: &ParamId) -> Result<Array2<f32>> {
        let c = self.get_commit(cid)?;
        let pv = c.params.get(pid).ok_or_else(|| anyhow!("param missing"))?;
        let mut w = self.get_blob(&pv.blob)?.to_array();
        for d in &pv.deltas {
            w = w + &d.dense();
        }
        Ok(w)
    }
    pub fn genesis(&mut self, mapping: HashMap<ParamId, BlobId>, msg: &str) -> CommitId {
        let params = mapping
            .into_iter()
            .map(|(pid, bid)| {
                (
                    pid,
                    ParamVersion {
                        blob: bid,
                        deltas: vec![],
                    },
                )
            })
            .collect();
        let id = CommitId(format!("C{}", self.commits.len()));
        self.add_commit(Commit {
            id: id.clone(),
            parents: vec![],
            params,
            meta: CommitMeta {
                message: msg.to_string(),
                forked: false,
            },
        })
    }
    pub fn derive_with_delta(
        &mut self,
        parent: &CommitId,
        pid: &ParamId,
        delta: Delta,
        cos_conflict: f32,
        msg: &str,
    ) -> Result<CommitId> {
        const DMAX: usize = 8;
        let p = self.get_commit(parent)?.clone();
        let mut params = p.params.clone();
        let pv = params
            .get_mut(pid)
            .ok_or_else(|| anyhow!("param missing"))?;
        // Vérifie si le delta est effectif (somme absolue nulle)
        let delta_dense = delta.dense();
        let delta_sum: f32 = delta_dense.iter().map(|v| v.abs()).sum();
        if delta_sum == 0.0 {
            // Pas de commit si le delta n'apporte rien
            return Ok(parent.clone());
        }
        // Limite DMAX : refuse d'ajouter plus de DMAX deltas
        if pv.deltas.len() >= DMAX {
            // Signal pour pack/compactage (à gérer dans le runner)
            return Err(anyhow!("DMAX reached for param: {}", pid.0));
        }
        // Simule l'ajout du delta pour vérifier si le param effectif change vraiment
        let mut pv_sim = pv.clone();
        pv_sim.deltas.push(delta.clone());
        let w_parent = self.compose(parent, pid)?;
        let w_new = {
            let mut w = self.get_blob(&pv_sim.blob)?.to_array();
            for d in &pv_sim.deltas {
                w = w + &d.dense();
            }
            w
        };
        // Tolérance numérique fine : commit uniquement si au moins un élément diffère de plus de tol
        let tol = 1e-7;
        let changed = w_parent
            .iter()
            .zip(w_new.iter())
            .any(|(a, b)| (a - b).abs() > tol);
        if !changed {
            // Pas de commit si aucun élément n'a changé significativement
            return Ok(parent.clone());
        }
        // Politique anti-fork :
        // - forked=true seulement si cos_conflit < -0.3 ET qu'il y a déjà ≥1 delta conflictuel récent
        // - sinon, append le delta au même commit (pas de nouveau fork)
        let mut conflict = false;
        if !pv.deltas.is_empty() {
            let base_shape = self.get_blob(&pv.blob)?.shape;
            let mut acc = Array2::<f32>::zeros((base_shape.out, base_shape.inp));
            for d in &pv.deltas {
                acc = acc + &d.dense();
            }
            // Conflit si le delta est opposé ou très différent
            if cosine_2d(&acc.view(), &delta_dense.view()) < -0.3 {
                if pv.deltas.len() >= 1 {
                    conflict = true;
                }
            }
            // Conflit si overlap non nul
            let overlap: f32 = acc
                .iter()
                .zip(delta_dense.iter())
                .map(|(a, b)| if *a != 0.0 && *b != 0.0 { 1.0 } else { 0.0 })
                .sum();
            if overlap > 0.0 && pv.deltas.len() >= 1 {
                conflict = true;
            }
        }
        // Toujours créer un nouveau commit (même sans conflit), mais forked=true uniquement si conflit
        let id = CommitId(format!("C{}", self.commits.len()));
        let child = Commit {
            id: id.clone(),
            parents: vec![parent.clone()],
            params,
            meta: CommitMeta {
                message: msg.to_string(),
                forked: conflict,
            },
        };
        Ok(self.add_commit(child))
    }
}

// ===================== NN: MLP (2 couches, biais intégrés) =====================
#[derive(Clone, Debug)]
pub struct MlpSpec {
    pub d_in: usize,
    pub d_hidden: usize,
    pub d_out: usize,
}
fn append_bias(x: &Array1<f32>) -> Array1<f32> {
    let mut y = Array1::<f32>::zeros(x.len() + 1);
    y.slice_mut(s![..x.len()]).assign(x);
    y[x.len()] = 1.0;
    y
}
fn relu(v: &Array1<f32>) -> Array1<f32> {
    v.map(|&t| if t > 0.0 { t } else { 0.0 })
}
fn layer_forward(
    repo: &Repo,
    cid: &CommitId,
    pid: &ParamId,
    x: &Array1<f32>,
) -> Result<Array1<f32>> {
    let w = repo.compose(cid, pid)?;
    let xext = append_bias(x);
    Ok(w.dot(&xext))
}
pub fn mlp_forward(
    repo: &Repo,
    cid: &CommitId,
    spec: &MlpSpec,
    x: &Array1<f32>,
) -> Result<Array1<f32>> {
    let h = relu(&layer_forward(repo, cid, &ParamId("layer0.Wb".into()), x)?);
    layer_forward(repo, cid, &ParamId("layer1.Wb".into()), &h)
}

// ===================== Gating & Scénarios parallèles =====================
pub trait Gater: Send + Sync {
    fn candidates(&self, ctx: &Context, commits: &[CommitId]) -> Vec<CommitId>;
}
pub struct SimpleGater;
impl Gater for SimpleGater {
    fn candidates(&self, ctx: &Context, commits: &[CommitId]) -> Vec<CommitId> {
        let mut v = commits.to_vec();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v.into_iter().take(2).collect()
    }
}

pub trait Verifier: Send + Sync {
    fn score(&self, req: &Request, y: &Array1<f32>, ctx: &Context) -> f32;
}
pub struct CompositeVerifier;
impl Verifier for CompositeVerifier {
    fn score(&self, _req: &Request, y: &Array1<f32>, _ctx: &Context) -> f32 {
        // conf = -entropy(softmax(y)) as a simple proxy
        let sm = softmax(y);
        -entropy(&sm)
    }
}

fn softmax(x: &Array1<f32>) -> Array1<f32> {
    let max = x.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = x.iter().map(|v| (v - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    Array1::from(
        exps.into_iter()
            .map(|e| e / sum.max(1e-9))
            .collect::<Vec<_>>(),
    )
}
fn entropy(p: &Array1<f32>) -> f32 {
    -p.iter()
        .map(|&pi| if pi > 0.0 { pi * (pi + 1e-9).ln() } else { 0.0 })
        .sum::<f32>()
}

pub struct Runner<'a> {
    pub repo: &'a Repo,
    pub spec: MlpSpec,
    pub gater: Box<dyn Gater>,
    pub verifier: Box<dyn Verifier>,
}
impl<'a> Runner<'a> {
    pub fn run(&self, req: &Request, available_commits: &[CommitId]) -> Result<Response> {
        let cand = self.gater.candidates(&req.ctx, available_commits);
        // Parallélisme des scénarios (premier passage)
        let mut scored: Vec<(CommitId, Array1<f32>, f32)> = cand
            .par_iter()
            .map(|cid| {
                let y = mlp_forward(self.repo, cid, &self.spec, &req.x).unwrap();
                let s = self.verifier.score(req, &y, &req.ctx);
                (cid.clone(), y, s)
            })
            .collect();
        // Trie par score décroissant
        scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        // Affichage explicite des branches/scénarios et de leur score
        println!("Scénarios explorés :");
        for (cid, _, s) in &scored {
            println!("  - Commit {} : score {:.4}", cid.0, s);
        }
        // Budget p-parallèle : si ambiguïté, réévalue top-2, sinon ne garde que le best
        if scored.len() >= 2 && (scored[0].2 - scored[1].2).abs() < 0.02 {
            let top2: Vec<_> = scored
                .iter()
                .take(2)
                .map(|(cid, _, _)| cid.clone())
                .collect();
            let rescored: Vec<(CommitId, Array1<f32>, f32)> = top2
                .par_iter()
                .map(|cid| {
                    let y = mlp_forward(self.repo, cid, &self.spec, &req.x).unwrap();
                    let s = self.verifier.score(req, &y, &req.ctx);
                    (cid.clone(), y, s)
                })
                .collect();
            let (best_cid, best_y, best_s) = rescored
                .into_iter()
                .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
                .ok_or_else(|| anyhow!("no candidates (top2)"))?;
            return Ok(Response {
                y: best_y,
                commit_used: best_cid,
                score: best_s,
            });
        }
        // Sinon, ne garde que le best
        let (best_cid, best_y, best_s) = scored
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("no candidates"))?;
        Ok(Response {
            y: best_y,
            commit_used: best_cid,
            score: best_s,
        })
    }
}

// ===================== Online learning & Finetune (hooks) =====================
pub trait OnlineLearner: Send + Sync {
    /// Observe une interaction et propose un Delta (ou None).
    fn propose_delta(
        &self,
        repo: &Repo,
        commit_id: &CommitId,
        req: &Request,
        resp: &Response,
    ) -> Option<(ParamId, Delta)>;
}

pub struct NoopLearner;
impl OnlineLearner for NoopLearner {
    fn propose_delta(
        &self,
        _: &Repo,
        _: &CommitId,
        _: &Request,
        _: &Response,
    ) -> Option<(ParamId, Delta)> {
        None
    }
}

pub trait FineTuner: Send + Sync {
    fn consolidate(&self, repo: &mut Repo, base: &CommitId, commits: &[CommitId]) -> Result<()>;
}
pub struct NoopFineTuner;
impl FineTuner for NoopFineTuner {
    fn consolidate(&self, _: &mut Repo, _: &CommitId, _: &[CommitId]) -> Result<()> {
        Ok(())
    }
}

// ===================== Utils =====================
fn cosine_2d(a: &ndarray::ArrayView2<f32>, b: &ndarray::ArrayView2<f32>) -> f32 {
    let a1: Array1<f32> = a.iter().copied().collect();
    let b1: Array1<f32> = b.iter().copied().collect();
    let na = (a1.dot(&a1)).sqrt();
    let nb = (b1.dot(&b1)).sqrt();
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        a1.dot(&b1) / (na * nb)
    }
}

// ===================== Bootstrap helper (optionnel) =====================
#[allow(dead_code)]
pub fn bootstrap_minimal_repo(spec: &MlpSpec, seed: u64) -> (Repo, CommitId) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut rand_mat =
        |o, i, s| Array2::from_shape_fn((o, i), |_| (rng.gen::<f32>() - 0.5) * 2.0 * s);
    let w0 = rand_mat(spec.d_hidden, spec.d_in + 1, 0.02);
    let w1 = rand_mat(spec.d_out, spec.d_hidden + 1, 0.02);
    let mut repo = Repo::new();
    let b0 = repo.put_blob(Blob::from_array(&w0));
    let b1 = repo.put_blob(Blob::from_array(&w1));
    let mut map = HashMap::new();
    map.insert(ParamId("layer0.Wb".into()), b0);
    map.insert(ParamId("layer1.Wb".into()), b1);
    let c0 = repo.genesis(map, "genesis");
    (repo, c0)
}

// Note: pas de main(); ce fichier est un *crate* réutilisable.
// Intégration: crée un binaire séparé qui importe brainkit, bootstrap le repo,
// enregistre/charge commits, et branche un OnlineLearner concret.
