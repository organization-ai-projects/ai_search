use super::data_generator::DataGenerator;
use ndarray::Array1;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

const CIFAR_URL: &str = "https://www.cs.toronto.edu/~kriz/cifar-10-binary.tar.gz";

fn download_if_needed(url: &str, dest: &str) {
    if !Path::new(dest).exists() {
        let mut resp = reqwest::blocking::get(url).expect("download failed");
        let mut out = File::create(dest).expect("create file failed");
        std::io::copy(&mut resp, &mut out).expect("copy failed");
    }
}

fn extract_tar_gz(path: &str, out_dir: &str) {
    if !Path::new(out_dir).exists() {
        let f = File::open(path).expect("open failed");
        let d = flate2::read::GzDecoder::new(f);
        let mut archive = tar::Archive::new(d);
        archive.unpack(out_dir).expect("unpack failed");
    }
}

pub struct CifarLoader {
    pub max_samples: usize,
}

impl DataGenerator for CifarLoader {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let n = n.min(self.max_samples);
        fs::create_dir_all(".cache").ok();
        let tar_path = ".cache/cifar-10-binary.tar.gz";
        let out_dir = ".cache/cifar-10-batches-bin";
        download_if_needed(CIFAR_URL, tar_path);
        extract_tar_gz(tar_path, ".cache");
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        let batch_path = format!("{}/data_batch_1.bin", out_dir);
        let mut f = File::open(batch_path).expect("open batch");
        let mut buf = vec![0u8; 10000 * 3073];
        f.read_exact(&mut buf).expect("read batch");
        for i in 0..n {
            let start = i * 3073;
            let label = buf[start] as f64;
            let img = &buf[start + 1..start + 3073];
            let arr = Array1::from(img.iter().map(|&b| b as f64 / 255.0).collect::<Vec<_>>());
            inputs.push(arr);
            targets.push(Array1::from(vec![label]));
        }
        (inputs, targets)
    }
    fn input_size(&self) -> usize {
        32 * 32 * 3
    }
    fn output_size(&self) -> usize {
        1
    }
}
