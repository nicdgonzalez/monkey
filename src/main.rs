//! # Monkey REPL
//!
//! The implementation for Monkey's Read-Evaluate-Print-Loop (REPL).

use std::io::Write;

fn main() {
    let mut buffer = String::new();

    println!(
        "Hello! Welcome to the Monkey programming language!\n\
        Feel free to type in commands."
    );

    loop {
        print!(">>> ");

        std::io::stdout()
            .flush()
            .expect("failed to print full prompt");

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("failed to read from stdin");

        let lexer = monkey::Lexer::new(&buffer);

        for token in lexer.iter() {
            println!("{:?}", token);
        }

        buffer.clear();
    }
}
