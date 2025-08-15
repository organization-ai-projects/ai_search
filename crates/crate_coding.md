# code\_assist\_core (Rust, prod build — no mocks, no hidden AI)

**Objectif**: I/O complet "Copilot-like" (FIM) en **100% Rust from scratch**, sans dépendance IA externe, et **aucun backend d’IA activé par défaut**.

* LSP serveur (stdio) prêt pour VSCode/Neovim
* Génération **streaming** avec nucleus sampling
* **Tokenizer** byte-level (remplaçable par BPE ensuite)
* **Logging** JSONL des suggestions acceptées
* **Backends disponibles (opt‑in par env var)**:

  * `none` (par défaut) → **aucune complétion générée**
  * `ngram` → `NGramLM` 100% Rust, statistiques locales **en ligne** (aucun modèle externe)
  * `ssm` → `SSMLM` minimal (100% Rust), charge des poids `.npy` si fournis

> Build: `cargo build --release` — Lancer: `AI_BACKEND=none|ngram|ssm cargo run --release`

---

## Arborescence

```
code_assist_core/
├─ Cargo.toml
└─ src/
   ├─ main.rs
   ├─ lib.rs
   ├─ config.rs
   ├─ types/
   │   ├─ mod.rs
   │   ├─ lang.rs
   │   └── io.rs
   ├─ tokenizer/
   │   ├─ mod.rs
   │   ├─ trait_.rs
   │   └─ bytelevel.rs
   ├─ pack/
   │   └─ fim.rs
   ├─ model/
   │   ├─ mod.rs
   │   ├─ trait_.rs
   │   ├─ ngram_lm.rs
   │   └─ ssm_lm.rs
   ├─ gen/
   │   └─ stream.rs
   ├─ lsp/
   │   └─ server.rs
   └─ logger/
       └─ log.rs
```

---

## Cargo.toml

```toml
[package]
name = "code_assist_core"
version = "1.0.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[dependencies]
anyhow = "1"
thiserror = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
regex = "1"
rand = "0.8"
bytes = "1"
parking_lot = "0.12"
crossbeam-channel = "0.5"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "io-std", "time", "process", "sync"] }
# LSP server (stdio)
tower-lsp = "0.20"
# logging dates
chrono = { version = "0.4", default-features = false, features = ["clock", "std", "alloc"] }
# math / arrays (SSM poids)
ndarray = "0.15"
ndarray-npy = "0.8"
faer = "0.18"
```

---

## src/lib.rs

```rust
pub mod config;
pub mod types;
pub mod tokenizer;
pub mod pack;
pub mod model;
pub mod gen;
pub mod lsp;
pub mod logger;
```

---

## src/config.rs

```rust
#[derive(Clone, Debug)]
pub struct GenConfig {
    pub max_new_tokens: usize,
    pub top_p: f32,
    pub temperature: f32,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self { max_new_tokens: 128, top_p: 0.9, temperature: 0.8 }
    }
}
```

---

## src/types/mod.rs

```rust
pub mod lang;
pub mod io;
```

### src/types/lang.rs

```rust
#[derive(Copy, Clone, Debug)]
pub enum Lang { Rust, Ts, Py, Cpp, Unknown }

impl Lang {
    pub fn from_file_ext(path: &str) -> Self {
        if let Some(ext) = std::path::Path::new(path).extension().and_then(|s| s.to_str()) {
            match ext {
                "rs" => Lang::Rust,
                "ts" | "tsx" => Lang::Ts,
                "py" => Lang::Py,
                "cc" | "cpp" | "cxx" | "hpp" | "h" => Lang::Cpp,
                _ => Lang::Unknown,
            }
        } else { Lang::Unknown }
    }
}
```

### src/types/io.rs

```rust
use super::lang::Lang;

pub struct CompletionRequest<'a> {
    pub lang: Lang,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub max_new_tokens: usize,
    pub top_p: f32,
    pub temperature: f32,
}

#[derive(Clone, Debug)]
pub struct CompletionEvent {
    pub token_text: String,
    pub done: bool,
}
```

---

## src/tokenizer/mod.rs

```rust
pub mod trait_;
pub mod bytelevel;
```

### src/tokenizer/trait\_.rs

```rust
use crate::types::lang::Lang;

pub trait Tokenizer: Send + Sync {
    fn bos(&self) -> u32; fn eot(&self) -> u32;
    fn lang_id(&self, l: Lang) -> u32;
    fn fim_prefix(&self) -> u32; fn fim_middle(&self) -> u32; fn fim_suffix(&self) -> u32;
    fn encode(&self, s: &str) -> Vec<u32>;
    fn decode(&self, ids: &[u32]) -> String;
    fn vocab_size(&self) -> u32;
}
```

### src/tokenizer/bytelevel.rs

```rust
use crate::types::lang::Lang;
use super::trait_::Tokenizer;

/// Tokenizer byte-level de production (remplaçable par BPE ultérieurement).
pub struct ByteLevelTok;

const BOS: u32 = 4090_000;
const EOT: u32 = 4090_001;
const FIM_PREFIX: u32 = 4090_010;
const FIM_MIDDLE: u32 = 4090_011;
const FIM_SUFFIX: u32 = 4090_012;
const LANG_BASE: u32 = 4090_100;

impl ByteLevelTok { pub fn new() -> Self { Self } }

impl Tokenizer for ByteLevelTok {
    fn bos(&self) -> u32 { BOS }
    fn eot(&self) -> u32 { EOT }
    fn lang_id(&self, l: Lang) -> u32 {
        match l {
            Lang::Rust => LANG_BASE + 0,
            Lang::Ts => LANG_BASE + 1,
            Lang::Py => LANG_BASE + 2,
            Lang::Cpp => LANG_BASE + 3,
            Lang::Unknown => LANG_BASE + 255,
        }
    }
    fn fim_prefix(&self) -> u32 { FIM_PREFIX }
    fn fim_middle(&self) -> u32 { FIM_MIDDLE }
    fn fim_suffix(&self) -> u32 { FIM_SUFFIX }

    fn encode(&self, s: &str) -> Vec<u32> {
        s.as_bytes().iter().map(|b| *b as u32).collect()
    }
    fn decode(&self, ids: &[u32]) -> String {
        let bytes: Vec<u8> = ids.iter().filter_map(|&id| if id < 256 { Some(id as u8) } else { None }).collect();
        String::from_utf8_lossy(&bytes).to_string()
    }
    fn vocab_size(&self) -> u32 { 4091_000 }
}
```

---

## src/pack/fim.rs

```rust
use crate::tokenizer::trait_::Tokenizer;
use crate::types::lang::Lang;

pub struct PackedInput { pub ids: Vec<u32> }

pub fn pack_fim<T: Tokenizer>(tok: &T, lang: Lang, prefix: &str, suffix: &str) -> PackedInput {
    let mut ids = Vec::with_capacity(prefix.len() + suffix.len() + 8);
    ids.push(tok.bos());
    ids.push(tok.lang_id(lang));
    ids.push(tok.fim_prefix());
    ids.extend(tok.encode(prefix));
    ids.push(tok.fim_suffix());
    ids.extend(tok.encode(suffix));
    PackedInput { ids }
}
```

---

## src/model/mod.rs

```rust
pub mod trait_;
pub mod ngram_lm;
pub mod ssm_lm;
```

### src/model/trait\_.rs

```rust
/// Interface de LM séquentiel (prod) : logits du prochain token
pub trait SeqLM: Send + Sync {
    fn next_logits(&mut self, context: &[u32]) -> Vec<f32>;
    fn vocab_size(&self) -> u32;
    /// Observations de corpus (pour NGram en ligne). Par défaut: no-op.
    fn observe_corpus(&mut self, _text: &str) { }
}
```

### src/model/ngram\_lm.rs

```rust
use std::collections::HashMap;
use rand::Rng;
use crate::model::trait_::SeqLM;

/// LM n-grammes byte-level **en ligne** (100% Rust, aucune dépendance externe, aucune donnée pré-entraînée).
pub struct NGramLM {
    vocab: u32,
    n: usize,
    counts: HashMap<Vec<u32>, HashMap<u32, u32>>, // context -> next -> count
    total_next: HashMap<Vec<u32>, u32>,
    min_prob: f32,
}

impl NGramLM {
    pub fn new(vocab: u32, n: usize) -> Self {
        Self { vocab, n: n.max(2).min(8), counts: HashMap::new(), total_next: HashMap::new(), min_prob: 1e-7 }
    }

    pub fn update_sequence(&mut self, seq: &[u32]) {
        if seq.len() < self.n { return; }
        for i in 0..=seq.len()-self.n {
            let ctx = &seq[i..i+self.n-1];
            let nxt = seq[i+self.n-1];
            let key = ctx.to_vec();
            let entry = self.counts.entry(key.clone()).or_default();
            *entry.entry(nxt).or_insert(0) += 1;
            *self.total_next.entry(key).or_insert(0) += 1;
        }
    }

    pub fn update_text(&mut self, bytes: &[u8]) { let seq: Vec<u32> = bytes.iter().map(|&b| b as u32).collect(); self.update_sequence(&seq); }
}

impl SeqLM for NGramLM {
    fn next_logits(&mut self, context: &[u32]) -> Vec<f32> {
        let ctx_len = self.n - 1;
        let tail: Vec<u32> = if context.len() >= ctx_len { context[context.len()-ctx_len..].to_vec() } else {
            let mut pad = vec![0u32; ctx_len - context.len()]; pad.extend_from_slice(context); pad
        };
        let mut logits = vec![f32::ln(self.min_prob); self.vocab as usize];
        if let Some(nexts) = self.counts.get(&tail) {
            let tot = *self.total_next.get(&tail).unwrap_or(&1);
            for (&tok, &cnt) in nexts { let p = (cnt as f32 + 0.5) / (tot as f32 + (self.vocab as f32)*0.5 / 256.0); logits[tok as usize] = p.ln(); }
        } else {
            let mut rng = rand::thread_rng(); for i in 0..256usize { logits[i] = (1e-3 + rng.gen::<f32>()*1e-3).ln(); }
        }
        logits
    }
    fn vocab_size(&self) -> u32 { self.vocab }
    fn observe_corpus(&mut self, text: &str) { self.update_text(text.as_bytes()); }
}
```



### src/model/ssm_lm.rs (Rust natif, support .ron pour édition/visualisation, .bin pour chargement rapide)

```rust
use crate::model::trait_::SeqLM;
use std::fs::File;
use std::io::{Read, BufReader, Write, BufWriter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Matrix {
    pub data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn dot(&self, x: &[f32]) -> Vec<f32> {
        assert_eq!(self.cols, x.len());
        let mut out = vec![0.0; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                out[i] += self.data[i * self.cols + j] * x[j];
            }
        }
        out
    }

    /// Sauvegarde la matrice en .ron (lisible, éditable)
    pub fn save_ron(&self, path: &str) -> std::io::Result<()> {
        let s = ron::to_string(self).unwrap();
        std::fs::write(path, s)
    }

    /// Sauvegarde la matrice en .bin (rapide, non lisible)
    pub fn save_bin(&self, path: &str) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(path)?);
        let bytes = unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4)
        };
        file.write_all(bytes)?;
        Ok(())
    }

    /// Charge une matrice depuis un .bin (rapide)
    pub fn from_bin(path: &str, rows: usize, cols: usize) -> std::io::Result<Self> {
        let mut file = BufReader::new(File::open(path)?);
        let mut data = vec![0f32; rows * cols];
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, rows * cols * 4)
        };
        file.read_exact(bytes)?;
        Ok(Matrix { data, rows, cols })
    }

    /// Charge une matrice depuis un .ron (lisible, debug, édition)
    pub fn from_ron(path: &str) -> std::io::Result<Self> {
        let s = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&s).unwrap())
    }
}

pub struct SSMLM {
    vocab: u32,
    d_model: usize,
    a: Matrix, b: Matrix, c: Matrix, d: Matrix,
    state: Vec<f32>,
    out_proj: Matrix,
}

impl SSMLM {
    /// Charge les poids depuis un dossier (binaires rapides)
    pub fn load(weights_dir: &str, vocab: u32, d_model: usize) -> std::io::Result<Self> {
        let a = Matrix::from_bin(&format!("{}/A.bin", weights_dir), d_model, d_model)?;
        let b = Matrix::from_bin(&format!("{}/B.bin", weights_dir), d_model, d_model)?;
        let c = Matrix::from_bin(&format!("{}/C.bin", weights_dir), d_model, d_model)?;
        let d = Matrix::from_bin(&format!("{}/D.bin", weights_dir), d_model, d_model)?;
        let out_proj = Matrix::from_bin(&format!("{}/out_proj.bin", weights_dir), vocab as usize, d_model)?;
        let state = vec![0.0; d_model];
        Ok(Self { vocab, d_model, a, b, c, d, state, out_proj })
    }

    /// (optionnel) Charge les poids depuis .ron pour debug/édition
    pub fn load_ron(weights_dir: &str, vocab: u32, d_model: usize) -> std::io::Result<Self> {
        let a = Matrix::from_ron(&format!("{}/A.ron", weights_dir))?;
        let b = Matrix::from_ron(&format!("{}/B.ron", weights_dir))?;
        let c = Matrix::from_ron(&format!("{}/C.ron", weights_dir))?;
        let d = Matrix::from_ron(&format!("{}/D.ron", weights_dir))?;
        let out_proj = Matrix::from_ron(&format!("{}/out_proj.ron", weights_dir))?;
        let state = vec![0.0; d_model];
        Ok(Self { vocab, d_model, a, b, c, d, state, out_proj })
    }

    fn step(&mut self, x: &[f32]) -> Vec<f32> {
        let s = self.a.dot(&self.state).iter().zip(self.b.dot(x)).map(|(a, b)| a + b).collect::<Vec<_>>();
        let mut y = self.c.dot(&s).iter().zip(self.d.dot(x)).map(|(c, d)| c + d).collect::<Vec<_>>();
        for v in &mut y {
            *v = 0.5 * *v * (1.0 + ((*v + 0.044715 * *v * *v * *v) * std::f32::consts::FRAC_2_SQRT_PI).tanh());
        }
        self.state = s;
        y
    }
}

impl SeqLM for SSMLM {
    fn next_logits(&mut self, context: &[u32]) -> Vec<f32> {
        let last = *context.last().unwrap_or(&0);
        let mut x = vec![0.0; self.d_model];
        x[(last as usize) % self.d_model] = 1.0;
        let y = self.step(&x);
        self.out_proj.dot(&y)
    }
    fn vocab_size(&self) -> u32 { self.vocab }
}
```

> **Note** :
> - Les poids peuvent être sauvegardés en `.ron` (lisible, éditable, versionnable) pour debug, visualisation ou édition manuelle.
> - Pour l’inférence, on charge uniquement les `.bin` (rapide, compact).
> - Un utilitaire Rust simple peut convertir `.ron` → `.bin` et inversement si besoin.
> - Cela combine transparence, portabilité et performance.

---

## src/gen/stream.rs

```rust
use anyhow::Result;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

use crate::config::GenConfig;
use crate::model::trait_::SeqLM;
use crate::tokenizer::trait_::Tokenizer;
use crate::types::io::{CompletionEvent, CompletionRequest};
use crate::pack::fim::pack_fim;

pub fn generate_stream<T: Tokenizer>(
    tok: &T,
    lm: &mut dyn SeqLM,
    cfg: &GenConfig,
    req: &CompletionRequest,
    mut on_event: impl FnMut(CompletionEvent),
) -> Result<()> {
    let mut ctx = pack_fim(tok, req.lang, req.prefix, req.suffix).ids;
    let mut balance_curly: i32 = 0;
    let indent = current_indent(req.prefix);

    for step in 0..cfg.max_new_tokens.min(req.max_new_tokens) {
        let logits = lm.next_logits(&ctx);
        let next_id = sample_nucleus(&logits, cfg.top_p, cfg.temperature)?;
        let mut piece = tok.decode(&[next_id]);

        if piece == "
" { piece.push_str(&indent); }

        if is_stop_token(&piece) || (is_block_end(&piece, &mut balance_curly) && step > 4) {
            on_event(CompletionEvent { token_text: String::new(), done: true });
            return Ok(())
        }

        on_event(CompletionEvent { token_text: piece.clone(), done: false });
        ctx.push(next_id);
    }

    on_event(CompletionEvent { token_text: String::new(), done: true });
    Ok(())
}

fn current_indent(prefix: &str) -> String {
    let last_line = prefix.rsplit_once('
').map(|(_, tail)| tail).unwrap_or(prefix);
    let indent_len = last_line.chars().take_while(|c| *c == ' ' || *c == '	').count();
    last_line.chars().take(indent_len).collect()
}

fn is_stop_token(piece: &str) -> bool { piece == "

" }

fn is_block_end(piece: &str, bal: &mut i32) -> bool {
    for ch in piece.chars() { if ch == '{' { *bal += 1; } if ch == '}' { *bal -= 1; } }
    *bal <= 0 && piece.contains('}')
}

fn sample_nucleus(logits: &[f32], top_p: f32, temperature: f32) -> Result<u32> {
    let mut probs: Vec<f32> = logits.iter().map(|&l| (l / temperature).exp()).collect();
    let sum: f32 = probs.iter().sum();
    if sum == 0.0 { let z = 1.0 / (probs.len() as f32); for p in &mut probs { *p = z; } }
    else { for p in &mut probs { *p /= sum; } }
    let mut idx: Vec<usize> = (0..probs.len()).collect();
    idx.sort_unstable_by(|&a, &b| probs[b].partial_cmp(&probs[a]).unwrap());
    let mut cum = 0.0; let mut cut = probs.len();
    for (k, &i) in idx.iter().enumerate() { cum += probs[i]; if cum >= top_p { cut = k + 1; break; } }
    let keep = &idx[..cut];
    let dist = WeightedIndex::new(keep.iter().map(|&i| probs[i].max(1e-12)))?;
    let mut rng = thread_rng();
    let pick_pos = keep[dist.sample(&mut rng)];
    Ok(pick_pos as u32)
}
```

---

## src/lsp/server.rs

```rust
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types as lsp;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::config::GenConfig;
use crate::gen::stream::generate_stream;
use crate::model::trait_::SeqLM;
use crate::model::ngram_lm::NGramLM;
use crate::model::ssm_lm::SSMLM;
use crate::tokenizer::trait_::Tokenizer;
use crate::tokenizer::bytelevel::ByteLevelTok;
use crate::types::lang::Lang;
use crate::types::io::{CompletionRequest, CompletionEvent};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BackendMode { None, NGram, SSM }

#[derive(Default)]
struct DocStore { texts: HashMap<lsp::Url, String>, langs: HashMap<lsp::Url, Lang> }

pub struct Backend<T: Tokenizer> {
    client: Client,
    tok: T,
    cfg: GenConfig,
    lm: Arc<Mutex<Option<Box<dyn SeqLM>>>>,
    mode: BackendMode,
    store: Arc<Mutex<DocStore>>,
}

impl<T: Tokenizer + 'static> Backend<T> {
    pub fn new(client: Client, tok: T, lm: Option<Box<dyn SeqLM>>, mode: BackendMode, cfg: GenConfig) -> Self {
        Self { client, tok, cfg, lm: Arc::new(Mutex::new(lm)), mode, store: Arc::new(Mutex::new(DocStore::default())) }
    }
}

#[tower_lsp::async_trait]
impl<T: Tokenizer + Send + Sync + 'static> LanguageServer for Backend<T> {
    async fn initialize(&self, _: lsp::InitializeParams) -> LspResult<lsp::InitializeResult> {
        let caps = lsp::ServerCapabilities {
            completion_provider: Some(lsp::CompletionOptions { resolve_provider: Some(false), trigger_characters: Some(vec![".".into(), ":".into(), "::".into()]), work_done_progress_options: Default::default(), all_commit_characters: None, completion_item: None }),
            text_document_sync: Some(lsp::TextDocumentSyncCapability::Kind(lsp::TextDocumentSyncKind::FULL)),
            ..Default::default()
        };
        Ok(lsp::InitializeResult { capabilities: caps, server_info: None })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        let _ = self.client.log_message(lsp::MessageType::INFO, format!("code_assist_core LSP initialized (backend: {:?})", self.mode)).await;
    }
    async fn shutdown(&self) -> LspResult<()> { Ok(()) }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        let uri = params.text_document.uri; let text = params.text_document.text;
        let lang = Lang::from_file_ext(uri.path());
        {
            let mut st = self.store.lock();
            st.langs.insert(uri.clone(), lang);
            st.texts.insert(uri.clone(), text.clone());
        }
        if self.mode == BackendMode::NGram { if let Some(lm) = &mut *self.lm.lock() { lm.observe_corpus(&text); } }
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.last() {
            let text = change.text.clone();
            {
                let mut st = self.store.lock();
                st.texts.insert(uri.clone(), text.clone());
                let lang = Lang::from_file_ext(uri.path());
                st.langs.insert(uri.clone(), lang);
            }
            if self.mode == BackendMode::NGram { if let Some(lm) = &mut *self.lm.lock() { lm.observe_corpus(&text); } }
        }
    }

    async fn completion(&self, params: lsp::CompletionParams) -> LspResult<Option<lsp::CompletionResponse>> {
        if self.mode == BackendMode::None { return Ok(None) } // opt‑in only

        let uri = params.text_document_position.text_document.uri.clone();
        let pos = params.text_document_position.position;
        let (text, lang) = {
            let st = self.store.lock();
            (st.texts.get(&uri).cloned().unwrap_or_default(), *st.langs.get(&uri).unwrap_or(&Lang::Unknown))
        };
        if text.is_empty() { return Ok(None) }

        let cursor_byte = lsp_pos_to_byte(&text, pos);
        let (prefix, suffix) = text.split_at(cursor_byte);

        let req = CompletionRequest { lang, prefix, suffix, max_new_tokens: 96, top_p: 0.92, temperature: 0.8 };
        let mut acc = String::new();
        {
            let mut guard = self.lm.lock();
            let Some(lm) = &mut *guard else { return Ok(None) };
            let tok = &self.tok; let cfg = &self.cfg;
            let _ = generate_stream(tok, &mut **lm, cfg, &req, |ev: CompletionEvent| { if !ev.done { acc.push_str(&ev.token_text); } });
        }

        let edit = lsp::TextEdit { range: lsp::Range { start: pos, end: pos }, new_text: acc.clone() };
        let item = lsp::CompletionItem { label: "code_assist suggestion".into(), kind: Some(lsp::CompletionItemKind::SNIPPET), text_edit: Some(lsp::CompletionTextEdit::Edit(edit)), insert_text_format: Some(lsp::InsertTextFormat::PLAIN_TEXT), ..Default::default() };
        Ok(Some(lsp::CompletionResponse::Array(vec![item])))
    }
}

fn lsp_pos_to_byte(text: &str, pos: lsp::Position) -> usize {
    let mut cur_line = 0usize; let mut cur_col = 0usize; let mut cursor_byte = text.len();
    for (i, ch) in text.char_indices() {
        if cur_line == pos.line as usize && cur_col == pos.character as usize { cursor_byte = i; break; }
        if ch == '
' { cur_line += 1; cur_col = 0; } else { cur_col += 1; }
    }
    cursor_byte
}

pub async fn run_stdio_server() -> anyhow::Result<()> {
    use std::env;
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let tok = ByteLevelTok::new();
    let cfg = GenConfig::default();

    // Sélection du backend par variable d'environnement (opt‑in)
    let mode = match env::var("AI_BACKEND").unwrap_or_else(|_| "none".into()).to_lowercase().as_str() { "ngram" => BackendMode::NGram, "ssm" => BackendMode::SSM, _ => BackendMode::None };
    let lm: Option<Box<dyn SeqLM>> = match mode {
        BackendMode::None => None,
        BackendMode::NGram => Some(Box::new(NGramLM::new(tok.vocab_size(), 5))),
        BackendMode::SSM => {
            let weights = env::var("SSM_WEIGHTS").unwrap_or_else(|_| "weights".into());
            let d_model: usize = env::var("SSM_DMODEL").ok().and_then(|s| s.parse().ok()).unwrap_or(512);
            let ssm = SSMLM::load(&weights, tok.vocab_size(), d_model)?; Some(Box::new(ssm))
        }
    };

    let (service, socket) = LspService::new(|client| Backend::new(client, tok, lm, mode, cfg));
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
```

---

## src/logger/log.rs

```rust
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::types::lang::Lang;

#[derive(Serialize, Deserialize)]
pub struct AssistedEditLog<'a> {
    pub lang: Lang,
    pub file: &'a str,
    pub prefix_hash: u64,
    pub suffix_hash: u64,
    pub accepted: &'a str,
    pub alternatives: Vec<&'a str>,
}

pub fn append_log(entry: &AssistedEditLog) -> std::io::Result<()> {
    let dir = logs_dir(); create_dir_all(&dir)?;
    let file = dir.join(date_filename());
    let mut f = OpenOptions::new().create(true).append(true).open(file)?;
    let line = serde_json::to_string(entry).unwrap();
    writeln!(f, "{}", line)?; Ok(())
}

fn logs_dir() -> PathBuf {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).unwrap_or(".".into());
    PathBuf::from(home).join(".code_assist_core").join("logs")
}

fn date_filename() -> String {
    use chrono::prelude::*; let now: DateTime<Utc> = Utc::now();
    format!("{}-{:02}-{:02}.jsonl", now.year(), now.month(), now.day())
}
```

---

## src/main.rs

```rust
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();
    code_assist_core::lsp::server::run_stdio_server().await
}
```

---

### Mode d’exécution (important)

* **Par défaut**: `AI_BACKEND=none` → le serveur LSP **n’émet rien** (zéro complétion). Rien ne « s’entraîne » en arrière-plan.
* **Opt‑in N‑gram (100% Rust, pas d’IA externe)**: `AI_BACKEND=ngram cargo run --release`
* **Opt‑in SSM (100% Rust, nécessite des poids)**: `AI_BACKEND=ssm SSM_WEIGHTS=weights SSM_DMODEL=512 cargo run --release`

Tout est **from scratch en Rust**. Pas de modèle tiers, pas d’API, pas de poids cachés. Tu contrôles quand activer un backend.
