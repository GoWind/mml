
#[cfg(test)]
mod tests {
    use env;
    use ast;
    use tokenizer;
    use std::rc::Rc;
    #[test]
    fn test_env() {
        let mut env = env::HashMap::new();
        env.insert("a".to_string(), Rc::new(env::IType::Atom(":hohoho".to_string())));
        env.insert(String::from("True"), Rc::new(env::IType::True));
        {
            let tok_stream = tokenizer::parse_string(&"a".to_string());
            let ast = ast::stream_to_ast(&tok_stream).unwrap();
            let val = env::eval(&mut env, &ast);
            assert_eq!(val.is_ok(), true);
        }
        let tok_stream2 = tokenizer::parse_string(&":a".to_string());
        let ast2 = ast::stream_to_ast(&tok_stream2).unwrap();
        let val2 = env::eval(&mut env, &ast2);
        assert_eq!(true, env::is_atom(&String::from(":a")));
        assert_eq!(true, val2.is_ok());
        assert_eq!(true, env::truthy(&String::from("True")));
        assert_eq!(true, env::truthy(&String::from("False")));
        assert_eq!(false, env::truthy(&String::from("Frue")));
        let tok_stream_3 = tokenizer::parse_string(&"(define a :b)".to_string());
        let ast3 = ast::stream_to_ast(&tok_stream_3).unwrap();
        let v = env::eval(&mut env, &ast3);
        assert_eq!(v.is_ok(), true);
        let ret_val = v.ok();
        assert_eq!(*ret_val.unwrap(), env::IType::Atom(":b".to_string()));
        let tok_stream_4 = tokenizer::parse_string(&"a".to_string());
        let ast4 = ast::stream_to_ast(&tok_stream_4).unwrap();
        let v = env::eval(&mut env, &ast4);
        assert_eq!(true, match v {
                            Ok(s) => { assert_eq!(*s, env::IType::Atom(":b".to_string())); true}
                            Err(g) =>{ false}
        });



    }

}
