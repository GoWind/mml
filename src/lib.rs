#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod env;
mod realtests;
pub mod tokenizer;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::tokenizer;
        let g = tokenizer::parse_string(&"(+ ab)".to_string());
        assert_eq!(
            vec![
                tokenizer::TokenType::o_brace,
                tokenizer::TokenType::Identifier("+".to_string()),
                tokenizer::TokenType::Identifier("ab".to_string()),
                tokenizer::TokenType::c_brace,
            ],
            g
        );
    }

    #[test]
    fn test_ast() {
        use super::ast;
        use super::env;
        use super::tokenizer;;
        let g = tokenizer::parse_string(&"(+ a)".to_string());
        let mut start_i: usize = 0;
        let m = ast::stream_to_ast(&g);
        assert_eq!(m.is_ok(), true);
        let ast_vec = m.unwrap();
        assert_eq!(
            ast::SExpType::Exp(vec![
                ast::SExpType::Identifier("+".to_string()),
                ast::SExpType::Identifier("a".to_string()),
            ]),
            ast_vec
        );
        assert_eq!(ast_vec.to_string(), "( + a)");
        let p = tokenizer::parse_string(&"a".to_string());
        assert_eq!(p.len() != 0, true);
        let q = ast::stream_to_ast(&p);
        assert_eq!(q.is_ok(), true);
        let ast_vec2 = q.unwrap();
        assert_eq!(ast::SExpType::Identifier("a".to_string()), ast_vec2);

        let token_vec3 = tokenizer::parse_string(&"(+ a) (+ c d)".to_string());
        let ast_vec3 = ast::stream_to_ast(&token_vec3);
        let av3 = ast_vec3.unwrap();
        assert_eq!(
            ast::SExpType::Exp(vec![
                ast::SExpType::Identifier("+".to_string()),
                ast::SExpType::Identifier("a".to_string()),
            ]),
            av3
        );
        let token_vec4 = tokenizer::parse_string(&"(lambda (a) (car a))".to_string());
        let ast_vec4 = ast::stream_to_ast(&token_vec4);
        let av4 = ast_vec4.unwrap();
        let lambda_str = "lambda".to_string();
        // Doing this as I am getting the following error in rust if I  compare av4
        // against vec![Identifier(), Exp(vec![identifier(), identifier()])],
        let mut args_vec: Vec<ast::SExpType> = Vec::new();
        args_vec.push(ast::SExpType::Identifier("a".to_string()));
        let mut body_vec: Vec<ast::SExpType> = Vec::new();
        body_vec.push(ast::SExpType::Identifier("car".to_string()));
        body_vec.push(ast::SExpType::Identifier("a".to_string()));
        assert_eq!(
            ast::SExpType::Exp(vec![
                ast::SExpType::Identifier("lambda".to_string()),
                ast::SExpType::Exp(args_vec),
                ast::SExpType::Exp(body_vec),
            ]),
            av4
        );
    }

    #[test]
    fn test_env_from_lib() {
        use super::ast;
        use super::env;
        use super::tokenizer;
        let label_form = tokenizer::parse_string(&"(label a nil)".to_string());
        let m = ast::stream_to_ast(&label_form);
        assert_eq!(m.is_ok(), true);
        let empty_form = tokenizer::parse_string(&"".to_string());
        let empty_m = ast::stream_to_ast(&empty_form);
        assert_eq!(empty_m.is_err(), true);
        let label_bad_form = tokenizer::parse_string(&"(label a ())".to_string());
        let mb = ast::stream_to_ast(&label_bad_form);
        assert_eq!(mb.is_err(), true);
        let p = m.unwrap();
        assert_eq!(env::is_car(&p), false);
        let lambda_form = tokenizer::parse_string(&"(lambda (k x) (+ k x))".to_string());
        let g = ast::stream_to_ast(&lambda_form).unwrap();
        assert_eq!(env::is_lambda(&g), true);
    }
}
