A micromanual of lisp implementation in Rust

This is an attempt by me to learn two things at the same time

1. How to write an interpreter
2. Learning Rust

Code style and idioms will be pretty bad. But this is a WIP and I hope acts as a great
way to learn Rust and interpreters

Where to start

The eval function is implemented in env.rs. 
Tokenizer.rs implements the tokenization to convert from string -> tokens
ast.rs takes the token stream and then parses them into s-expressions that are then fed into the eval function

TODO: write a main module with a repl for playing around with the interpreter
