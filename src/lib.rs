//! # Monkey Interpreter
//!
//! An interpreter for the Monkey programming language—a custom programming
//! language from the book, *Writing an Interpreter in Go*, by Thorsten Ball.
//!
//! This implementation provides a lexer, parser, and evaluator for the Monkey
//! programming language.

mod evaluator;
mod lexer;
mod parser;

pub use lexer::Lexer;
