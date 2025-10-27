use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Ok(db) = bincode::deserialize_from(reader){
                return db;
            }
        }
        PirateDBooty::new()
    }

    pub fn persist(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true).
            truncate(true)
            .open(path)
            .unwrap();

        bincode::serialize_into(&mut file, &self).unwrap();
    }
}