use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path};
use serde::{Deserialize, Serialize};
use serde_json;

const CACHE_NAME: &str = "jp_cache.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyValue {
    word: String,
    full_path: String,
}

pub struct Cache {
    pub map: HashMap<String, String>,
    pub file_path: String,
}


impl Cache {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let map = create_or_load_json_file()?;
        let mut home_dir = dirs::home_dir().ok_or("Failed to find the user's home directory")?;
        home_dir.push(CACHE_NAME);
        Ok(Cache {
            map,
            file_path: home_dir.to_str().unwrap().to_owned(),
        })
    }

    pub fn get(&mut self, word: &str) -> Option<String> {
        if let Some(full_path) = self.map.get(word) {
            if Path::new(full_path).exists() {
                Some(full_path.clone())
            } else {
                self.map.remove(word);
                self.save().ok();
                None
            }
        } else {
            None
        }
    }

    pub fn store(&mut self, word: &str, full_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.map.insert(word.to_string(), full_path.to_string());
        self.save()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let key_values: Vec<KeyValue> = self
            .map
            .iter()
            .map(|(word, full_path)| KeyValue {
                word: word.clone(),
                full_path: full_path.clone(),
            })
            .collect();

        let json_data = serde_json::to_string_pretty(&key_values)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }
}


pub fn create_or_load_json_file() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => return Err("Failed to find the user's home directory".into()),
    };

    home_dir.push(CACHE_NAME);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&home_dir)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        let key_values: Vec<KeyValue> = vec![];
        let json_data = serde_json::to_string_pretty(&key_values)?;
        file.write_all(json_data.as_bytes())?;
    } else {
        let key_values: Vec<KeyValue> = serde_json::from_str(&contents)?;
        let mut map = HashMap::new();
        for kv in key_values {
            map.insert(kv.word, kv.full_path);
        }
        return Ok(map);
    }

    Err("Unexpected error".into())
}
