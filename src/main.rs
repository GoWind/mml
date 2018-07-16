extern crate mml;

use mml::{ast, env, tokenizer};
use std::io;

fn main() -> io::Result<()> {
    let mut lisp_env = env::make_env();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim() == "quit".to_string() {
            println!("quitting the program");
            break;
        } else {
            let tok_stream = tokenizer::parse_string(&input);
            let ast = ast::stream_to_ast(&tok_stream).unwrap();
            let result = env::eval(&mut lisp_env, &ast);
            println!("{:?}", result);
        }
    }

    Ok(())
}
