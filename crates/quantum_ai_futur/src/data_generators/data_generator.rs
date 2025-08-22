use ndarray::Array1;

pub trait DataGenerator {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>);
    fn input_size(&self) -> usize;
    fn output_size(&self) -> usize;
}
