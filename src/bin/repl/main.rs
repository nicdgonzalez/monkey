use std::io::Write;

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

        let mut lexer = monkey::lexer::Lexer::new(&buffer);
        let mut parser = monkey::parser::Parser::new(&mut lexer);
        let program = parser.parse_program();

        if program.errors.len() > 0 {
            eprintln!("errors:");
            for error in program.errors.iter() {
                eprintln!("  {}", error);
            }
        }

        println!("{}", program);
        buffer.clear();
    }
}
