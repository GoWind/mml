use ast;
use tokenizer;
use std::collections::HashMap;
use std::option;
use std::result;
pub enum IType {
        Atom(String),
        Function,
        List(Vec<IType>),
        True,
        False,
        Nil
}

pub enum Var {
    Bound(IType),
    UnBound
}


pub fn create_env() -> HashMap<String, IType> {
    let mut k =  HashMap::new();
    k
}


pub fn is_define(exp:  &ast::SExpType) -> bool {
    get_first_term(exp) == "define"
}

pub fn is_car(exp : &ast::SExpType) -> bool {
    get_first_term(exp) == "car"
}

pub fn is_cdr(exp : &ast::SExpType) -> bool {
    get_first_term(exp) == "cdr"
}

pub fn is_cons(exp : &ast::SExpType) -> bool {
    get_first_term(exp) == "cons"
}

pub fn is_list(exp : &ast::SExpType) -> bool {
    get_first_term(exp) == "list"
}

pub fn is_lambda(exp : &ast::SExpType) -> bool {
    get_first_term(exp) == "lambda"
}

pub fn get_first_term(exp: &ast::SExpType) -> String {
    match *exp {
        ast::SExpType::Exp(ref form) => {
            match (*form)[0] {
                ast::SExpType::Identifier(ref name) => { name.clone()}
                _ => { "".to_string()}
            }
        }
        _ => { "".to_string()}
    }
}

pub fn eval<'a>(env: &'a mut HashMap<String, Var>, exp: &ast::SExpType) -> Result<&'a IType, &'static str> {
    match *exp {
        ast::SExpType::Identifier(ref name) => 
        {
            let k = env.get(name);
            if k.is_some() {
                match *k.unwrap() {
                    Var::Bound(ref s) => { Ok(s.clone())}
                    Var::UnBound => { Err("unbound variable")}
                }
            } else {
                Err("no such variable defined")
            }
        }
        _ => { Err("have not implemented further")}
    }
}


#[cfg(test)]

mod tests {
    use env;
    use ast;
    use tokenizer;
    #[test]
    fn test_env() {
        let mut env = env::HashMap::new();
        env.insert("a".to_string(), env::Var::Bound(env::IType::Atom(":hohoho".to_string())));
        let tok_stream = tokenizer::parse_string(&"a".to_string());
        let ast = ast::stream_to_ast(&tok_stream).unwrap();
        let val = env::eval(&mut env, &ast);
        assert_eq!(val.is_ok(), true);



        

    }
}
