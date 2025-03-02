# Key-Value Store

This project is a simple key-value store implemented in Rust. It consists of two packages:
1. **`kvstore`**: A library that provides the core key-value store functionality.
2. **`cli`**: A command-line interface (CLI) that uses the `kvstore` library to interact with the key-value store.

The key-value store persists data to a file, allowing data to be saved and retrieved across program runs.

---

```
kvstore_project/
├── Cargo.toml            # Workspace configuration
├── kvstore/              # Library package
│   ├── Cargo.toml        # Library package configuration
│   └── src/
│       └── lib.rs        # Library implementation
├── cli/                  # CLI package
│   ├── Cargo.toml        # CLI package configuration
│   └── src/
│       └── main.rs       # CLI implementation
└── README.md             # Project documentation
```

## Features

- **Key-Value Store**:
  - Stores key-value pairs (`String` keys and `String` values).
  - Persists data to a file for durability.
  - Provides two public methods:
    - `set(key: String, value: String)`: Saves a key-value pair.
    - `get(key: &str) -> Option<&String>`: Retrieves the value for a given key.

- **CLI**:
  - Takes a file path as a command-line argument to initialize the key-value store. If provide `-b/--binary` as arg, will save file in binary format.
  - Demonstrates the usage of the `set` and `get` methods.