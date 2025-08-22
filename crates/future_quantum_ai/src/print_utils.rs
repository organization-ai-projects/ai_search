use crate::brainkit::{Delta, Repo};

/// Affiche l'historique des commits et des deltas du repo (pour debug/visualisation)
pub fn print_commit_history(repo: &Repo) {
    println!("\n--- Historique des commits ---");
    for (cid, commit) in &repo.commits {
        println!("Commit {}", cid.0);
        println!(
            "  Parents: {:?}",
            commit.parents.iter().map(|p| &p.0).collect::<Vec<_>>()
        );
        println!("  Message: {}", commit.meta.message);
        println!("  Forked: {}", commit.meta.forked);
        for (pid, pv) in &commit.params {
            println!("    Param: {}", pid.0);
            println!("      Blob: {}", (pv.blob).0);
            println!("      Deltas: {}", pv.deltas.len());
            for (i, d) in pv.deltas.iter().enumerate() {
                match d {
                    Delta::LowRank { r, scale, .. } => {
                        println!("        [{}] LowRank (r={}, scale={})", i, r, scale)
                    }
                    Delta::Sparse { idx, .. } => {
                        println!("        [{}] Sparse ({} entrées)", i, idx.len())
                    }
                }
            }
        }
    }
    println!("------------------------------\n");
}
