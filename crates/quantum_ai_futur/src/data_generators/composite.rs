use super::data_generator::DataGenerator;
use ndarray::Array1;
use rand::seq::SliceRandom;

pub struct CompositeGenerator {
    pub generators: Vec<Box<dyn DataGenerator>>,
    pub n_each: usize,
}

impl DataGenerator for CompositeGenerator {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let mut all_inputs = Vec::new();
        let mut all_targets = Vec::new();
        for gen in self.generators.iter() {
            let (inputs, targets) = gen.generate(self.n_each);
            all_inputs.extend(inputs);
            all_targets.extend(targets);
        }
        let mut zipped: Vec<_> = all_inputs
            .into_iter()
            .zip(all_targets.into_iter())
            .collect();
        let mut rng = rand::thread_rng();
        zipped.shuffle(&mut rng);
        let (inputs, targets): (Vec<_>, Vec<_>) = zipped.into_iter().take(n).unzip();
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        self.generators[0].input_size()
    }
    fn output_size(&self) -> usize {
        self.generators[0].output_size()
    }
}
