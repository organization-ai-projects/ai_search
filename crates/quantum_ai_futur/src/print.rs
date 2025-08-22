pub fn print_usage(bin: &str) {
    eprintln!(
        "Usage: {} [epochs] [batch_size] [lr] [optimizer] [save_path]",
        bin
    );
    eprintln!("Defaults: epochs=1000 batch_size=16 lr=0.1 optimizer=sgd");
}
