
mod tokenizer;
mod ast;
mod env;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works()  {
		use super::tokenizer;
		let g = tokenizer::parse_string(&"(+ ab)".to_string());
		assert_eq!(vec![tokenizer::TokenType::o_brace,
                        tokenizer::TokenType::Identifier("+".to_string()),
                        tokenizer::TokenType::Identifier("ab".to_string()),
                        tokenizer::TokenType::c_brace],
					g);
    }

    #[test]
    fn test_ast() {
        use super::ast;
        use super::tokenizer;;
        use super::env;
		let g = tokenizer::parse_string(&"(+ a)".to_string());
        let mut start_i: usize = 0;
        let m = ast::make_ast(&g, &mut start_i);
        assert_eq!(m.is_some(), true);
        let ast_vec = m.unwrap();
        assert_eq!(ast::SExpType::Exp(vec![ast::SExpType::Identifier("+".to_string()),
                         ast::SExpType::Identifier("a".to_string())])
                    ,ast_vec);
        assert_eq!(ast_vec.to_string(), "( + a)");
        let p = tokenizer::parse_string(&"a".to_string());
        assert_eq!(p.len() !=0, true);
        start_i = 0;
        let q = ast::make_ast(&p, &mut start_i);
        assert_eq!(q.is_some(), true);
        let ast_vec2 = q.unwrap();
        assert_eq!(ast::SExpType::Identifier("a".to_string())
                   ,ast_vec2);
 //       assert_eq!(env::interpret(ast_vec2, "a".to_string()));

    }
    
}

