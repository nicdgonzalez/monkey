use std::io::Write;

use monkey::token::TokenKind;

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
        let mut token = lexer.next_token();

        while token.kind != TokenKind::EndOfFile {
            println!("{:?}", token);
            token = lexer.next_token();
        }

        buffer.clear();
    }
}
