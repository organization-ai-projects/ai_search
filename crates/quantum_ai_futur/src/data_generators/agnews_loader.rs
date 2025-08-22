use super::data_generator::DataGenerator;
use ndarray::Array1;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;

const AGNEWS_URL: &str =
    "https://raw.githubusercontent.com/mhjabreel/CharCnn_Keras/master/data/ag_news_csv/train.csv";

fn download_if_needed(url: &str, dest: &str) {
    if !Path::new(dest).exists() {
        let mut resp = reqwest::blocking::get(url).expect("download failed");
        let mut out = File::create(dest).expect("create file failed");
        std::io::copy(&mut resp, &mut out).expect("copy failed");
    }
}

pub struct AgnewsLoader {
    pub max_samples: usize,
}

impl DataGenerator for AgnewsLoader {
    fn generate(&self, n: usize) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
        let n = n.min(self.max_samples);
        fs::create_dir_all(".cache").ok();
        let csv_path = ".cache/agnews.csv";
        download_if_needed(AGNEWS_URL, csv_path);
        let mut rdr = csv::Reader::from_path(csv_path).expect("csv open failed");
        let mut inputs = Vec::with_capacity(n);
        let mut targets = Vec::with_capacity(n);
        for (i, result) in rdr.records().enumerate() {
            if i >= n {
                break;
            }
            let rec = result.expect("csv parse");
            let label: f64 = rec[0].parse().unwrap_or(0.0);
            let text = &rec[1];
            let len = text.len() as f64;
            let words = text.split_whitespace().count() as f64;
            inputs.push(Array1::from(vec![len, words]));
            targets.push(Array1::from(vec![label]));
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
