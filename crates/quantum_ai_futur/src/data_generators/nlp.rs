use super::data_generator::DataGenerator;
use ndarray::Array1;

pub struct NlpGenerator;

impl DataGenerator for NlpGenerator {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        // Génère des séquences binaires simples : [0, 1, 0, 1] -> [1] si nombre de 1 pair, [0] sinon
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for i in 0..n {
            let seq = vec![
                ((i >> 0) & 1) as f64,
                ((i >> 1) & 1) as f64,
                ((i >> 2) & 1) as f64,
                ((i >> 3) & 1) as f64,
            ];
            let ones = seq.iter().filter(|&&v| v == 1.0).count();
            let target = if ones % 2 == 0 { 1.0 } else { 0.0 };
            inputs.push(Array1::from(seq));
            targets.push(Array1::from(vec![target]));
        }
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        4
    }
    fn output_size(&self) -> usize {
        1
    }
}
