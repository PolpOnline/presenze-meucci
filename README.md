#### Generate a cookie key

To generate a cookie key,
you need to spin up a new Rust project with `cargo new your_project_name`
and paste the following code:

```toml
# Cargo.toml
rand = "0.9.0-alpha.1"
axum-extra = { version = "0.9.3", features = ["cookie-private"] }
```

```rust
// src/main.rs
use axum_extra::extract::cookie::Key;
use rand::{Rng, thread_rng};

fn main() {
    // Generate a cryptographically random key of 64 bytes
    let mut rng = thread_rng();
    let mut random_key = [0u8; 64];
    rng.fill(&mut random_key);
    match Key::try_from(&random_key[..]) {
        Ok(key) => {
            println!("Random key: {:?}", key.master());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
```

Then run the project with cargo run and copy the cookie key to the .env file in the backend directory.