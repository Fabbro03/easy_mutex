# EasyMutex

[![Crates.io](https://img.shields.io/crates/v/easy_mutex.svg)](https://crates.io/crates/easy_mutex)
[![Docs.rs](https://docs.rs/easy_mutex/badge.svg)](https://docs.rs/easy_mutex)
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

---

**EasyMutex** is a lightweight, thread-safe, and clonable wrapper around `std::sync::Mutex<T>` using `Arc`.

It simplifies shared mutable state management by providing an easy-to-use API for safely reading and writing data across threads, with handy convenience methods and error handling.

---

## Features

- Thread-safe mutable access using `Mutex` wrapped in an `Arc`.
- Cloneable wrapper for shared ownership.
- All APIs drop the lock before returning, so it should be deadlock free.
- Simple API: `read()`, `write()`, `read_result()`, `write_result()`.
- Implements `From<T>` for ergonomic construction.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
easy_mutex = "0.1.1"
```

---

## Usage
```rust
use easy_mutex::EasyMutex;

let shared = EasyMutex::new(5);
let clone = shared.clone();

assert_eq!(shared.read(), 5);
clone.write(10);
assert_eq!(shared.read(), 10);

assert!(clone.write_result(2).is_ok());

let readed  = match shared.read_result() {
     Ok(val) => {println!("Safe read: {val}"); val},
     Err(e) => {println!("Poisoned mutex: {e}"); 0},
};
assert_eq!(readed, 2);

let data: EasyMutex<String> = "hello".to_string().into();
assert_eq!(data.read(), "hello");
```

---

## API Overview
- `EasyMutex::new(value)` — Create a new EasyMutex.
- `read()` — Acquire lock and clone the value. Panics if poisoned.
- `write(value)` — Acquire lock and replace the value. Panics if poisoned.
- `read_safe()` / `write_safe(value)` — Return Result with poison error info.
- `From<T>` implemented for convenient construction via `.into()`.

---

## Contributing
Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## Licensing
All contributions to this project are licensed under the terms of the Apache License, Version 2.0.

By contributing, you agree that your code will be released under the same license.
