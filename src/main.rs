mod ast;
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

use crate::environment::Environment;
use crate::evaluator::Evaluate;
use crate::lexer::Lexer;
use crate::parser::{Parse, Parser};
use crate::program::Program;

fn main() {
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
        let program = Program::parse(&mut parser)
            .expect("failed to parse program")
            .into_program_unchecked();

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
