mod environment;
mod evaluator;
mod expression;
mod lexer;
mod object;
mod parser;
mod precedence;
mod program;
mod statement;
mod token;

use std::io::{self, Write as _};
use std::{env, fs};

use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::program::Program;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::TRACE))
        .init();

    if env::args().len() == 2 {
        let input = fs::read_to_string(env::args().nth(1).unwrap()).expect("failed to read file");
        let mut env = Environment::default();
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer.tokens());
        let program = Program::parse(&mut parser).expect("failed to parse program");

        if !program.errors().is_empty() {
            _ = writeln!(io::stderr(), "errors:");

            for err in program.errors() {
                _ = writeln!(io::stderr(), "  {err}");
            }
        }

        // _ = writeln!(io::stdout(), "AST: {:#?}", program.statements());

        let value = program.evaluate(&mut env);
        _ = writeln!(io::stdout(), "{}", value);

        return;
    }

    let mut buffer = String::new();
    let mut env = Environment::default();

    _ = writeln!(
        io::stdout(),
        "Welcome to the Monkey programming language! Feel free to type in commands."
    );

    loop {
        _ = write!(io::stdout(), ">>> ");

        io::stdout().flush().expect("failed to print full prompt");

        io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let lexer = Lexer::new(buffer.to_owned());
        let mut parser = Parser::new(lexer.tokens());
        let program = Program::parse(&mut parser).expect("failed to parse program");

        if !program.errors().is_empty() {
            _ = writeln!(io::stderr(), "errors:");

            for err in program.errors() {
                _ = writeln!(io::stderr(), "  {err}");
            }
        }

        // _ = writeln!(io::stdout(), "AST: {:#?}", program.statements());

        let value = program.evaluate(&mut env);
        _ = writeln!(io::stdout(), "{}", value);

        buffer.clear();
    }
}
