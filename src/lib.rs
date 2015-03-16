// Copyright 2015 Jakob Lautrup Nysom

//! A simple string tokenizer, and a tokenizer of sentences based on it.
//! The tokenizer ignores all the given separator chars, and returns the
//! characters between as string slice
//!
//! # Examples
//! **General use (as an iterator)**
//!
//! This is how you will probably use it
//!
//! ```
//! let separators = vec![' ', '\n', '\t', '\r'];
//! let source: &str = "    Hello world \n  How do you do\t-Finely I hope";
//!
//! let mut tokenizer = token::Tokenizer::new(source.as_bytes(), separators);
//! println!("Tokenizing...");
//! for token in tokenizer {
//!     println!("- Got token: {}", token.unwrap());
//! }
//! println!("Done!");
//! ```
//!
//! **Behavior**
//!
//! This is what to expect when parsing a string (or input from a reader)
//!
//! ```
//! let separators = vec![' ', '\n', '\t', '\r'];
//! let source: &str = "    Hello world \n  How do you do\t-Finely I hope";
//!
//! let mut tokenizer = token::Tokenizer::new(source.as_bytes(), separators);
//! assert_eq!("Hello",     tokenizer.next().expect("1").unwrap());
//! assert_eq!("world",     tokenizer.next().expect("2").unwrap());
//! assert_eq!("How",       tokenizer.next().expect("3").unwrap());
//! assert_eq!("do",        tokenizer.next().expect("4").unwrap());
//! assert_eq!("you",       tokenizer.next().expect("5").unwrap());
//! assert_eq!("do",        tokenizer.next().expect("6").unwrap());
//! assert_eq!("-Finely",   tokenizer.next().expect("7").unwrap());
//! assert_eq!("I",         tokenizer.next().expect("8").unwrap());
//! assert_eq!("hope",      tokenizer.next().expect("9").unwrap());
//! assert_eq!(None,        tokenizer.next());
//! ```

#![feature(core)]
#![feature(io)]
use std::vec::Vec;
use std::iter::Iterator;
use std::io;
use std::io::Read;

/// A tokenizer returning string slices from a reader
///
/// **Note:** The returned tokens are only valid until the next iteration
pub struct Tokenizer<R: Read> {
    separators: Vec<char>,
    chars: io::Chars<R>,
    current: String,
}

impl <R> Tokenizer<R> where R: Read {
    /// Creates a new tokenizer from a reader and a set of separating characters
    ///
    /// ```
    /// let separators = vec![' ', '\n', '\t'];
    /// let source: &str = "   Hello world\nHow do you do\t-Finely I hope";
    ///
    /// let mut tokenizer = token::Tokenizer::new(source.as_bytes(), separators);
    ///
    ///
    /// ```
    ///
    pub fn new(reader: R, separators: Vec<char>) -> Tokenizer<R> {
        Tokenizer {
            chars: reader.chars(),
            separators: separators,
            current: String::new(),
        }
    }
}

impl<'a, R: Read> Iterator for Tokenizer<R> {
    type Item = Result<&'a str, io::CharsError>;

    /// Returns a string slice of the next non-empty sequence that terminates
    /// in one of the specified separator strings
    fn next(&mut self) -> Option<Result<&str, io::CharsError>> {
        self.current.clear();
        'main: loop {
            match self.chars.next() {
                None => break,
                Some(res) => {
                    match res {
                        Ok(c) => {
                            // Check for token terminators
                            for &t in self.separators.iter() {
                                if c == t {
                                    if !self.current.is_empty() {
                                        return Some(Ok(self.current.as_slice()));
                                    }
                                    continue 'main; // No token: ignore
                                }
                            }

                            // Just add the char
                            self.current.push(c);
                        }

                        Err(e) => return Some(Err(e)),
                    }
                }
            }
        }
        // Handle leftover chars
        if !self.current.is_empty() {
            Some(Ok(self.current.as_slice()))
        } else {
            None // No more chars left
        }
    }

    /// Uses the underlying reader's size hint
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chars.size_hint()
    }
}

/// A structure for iteratively splitting stringy things into sentences
pub struct SentenceSplitter<'a, R: Read> {
    tokenizer: Tokenizer<R>,
    terminators: Vec<&'a str>,
    current: String,
    quote_types: Vec<&'a str>,
}

impl <'a, R: Read> SentenceSplitter<'a, R> {

    /// Creates a new sentence-splitting iterator
    ///
    /// ```
    /// let separators  = vec![' ', '\n', '\t', '\r'];
    /// let terminators = vec![".", "!", "?"];
    /// let quote_types = vec![]; // For when whole lines are quoted
    ///
    /// let text = "I walked down the road.\n\"What did he say\", she asked me.\n\"Nothing\", I replied, and continued walking...\nit wasn't any of my business.\nOr was it?";
    ///
    /// let mut tokenizer = token::Tokenizer::new(text.as_bytes(), separators);
    /// let mut splitter = token::SentenceSplitter::new(
    ///     tokenizer, terminators, quote_types
    /// );
    ///
    /// assert_eq!("I walked down the road.",
    ///     splitter.next().expect("1").unwrap());
    /// assert_eq!("\"What did he say\", she asked me.",
    ///     splitter.next().expect("2").unwrap());
    /// assert_eq!("\"Nothing\", I replied, and continued walking... \
    ///     it wasn't any of my business.",
    ///     splitter.next().expect("3").unwrap());
    /// assert_eq!("Or was it?", splitter.next().expect("3").unwrap());
    /// assert_eq!(None, splitter.next());
    /// ```
    pub fn new(source: Tokenizer<R>, terminators: Vec<&'a str>,
        quote_types: Vec<&'a str>) -> SentenceSplitter<'a, R>
    {
        SentenceSplitter{
            tokenizer: source,
            current: String::new(),
            terminators: terminators,
            quote_types: quote_types
        }
    }
}

impl <'a, 'b, R: Read> Iterator for SentenceSplitter<'a, R> {
    type Item = Result<&'b str, io::CharsError>;

    /// Returns the next sentence
    fn next(&mut self) -> Option<Result<&str, io::CharsError>> {
        self.current.clear();
        let mut quote = "";
        'main: loop {
            match self.tokenizer.next() {
                Some(res) => {
                    match res {
                        Ok(s) => {
                            self.current.push_str(s);

                            // Inside a quote
                            if !quote.is_empty() {
                                if s.ends_with(quote) {
                                    return Some(Ok(self.current.as_slice()))
                                }
                            }

                            // Not inside a quote
                            else {
                                // Check whether a quote is starting
                                for &qt in self.quote_types.iter() {
                                    if s.starts_with(qt) {
                                        if s.ends_with(qt) { // It can end again
                                            return Some(Ok(self.current.as_slice()));
                                        }
                                        quote = qt;
                                        self.current.push_str(" ");
                                        continue 'main;
                                    }
                                }

                                // Check whether the token is ending normally
                                // It ends in a terminating character
                                if s.ends_with("..") { // Continue thought trails
                                    self.current.push_str(" ");
                                    continue;
                                }
                                for &t in self.terminators.iter() {
                                    if s.ends_with(t) {
                                        return Some(Ok(self.current.as_slice()));
                                    }
                                }
                            }
                            // SPAAAAAAAAACE
                            self.current.push_str(" ");
                        }

                        Err(e) => return Some(Err(e)),
                    }
                },
                None => {
                    if self.current.len() != 0 {
                        return Some(Ok(self.current.as_slice()));
                    } else {
                        return None;
                    }
                }
            }
        }
    }
    /// Uses the underlying tokenizer's size hint
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.tokenizer.size_hint()
    }
}