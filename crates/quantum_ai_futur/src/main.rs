// Réseau neuronal minimaliste à 2 couches (input -> hidden -> output)
// Implementé avec ndarray, support de mini-batch, shuffle, SGD ou Adam, et save/load JSON.

mod data_generators;
use data_generators::{
    agnews_loader::AgnewsLoader, audio::AudioGen, cifar_loader::CifarLoader, codegen::CodeGen,
    composite::CompositeGenerator, graph::GraphGen, maths::MathsGenerator,
    mnist_loader::MnistLoader, nlp::NlpGenerator, tabular::TabularGen, DataGenerator,
};
use quantum_ai_futur::print::print_usage;
mod model;
use burn::data::dataloader::DataLoaderBuilder;
use burn::data::dataset::{Dataset, InMemDataset};
use burn::optim::AdamConfig;
use burn::tensor::Tensor;
use burn::train::LearnerBuilder;
use burn_tch::{LibTorch, LibTorchDevice};
use model::{ActivationFn, MlpModel, MyBatcher, Sample};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(&args[0]);
        return;
    }

    let epochs = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(500usize); // epochs par défaut augmenté
    let batch_size = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(16usize); // batch plus grand
    let lr = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.1f64); // learning rate plus faible
    let optimizer = args.get(4).map(|s| s.as_str()).unwrap_or("sgd");
    let save_path = args.get(5).map(|s| s.as_str());
    let activation = args
        .iter()
        .position(|a| a == "--activation")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("sigmoid");
    let fractal_depth = args
        .iter()
        .position(|a| a == "--fractal-depth")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    let dataset = args
        .iter()
        .position(|a| a == "--dataset" || a == "-d")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("maths");
    let n_samples = args
        .iter()
        .position(|a| a == "--max-samples" || a == "-n")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000); // augmenter à 1000 par défaut

    let (generator, input_size, output_size): (Box<dyn DataGenerator>, usize, usize) = match dataset
    {
        "mnist" => {
            let g = MnistLoader {
                max_samples: n_samples,
            };
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "cifar" => {
            let g = CifarLoader {
                max_samples: n_samples,
            };
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "agnews" => {
            let g = AgnewsLoader {
                max_samples: n_samples,
            };
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "maths" => {
            let g = MathsGenerator;
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "nlp" => {
            let g = NlpGenerator;
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "code" => {
            let g = CodeGen;
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "tabular" => {
            let g = TabularGen {
                features: 8,
                max_samples: n_samples,
            };
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "audio" => {
            let g = AudioGen {
                max_samples: n_samples,
                bins: 64,
            };
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "graph" => {
            let g = GraphGen {
                nodes: 6,
                max_samples: n_samples,
            };
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
        "mixte" => {
            let g1 = Box::new(MnistLoader {
                max_samples: n_samples / 6,
            });
            let g2 = Box::new(CifarLoader {
                max_samples: n_samples / 6,
            });
            let g3 = Box::new(MathsGenerator);
            let g4 = Box::new(NlpGenerator);
            let g5 = Box::new(CodeGen);
            let g6 = Box::new(TabularGen {
                features: 8,
                max_samples: n_samples / 6,
            });
            let g7 = Box::new(AudioGen {
                max_samples: n_samples / 6,
                bins: 64,
            });
            let g8 = Box::new(GraphGen {
                nodes: 6,
                max_samples: n_samples / 6,
            });
            let composite = CompositeGenerator {
                generators: vec![g1, g2, g3, g4, g5, g6, g7, g8],
                n_each: n_samples / 8,
            };
            let input_size = composite.input_size();
            let output_size = composite.output_size();
            (Box::new(composite), input_size, output_size)
        }
        _ => {
            eprintln!(
                "Dataset inconnu: {}. Utilisation de maths par défaut.",
                dataset
            );
            let g = MathsGenerator;
            let input_size = g.input_size();
            let output_size = g.output_size();
            (Box::new(g), input_size, output_size)
        }
    };

    let (mut inputs, targets) = generator.generate(n_samples);
    // Ajout d'une feature temporelle t et d'une mémoire implicite y_{t-1} à chaque entrée
    let n = inputs.len();
    let beta = 2.0;
    let alpha = 0.8; // poids de la mémoire
    let mut y_prev = 0.0;
    for (i, x) in inputs.iter_mut().enumerate() {
        let t = if n > 1 {
            i as f64 / (n as f64 - 1.0)
        } else {
            0.0
        };
        // On calcule y_t = tanh(mean(x) + β·t + α·y_{t-1})
        let x_mean = x.mean().unwrap_or(0.0);
        let y_t = (x_mean + beta * t + alpha * y_prev).tanh();
        y_prev = y_t;
        // On ajoute t et y_t comme features
        x.append(ndarray::Axis(0), ndarray::Array1::from(vec![t, y_t]).view())
            .unwrap();
    }

    // Conversion des données en Vec<f32> pour burn
    let dataset: Vec<Sample> = inputs
        .into_iter()
        .zip(targets.into_iter())
        .map(|(x, y)| Sample {
            input: x.iter().map(|v| *v as f32).collect(),
            target: y.iter().map(|v| *v as f32).collect(),
        })
        .collect();

    // Sélection dynamique de l'activation
    let activation_fn = match activation {
        "tanh" => ActivationFn::Tanh,
        "sigmoid" => ActivationFn::Sigmoid,
        "relu" => ActivationFn::Relu,
        "tanh_time" => ActivationFn::TanhTime,
        _ => ActivationFn::Tanh,
    };

    let device = LibTorchDevice::Cuda(0);
    let model = MlpModel::<LibTorch<f32>>::new(input_size + 2, 32, output_size, &device);

    // Adapter DataLoader à la nouvelle API burn 0.18
    let dataset = InMemDataset::new(dataset);
    let dataloader = DataLoaderBuilder::new(MyBatcher::<LibTorch<f32>>::new())
        .batch_size(batch_size)
        .build(dataset);

    // Adapter l'optimiseur Adam à la nouvelle API
    let mut learner = LearnerBuilder::new(model)
        .with_optimiser(AdamConfig::new())
        .num_epochs(epochs)
        .build(&device);

    let _train_output = learner.fit(dataloader, &device);

    // Évaluation rapide après entraînement
    let model = learner.model();
    println!("Résultats après entraînement :");
    for i in 0..dataset.len().min(20) {
        let sample = &dataset[i];
        let shape = [1, sample.input.len()];
        let input = Tensor::<LibTorch<f32>, 2>::from_floats(sample.input.as_slice(), &device)
            .reshape(shape);
        let out = model.forward(input, activation_fn).to_data().value;
        let pred_class = out[0]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        let true_class = sample
            .target
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);
        println!(
            "target={} -> pred={} (proba={:.3})",
            true_class, pred_class, out[0][pred_class]
        );
    }
}
