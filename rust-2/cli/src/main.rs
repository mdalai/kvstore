
use kvstore::KeyValueStore;
use clap::Parser;

#[derive(Parser, Debug)]
struct  Args {
    /// The file path to store the key-value pairs
    file: String,
    /// Use binary format
    #[arg(short = 'b', long = "binary")]
    binary: bool,
}


fn main() {
    let args = Args::parse();

    let filepath = args.file.as_str();
    let use_binary = args.binary;

    let mut kv_store = KeyValueStore::new(filepath, use_binary);

    kv_store.set("name".to_string(), "John".to_string()).unwrap();
    kv_store.set("age".to_string(), "30".to_string()).unwrap();

    println!("name: {:?}", kv_store.get("name"));
    println!("age: {:?}", kv_store.get("age"));
    println!("unknown: {:?}", kv_store.get("unknown"));
}
