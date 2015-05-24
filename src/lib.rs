// Copyright 2015 Jakob Lautrup Nysom

//! A simple string tokenizer, and a tokenizer of sentences based on it.
//! The tokenizer ignores all the given separator chars, and returns the
//! characters between as string slice
//!
//! # Examples
//!
//! This is what to expect when parsing a string (or input from a reader)
//!
//! ```
//! let separators = vec![' ', '\n', '\t', '\r'];
//! let source: &str = "    Hello world \n  How do you do\t-Finely I hope";
//!
//! let mut tokenizer = token::Tokenizer::new(source.as_bytes(), separators);
//! assert_eq!(Some("Hello"),  tokenizer.next().unwrap());
//! assert_eq!(Some("world"),  tokenizer.next().unwrap());
//! assert_eq!(Some("How"),     tokenizer.next().unwrap());
//! assert_eq!(Some("do"),      tokenizer.next().unwrap());
//! assert_eq!(Some("you"),     tokenizer.next().unwrap());
//! assert_eq!(Some("do"),      tokenizer.next().unwrap());
//! assert_eq!(Some("-Finely"), tokenizer.next().unwrap());
//! assert_eq!(Some("I"),       tokenizer.next().unwrap());
//! assert_eq!(Some("hope"),    tokenizer.next().unwrap());
//! assert_eq!(None,            tokenizer.next().unwrap());
//! ```

#![feature(io)]

use std::vec::Vec;
use std::iter::Iterator;
use std::io;
use std::io::Read;

/// A tokenizer returning string slices from a reader
pub struct Tokenizer<R: Read> {
    separators: Vec<char>,
    chars: io::Chars<R>,
    current: String,
}

impl <R> Tokenizer<R> where R: Read {
    /// Creates a new tokenizer from a reader and a set of separating characters
    ///
    /// ```
    /// let seps = vec![' ', '\n', '\t'];
    /// let source: &str = "   Hello world\nHow do you do\t-Finely I hope";
    ///
    /// let mut tokenizer = token::Tokenizer::new(source.as_bytes(), seps);
    /// ```
    ///
    pub fn new(reader: R, separators: Vec<char>) -> Tokenizer<R> {
        Tokenizer {
            chars: reader.chars(),
            separators: separators,
            current: String::new(),
        }
    }
    
    /// Returns a string slice of the next non-empty sequence that terminates
    /// in one of the specified separator strings
    pub fn next(&mut self) -> Result<Option<&str>, io::CharsError> {
        self.current.clear();
        for res in &mut self.chars {
            let c = try!(res);
            // Is `c` a separator?
            if self.separators.iter().any(|t| *t == c) {
                if !&self.current.is_empty() {
                    return Ok(Some(&self.current));
                }
            } else {
                // Just add the char
                self.current.push(c);
            }
        }
        // Handle leftover chars
        if !self.current.is_empty() {
            Ok(Some(&self.current))
        } else {
            Ok(None) // No more chars left
        }
    }
}

/// A structure for iteratively splitting stringy things into sentences
pub struct SentenceSplitter<'a, R: Read> {
    tokenizer: Tokenizer<R>,
    terminators: Vec<&'a str>,
    current: String,
    quotes: Vec<&'a str>,
}

impl <'a, R: Read> SentenceSplitter<'a, R> {

    /// Creates a new sentence-splitting iterator
    ///
    /// ```
    /// let separators  = vec![' ', '\n', '\t', '\r'];
    /// let terminators = vec![".", "!", "?"];
    /// let quotes = vec![]; // For when whole lines are quoted
    ///
    /// let text = "I walked down the road.\n\"What did he say\", she asked me.\n\"Nothing\", I replied, and continued walking...\nit wasn't any of my business.\nOr was it?";
    ///
    /// let mut tokenizer = token::Tokenizer::new(text.as_bytes(), separators);
    /// let mut splitter = token::SentenceSplitter::new(
    ///     tokenizer, terminators, quotes
    /// );
    ///
    /// assert_eq!(Some("I walked down the road."), splitter.next().unwrap());
    /// assert_eq!(Some("\"What did he say\", she asked me."),
    ///            splitter.next().unwrap());
    /// assert_eq!(Some("\"Nothing\", I replied, and continued walking... \
    ///                  it wasn't any of my business."),
    ///            splitter.next().unwrap());
    /// assert_eq!(Some("Or was it?"), splitter.next().unwrap());
    /// assert_eq!(None, splitter.next().unwrap());
    /// ```
    pub fn new(source: Tokenizer<R>, terminators: Vec<&'a str>,
               quotes: Vec<&'a str>) -> SentenceSplitter<'a, R>
    {
        SentenceSplitter{
            tokenizer: source,
            current: String::new(),
            terminators: terminators,
            quotes: quotes,
        }
    }

    /// Returns the next sentence
    pub fn next(&mut self) -> Result<Option<&str>, io::CharsError> {
        self.current.clear();
        let mut quote = "";
        loop {
            let s = match try!(self.tokenizer.next()) {
                Some(s) => s,
                None => {
                    if self.current.len() != 0 {
                        return Ok(Some(&self.current));
                    } else {
                        return Ok(None);
                    }
                }
            };
            self.current.push_str(s);

            // Inside a quote
            if !quote.is_empty() {
                if s.ends_with(quote) {
                    return Ok(Some(&self.current))
                } else {
                    self.current.push_str(" ");
                    continue;
                }
            }

            // Not inside a quote
            // Check to see if a quote is starting
            match self.quotes.iter().find(|q| s.starts_with(*q)) {
                Some(q) => {
                    if s.ends_with(q) { // It can end again
                        return Ok(Some(&self.current));
                    }
                    quote = q;
                    self.current.push_str(" ");
                    continue;
                }
                None => {}
            }

            // Check whether the token is ending normally
            // It ends in a terminating character
            if s.ends_with("..") {
                // Continue thought trails
                self.current.push_str(" ");
                continue;
            }
            if self.terminators.iter().any(|t| s.ends_with(*t)) {
                return Ok(Some(&self.current));
            }
            // SPAAAAAAAAACE
            self.current.push_str(" ");
        }
    }
}
