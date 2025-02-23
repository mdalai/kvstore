use kv_store::KeyValueStore;

fn main() {
    let mut kv_store = KeyValueStore::new("kv_store.txt");

    kv_store.set("name".to_string(), "John".to_string()).unwrap();
    kv_store.set("age".to_string(), "30".to_string()).unwrap();

    println!("name: {:?}", kv_store.get("name"));
    println!("age: {:?}", kv_store.get("age"));
    println!("unknown: {:?}", kv_store.get("unknown"));
}