# Monkey

## Introduction

![build_cmake](https://img.shields.io/badge/build-CMake-blue)

An interpreter for the Monkey programming language, written in C++.

Based on the book [*Writing An Interpreter In Go*](https://interpreterbook.com/)
by [Thorsten Ball](https://thorstenball.com/).

## Quickstart

How to build and run the Monkey REPL.

### Using CMake

```bash
cmake -B build -S .
make -C build
./build/monkey
```

### Example Program

> [!NOTE]
> To end interactive mode, execute `exit` or press <kbd>Ctrl</kbd>+<kbd>D</kbd>.

```monkey
Hello, nicdgonzalez! This is the Monkey programming language!
Feel free to type in commands.
>>> let five = 5;
Token(LET, "let")
Token(IDENTIFIER, "five")
Token(ASSIGN, "=")
Token(INTEGER, "5")
Token(SEMICOLON, ";")
>>> exit
```

## Bug/Feature Request
If you find a bug (program failed to run and/or gave undesired results) or you
just want to request a feature, kindly open a new issue
[here](https://github.com/nicdgonzalez/monkey/issues).

## Contributing

This project is primarily focused on learning, so I will likely not integrate
pull requests containing substantial code changes. However, feel free to fork
the project if you wish to implement significant changes!
