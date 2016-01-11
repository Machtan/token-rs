# Token
[![Build status (master)](https://travis-ci.org/Machtan/token-rs.svg?branch=master)](https://travis-ci.org/Machtan/token-rs)

This is a small package containing a simple string-tokenizer for the rust programming language. The package also contains a simple sentence-splitting iterator.

(The sentence splitter might be moved, once I find out where I want it).

## Documentation

[machtan.github.io/token-rs/token](http://machtan.github.io/token-rs/token)

# Building
Add the following to your Cargo.toml file

```toml
[dependencies.token]
git = "https://github.com/machtan/token-rs"
```

# Examples

```rust
extern crate token;

let separators = vec![' ', '\n', '\t', '\r'];
let source: &str = "    Hello world \n  How do you do\t-Finely I hope";

let mut tokenizer = token::Tokenizer::new(source.as_bytes(), separators);
println!("Tokenizing...");
loop {
    let token = match tokenizer.next().unwrap() {
        Some(token) => token,
        None => { break; }
    };
    println!("- Got token: {}", token);
}
println!("Done!");
```

# License
MIT (do what you want with it)
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
