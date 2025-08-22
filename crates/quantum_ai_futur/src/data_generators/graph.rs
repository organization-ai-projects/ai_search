use super::data_generator::DataGenerator;
use ndarray::Array1;
use rand::Rng;

pub struct GraphGen {
    pub nodes: usize,
    pub max_samples: usize,
}

impl DataGenerator for GraphGen {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let n = n.min(self.max_samples);
        let mut rng = rand::thread_rng();
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for _ in 0..n {
            // Génère une matrice d'adjacence aplatie
            let mut adj = vec![0.0; self.nodes * self.nodes];
            for i in 0..self.nodes {
                for j in 0..self.nodes {
                    if i != j && rng.gen_bool(0.2) {
                        adj[i * self.nodes + j] = 1.0;
                    }
                }
            }
            let label = if adj.iter().sum::<f64>() > (self.nodes * self.nodes / 4) as f64 {
                1.0
            } else {
                0.0
            };
            inputs.push(Array1::from(adj));
            targets.push(Array1::from(vec![label]));
        }
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        self.nodes * self.nodes
    }
    fn output_size(&self) -> usize {
        1
    }
}
