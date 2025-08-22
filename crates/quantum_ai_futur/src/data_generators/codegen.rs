use super::data_generator::DataGenerator;
use ndarray::Array1;

const SNIPPETS: &[(&str, f64)] = &[
    ("print('Hello, world!')", 0.0),
    ("for i in range(10): print(i)", 0.0),
    ("fn main() { println!(\"Hello\"); }", 1.0),
    ("let x = 42;", 1.0),
    ("def add(a, b): return a + b", 0.0),
    ("struct Point { x: f64, y: f64 }", 1.0),
];

pub struct CodeGen;

impl DataGenerator for CodeGen {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for i in 0..n {
            let (snippet, label) = SNIPPETS[i % SNIPPETS.len()];
            let len = snippet.len() as f64;
            let tokens = snippet.split_whitespace().count() as f64;
            inputs.push(Array1::from(vec![len, tokens]));
            targets.push(Array1::from(vec![label]));
        }
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        2
    }
    fn output_size(&self) -> usize {
        1
    }
}
