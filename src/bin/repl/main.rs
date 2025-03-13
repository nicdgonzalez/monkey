use std::io::Write;

use monkey::{evaluator, Lexer, Program};
use monkey::{Parse, Parser};

fn main() {
    let mut buffer = String::new();

    println!(
        "Hello! Welcome to the Monkey programming language!\n\
        Feel free to type in commands."
    );

    loop {
        print!(">> ");

        std::io::stdout()
            .flush()
            .expect("failed to print full prompt");

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let mut lexer = Lexer::new(&buffer);
        let mut parser = Parser::new(&mut lexer);
        let program: Program = Program::parse(&mut parser)
            .expect("failed to parse program")
            .into();

        if program.errors.len() > 0 {
            eprintln!("errors:");
            for error in program.errors.iter() {
                eprintln!("  {}", error);
            }
        }

        let value = evaluator::eval(program.into());
        println!("{}", value);

        buffer.clear();
    }
}
