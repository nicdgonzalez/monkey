# MonkeyLang

> [!WARN]\
> This project is still under development!

An interpreter for the Monkey programming language, written in Rust.

Based on the book, [*Writing An Interpreter In Go*] by [Thorsten Ball].

## ✨ Features

- You can expect all of the features up to Chapter 3.8 to be implemented.

### Roadmap

This is a list of things I'd like to work on, if time permits:

- [ ] Better error messages: I want to have clearer, more deliberate error
  messages.
- [ ] Unit tests: For better stability.
- [ ] Logging/tracing: Easier debugging.
- [ ] mdbook Documentation: To show examples of what can be done.
- [ ] Web application for writing code in the browser (makes it easier to share
  with people).
- [ ] Vim syntax highlighting: Free experience + for convenience.
- [ ] Code formatter: Just because it would be nice and convenient, and seems
  like it would have a lot of edge cases. Seems fun.
- [ ] Multi-line commands in REPL (functions, for example, have to be written
  in a single line right now; very inconvenient).
- [ ] Add `struct`s: I want to experiment with implementing at least one
  feature from scratch without any guidance from the book.

## 📦 Installation

Install using `cargo`:

```bash
cargo install --git https://github.com/nicdgonzalez/monkey.git
```

Then, execute `monkey` to start the REPL (Read-Evaluate-Print-Loop):

```console
$ monkey
Welcome to the Monkey programming language! Feel free to type in commands.
>>> let x = 4;
4
>>> let y = x + x;
8
>>> let add = fn(x, y) { return x + y };
function
>>> add(2, 2);
4
```

## 📖 Overview

This section will go over the project's internals, so other developers (or
future me) can more easily make changes/navigate through the codebase.

(Work in progress. The following is a rough draft.)

There are 3 steps when it comes to interpreting the language:

1. Lexical Analysis: Converts the user input into tokens.
1. Parsing: Takes the tokens and ensures they form proper statements.
1. Evaluation: Takes the statements and executes them.

What are tokens? What does it mean to be a proper statement? What does it mean
to execute a statement?

I think comparing it to spoken languages might help it make a bit more sense.

In the beginning, user input is just a series of random letters. The Lexer's
job is to perform Lexical Analysis on the input, which outputs "tokens".

The Lexer's job is essentially to take a series of letters, and break them
apart into words and punctuation. For example, this series of random letters,
"let five = 5;", can be split into several "words." For example, the "random"
series of letters "l", "e", and "t", can be put together to form the keyword
`let`. "f", "i", "v", and "e", can be put together to form the identifier
`five`, and so on.

Then, these tokens are interpreted by the parser, whose job is to ensure that
the incoming tokens appear in an order that makes sense, otherwise, you get a
syntax error. For example, the parser should accept the tokens: `Let`,
`Identifier`, `Assign`, `Value`, `Semicolon`, but it should reject, for
example, `Let` -> `True` -> `If` -> `Comma`.

Lastly, the Evaluator should take these statements, and do something with them.
For example, if we are greeted by a `Let` statement, we should store whatever
that value is somewhere in our environment so we can retrieve it later, as
opposed to just reading it and going "oh, okay. cool." If we get a `Return`
statement, stop executing the remaining statements in that block. And so on...

The program starts in `src/main.rs`, where we prepare for lexical analysis.

Currently, everything is split into 3 parts. The lexer is it's own type, and
needs to be instantiated first. The `Lexer` returns a type that implements
`Iterator` through the `tokens` method. This is passed to `Parser` during
construction. The parser is then passed to `Program`, which begins the parsing
process.

Tokens are lazily generated through the `Tokens` iterator. The iterator always
returns `Some(token)` (even if the token is invalid, in which case it returns
the `Illegal` token), or `None` if we reached the end of the file.

[*writing an interpreter in go*]: https://interpreterbook.com/
[thorsten ball]: https://thorstenball.com/
