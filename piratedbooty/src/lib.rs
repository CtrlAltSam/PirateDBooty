use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};

pub struct PirateDBooty {
    data: std::collections::HashMap<String, String>,
}

impl PirateDBooty {
    pub fn new() -> Self {
        PirateDBooty { data: HashMap::new()}
    }

    pub fn set(&mut self, key: String, value: String){
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str){
        self.data.remove(key);
    }

    pub fn load(path: &str) -> Self {
        let mut db = PirateDBooty::new();
        if let Ok(file) = File::open(path) {
            for line in BufReader::new(file).lines().flatten() {
                if let Some((k, v)) = line.split_once('=') {
                    db.data.insert(k.to_string(), v.to_string());
                }
            }
        }
        db
    }

    pub fn persist(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true).
            truncate(true)
            .open(path)
            .unwrap();

        for (k, v) in &self.data {
            writeln!(file, "{}={}", k, v).unwrap();
        }
    }
}