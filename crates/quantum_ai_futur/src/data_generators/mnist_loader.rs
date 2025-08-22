use super::data_generator::DataGenerator;
use ndarray::Array1;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

const MNIST_URL: &str =
    "https://storage.googleapis.com/cvdf-datasets/mnist/train-images-idx3-ubyte.gz";
const MNIST_LABELS_URL: &str =
    "https://storage.googleapis.com/cvdf-datasets/mnist/train-labels-idx1-ubyte.gz";

use std::path::PathBuf;

fn cache_dir() -> PathBuf {
    // Chemin en dur relatif à la racine du workspace
    PathBuf::from("crates/quantum_ai_futur/.cache")
}

fn download_if_needed(url: &str, dest: &Path) {
    if dest.exists() {
        println!("[MNIST] Présence détectée : {:?}", dest);
    } else {
        println!("[MNIST] Absent, téléchargement requis : {:?}", dest);
    }
    let mut need_download = !dest.exists();
    if !need_download {
        // Vérifie si le fichier est un gzip valide
        let f = File::open(dest);
        if let Ok(f) = f {
            let mut d = flate2::read::GzDecoder::new(f);
            let mut buf = Vec::new();
            if d.read_to_end(&mut buf).is_err() {
                eprintln!(
                    "Fichier corrompu, suppression et re-téléchargement: {:?}",
                    dest
                );
                let _ = fs::remove_file(dest);
                need_download = true;
            }
        } else {
            need_download = true;
        }
    }
    if need_download {
        println!("Téléchargement de {}...", url);
        let resp = reqwest::blocking::get(url);
        match resp {
            Ok(mut resp) => {
                let mut out = File::create(dest).expect("create file failed");
                std::io::copy(&mut resp, &mut out).expect("copy failed");
                // Vérification de la présence après téléchargement
                if dest.exists() {
                    println!(
                        "[MNIST] Présence détectée après téléchargement : {:?}",
                        dest
                    );
                } else {
                    println!("[MNIST] Toujours absent après téléchargement : {:?}", dest);
                }
            }
            Err(e) => {
                panic!("Échec du téléchargement de {}: {}", url, e);
            }
        }
    }
}

fn extract_gz(path: &Path) -> Vec<u8> {
    let f = File::open(path).expect("open failed");
    let mut d = flate2::read::GzDecoder::new(f);
    let mut buf = Vec::new();
    d.read_to_end(&mut buf).unwrap_or_else(|e| {
        panic!(
            "Décompression échouée pour {:?}: {}. Supprimez le fichier et relancez.",
            path, e
        )
    });
    buf
}

pub struct MnistLoader {
    pub max_samples: usize,
}

impl DataGenerator for MnistLoader {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let n = n.min(self.max_samples);
        let cache = cache_dir();
        println!("[MNIST] Dossier cache utilisé : {:?}", cache);
        fs::create_dir_all(&cache).ok();
        let img_path = cache.join("mnist-images.gz");
        let lbl_path = cache.join("mnist-labels.gz");
        println!("[MNIST] Fichier images : {:?}", img_path);
        println!("[MNIST] Fichier labels : {:?}", lbl_path);
        download_if_needed(MNIST_URL, &img_path);
        download_if_needed(MNIST_LABELS_URL, &lbl_path);
        let images = extract_gz(&img_path);
        let labels = extract_gz(&lbl_path);
        let img_offset = 16;
        let lbl_offset = 8;
        let img_size = 28 * 28;
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for i in 0..n {
            let start = img_offset + i * img_size;
            let end = start + img_size;
            let img = &images[start..end];
            let sum: u32 = img.iter().map(|&b| b as u32).sum();
            let max = img.iter().map(|&b| b as u32).max().unwrap_or(0);
            println!("[MNIST][DEBUG] Image {i}: somme_pixels={sum} max_pixel={max}");
            let arr = Array1::from(img.iter().map(|&b| b as f64 / 255.0).collect::<Vec<_>>());
            let label = labels[lbl_offset + i] as usize;
            let mut one_hot = vec![0.0; 10];
            if label < 10 {
                one_hot[label] = 1.0;
            }
            inputs.push(arr);
            targets.push(Array1::from(one_hot));
        }
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        28 * 28
    }
    fn output_size(&self) -> usize {
        10
    }
}
