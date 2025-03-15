# Monkey

An interpreter for the Monkey programming language, with guidance from the
book, [*Writing An Interpreter In Go*], by Thorsten Ball.

## Quickstart

Install the project using cargo:

```bash
cargo install --git https://github.com/nicdgonzalez/monkey
```

Then, simply execute `monkey` to start the REPL (Read-Evaluate-Print-Loop).

> [!NOTE]
> To exit the program, press <kbd>Ctrl</kbd>+<kbd>C</kbd> on your keyboard.

```console
$ monkey
Hello! Welcome to the Monkey programming language!
Feel free to type in commands.
>>> let x = 10;
Token { kind: Let, literal: "let" }
Token { kind: Identifier, literal: "x" }
Token { kind: Assign, literal: "=" }
Token { kind: Integer, literal: "10" }
Token { kind: Semicolon, literal: ";" }
```

## Roadmap

As of writing, I have completed the first 3 chapters of the book. Before moving
forward, I'm taking the time to rewrite each chapter, adding documentation and
making changes where I see fit now that I have a better idea of how everything
should be laid out.

Rewrite progress:

- [x] Lexer
- [ ] Parser
- [ ] Evaluator

[*writing an interpreter in go*]: https://interpreterbook.com/
