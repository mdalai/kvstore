
use kvstore::KeyValueStore;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len()< 2 {
        eprintln!("Usage: {} <filepath>", args[0]);
        return;
    }
    let filepath = &args[1];

    let mut kv_store = KeyValueStore::new(filepath);

    kv_store.set("name".to_string(), "John".to_string()).unwrap();
    kv_store.set("age".to_string(), "30".to_string()).unwrap();

    println!("name: {:?}", kv_store.get("name"));
    println!("age: {:?}", kv_store.get("age"));
    println!("unknown: {:?}", kv_store.get("unknown"));
}
