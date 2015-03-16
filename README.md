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

let mut tokenizer = tokenizer::Tokenizer::new(source.as_bytes(), separators);
println!("Tokenizing...");
for token in tokenizer {
    println!("- Got token: {}", token.unwrap());
}
println!("Done!");
```

# License
MIT (do what you want with it)