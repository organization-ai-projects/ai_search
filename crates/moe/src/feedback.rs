use std::io::{self, Write};

pub fn ask_feedback(
    input: &str,
    results: &[(usize, Result<String, String>, u128)],
    selected_experts: Vec<String>,
    rule: Option<String>,
) {
    println!("\nLe résultat obtenu vous convient-il ? (oui/non)");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut feedback = String::new();
    if io::stdin().read_line(&mut feedback).is_ok() {
        let feedback = feedback.trim().to_lowercase();
        if feedback == "non" || feedback == "n" {
            // Loguer le feedback négatif
            let log_entry = serde_json::json!({
                "input": input,
                "rule": rule,
                "selected_experts": selected_experts,
                "results": results.iter().map(|(_, r, _)| match r { Ok(v) => v.clone(), Err(e) => format!("ERREUR: {}", e) }).collect::<Vec<_>>()
            });
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("routing_feedback.json")
            {
                let _ = writeln!(file, "{}", log_entry);
            }
            println!("[Feedback] Merci, le cas a été enregistré pour analyse.");
        }
    }
}
