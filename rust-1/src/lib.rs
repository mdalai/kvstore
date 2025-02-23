
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};
use std::path::Path;


#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<String, String>,
    filepath: String,
}

impl KeyValueStore {
    // new instance
    pub fn new(filepath: &str) -> Self {
        let store = Self::read_from_file(filepath).unwrap_or_else(|_| HashMap::new());
        KeyValueStore {
            store,
            filepath: filepath.to_string(),
        }
    }

    fn read_from_file(filepath: &str) -> io::Result<HashMap<String, String>> {
        let path = Path::new(filepath);
        if !path.exists() {
            return Ok(HashMap::new());
        }
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let mut store = HashMap::new();

        for line in reader.lines(){
            let line= line?;
            let parts: Vec<&str> = line.splitn(2, ',').collect();
            if parts.len() == 2 {
                store.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
        Ok(store)
    }

    fn write_to_file(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.filepath)?;
        for (k, v) in &self.store {
            writeln!(file, "{},{}", k, v)?;
        }

        Ok(())
    }


    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        self.store.insert(key, value);
        self.write_to_file()
    }
}