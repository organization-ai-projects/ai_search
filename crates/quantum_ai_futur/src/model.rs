#[derive(Clone, Debug)]
pub struct Sample {
    pub input: Vec<f32>,
    pub target: Vec<f32>,
}

// Enum pour activer dynamiquement l'activation
#[derive(Clone, Copy, Debug)]
pub enum ActivationFn {
    Relu,
    Tanh,
    Sigmoid,
    TanhTime,
}

use burn::data::dataloader::batcher::Batcher;
use burn::module::Module;
use burn::nn;
use burn::tensor::activation::{relu, sigmoid, tanh};
use burn::tensor::{backend::Backend, Tensor};

#[derive(Module, Debug)]
pub struct MlpModel<B: Backend> {
    layer1: nn::Linear<B>,
    layer2: nn::Linear<B>,
}

impl<B: Backend> MlpModel<B> {
    pub fn new(input: usize, hidden: usize, output: usize, device: &B::Device) -> Self {
        Self {
            layer1: nn::LinearConfig::new(input, hidden).init(device),
            layer2: nn::LinearConfig::new(hidden, output).init(device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 2>, activation: ActivationFn) -> Tensor<B, 2> {
        let x = self.layer1.forward(x.clone());
        let x = match activation {
            ActivationFn::Relu => relu(x),
            ActivationFn::Tanh => tanh(x),
            ActivationFn::Sigmoid => sigmoid(x),
            ActivationFn::TanhTime => {
                // tanh(mean(x) sur dim 1, unsqueeze)
                let mean = x.clone().mean_dim(1);
                mean.unsqueeze().tanh()
            }
        };
        self.layer2.forward(x)
    }
}

use burn_tch::{LibTorch, LibTorchDevice};

pub struct MyBatcher<B: Backend> {
    _b: std::marker::PhantomData<B>,
}

impl<B: Backend> MyBatcher<B> {
    pub fn new() -> Self {
        Self {
            _b: std::marker::PhantomData,
        }
    }
}

impl<B: Backend> Batcher<Sample, Tensor<B, 2>, Tensor<B, 2>> for MyBatcher<B> {
    fn batch(&self, items: Vec<Sample>, device: &B::Device) -> (Tensor<B, 2>, Tensor<B, 2>) {
        let inputs: Vec<f32> = items.iter().flat_map(|s| s.input.iter()).copied().collect();
        let targets: Vec<f32> = items
            .iter()
            .flat_map(|s| s.target.iter())
            .copied()
            .collect();
        let shape_in = [items.len(), items[0].input.len()];
        let shape_out = [items.len(), items[0].target.len()];
        let input = Tensor::<B, 2>::from_floats(&inputs, device).reshape(shape_in);
        let target = Tensor::<B, 2>::from_floats(&targets, device).reshape(shape_out);
        (input, target)
    }
}
