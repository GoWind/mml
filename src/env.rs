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


pub fn is_atom(exp: &String) -> bool {
    exp.char_indices().count() > 1 && exp.chars().next() == Some(':')
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
            if is_atom(name) {
             // I have to do this nonsense because i really dont get lifetimes or borrow well
             // and rust is throwing an error if there is  a mutable and immutable borrow in the same place
             // like if env.get(name).is_some { } else { env.insert(name); env.get(name)
             let g = IType::Atom(name.clone());
             env.insert(name.clone(), Var::Bound(g));
             match env.get(name).unwrap() {
                &Var::Bound(ref k) => { Ok(k)}
                _ => { Err("This should never be hit")}
             }
            } else {
                let k = env.get(name);
                if k.is_some() {
                    // this is fuckall. why can i not say, get k.get_type(Var::Bound)
                    // or some easier method of getting the right subtype from an algebraic type
                    // without a match 
                    match *k.unwrap() {
                        Var::Bound(ref s) => { Ok(s.clone())}
                        Var::UnBound => { Err("unbound variable")}
                    }
                } else {
                    Err("no such variable defined")
                }
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
        {
        let tok_stream = tokenizer::parse_string(&"a".to_string());
        let ast = ast::stream_to_ast(&tok_stream).unwrap();
        let val = env::eval(&mut env, &ast);
        assert_eq!(val.is_ok(), true);
        }
        let tok_stream2 = tokenizer::parse_string(&":a".to_string());
        let ast2 = ast::stream_to_ast(&tok_stream2).unwrap();
        let val2 = env::eval(&mut env, &ast2);
        assert_eq!(val2.is_ok(), true);
        assert_eq!(true,
                    match val2.unwrap() {
                                &env::IType::Atom(ref atomstring) => {*atomstring == String::from(":a") }
                                _ => {false}
                            
                    });


        

    }
}
