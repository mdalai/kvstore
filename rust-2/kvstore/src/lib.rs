

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;


#[derive(Debug)]
pub struct KeyValueStore {
    store: HashMap<String, String>,
    filepath: String,
    use_binary: bool,
}

impl KeyValueStore {
    // new instance
    pub fn new(filepath: &str, use_binary: bool) -> Self {
        let store;   
        if use_binary {
            store= Self::read_from_binary_file(filepath).unwrap_or_else(|_| HashMap::new());
        } else {
            store = Self::read_from_file(filepath).unwrap_or_else(|_| HashMap::new());
        }
        
        KeyValueStore {
            store,
            filepath: filepath.to_string(),
            use_binary,
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

    fn read_from_binary_file(filepath: &str) -> io::Result<HashMap<String, String>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let store = bincode::deserialize_from(reader).unwrap();
        Ok(store)
    }

    fn write_to_binary_file(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.filepath)?;

        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self.store).unwrap();

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        self.store.insert(key, value);
        if self.use_binary {
            self.write_to_binary_file()
        } else {
            self.write_to_file()
        }
    }
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
