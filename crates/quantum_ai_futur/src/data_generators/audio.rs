use super::data_generator::DataGenerator;
use ndarray::Array1;
use rand::Rng;

pub struct AudioGen {
    pub max_samples: usize,
    pub bins: usize,
}

impl DataGenerator for AudioGen {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let n = n.min(self.max_samples);
        let mut rng = rand::thread_rng();
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for _ in 0..n {
            let spec: Vec<f64> = (0..self.bins).map(|_| rng.gen_range(0.0..1.0)).collect();
            let label = if spec.iter().sum::<f64>() > (self.bins as f64) / 2.0 {
                1.0
            } else {
                0.0
            };
            inputs.push(Array1::from(spec));
            targets.push(Array1::from(vec![label]));
        }
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        self.bins
    }
    fn output_size(&self) -> usize {
        1
    }
}
