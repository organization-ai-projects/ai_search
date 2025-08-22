use super::data_generator::DataGenerator;
use ndarray::Array1;
use rand::Rng;

pub struct MathsGenerator;

impl DataGenerator for MathsGenerator {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        // Génère des additions simples : [a, b] -> [a + b]
        let mut rng = rand::thread_rng();
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for _ in 0..n {
            let a = rng.gen_range(0.0..10.0);
            let b = rng.gen_range(0.0..10.0);
            inputs.push(Array1::from(vec![a, b]));
            targets.push(Array1::from(vec![a + b]));
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
