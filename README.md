# Trust me!
Use `trust_me` replace `unsafe`. Compiler shut up! Readers shut up too! My code has no bugs, trust me!

add the following line to your Cargo.toml:
```toml
shut_up = "0.1.0"
```

```rust
use shut_up::trust_me;

fn main() {
    trust_me! {
        // Your original "safe" code
        println!("trust me, this is safe!");
    }
}
```
