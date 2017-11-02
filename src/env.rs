use ast;
use tokenizer;
use std::collections::HashMap;
use std::option;
use std::result;
use std::rc::Rc;

#[derive(Debug)]
pub enum IType {
        Atom(String),
        Function,
        List(Vec<IType>),
        True,
        False,
        Nil
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

pub fn eval<'a>(env: &'a mut HashMap<String, Rc<IType>>, exp: &ast::SExpType) -> Result<Rc<IType>, &'static str> {
    match *exp {
        ast::SExpType::Identifier(ref name) => {
            if is_atom(name) {
                if env.contains_key(name) {
                    Ok(env.get(name).unwrap().clone())
                } else {
                    env.insert(name.clone(), Rc::new(IType::Atom(name.clone())));
                    Ok(env.get(name).unwrap().clone())
                }
            } else {
                // see if the variable is define
                if env.contains_key(name) {
                    Ok(env.get(name).unwrap().clone())
                } else {
                    Err("undefined variable")
                }
            }
        }
        ast::SExpType::Exp(ref n) => { match get_first_term(&n[0]).as_ref()  {
                                          "define" => { Err("define not implemented yet")}
                                          "list"   => {Err("list not implemented yet")}
                                          _ => {Err("not implemented yet")}
                                        }
                                      }
    }
}


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
        println!("output is {:?}", val2);
        assert_eq!(true, env::is_atom(&String::from(":a")));
        assert_eq!(true, val2.is_ok());


        

    }
}
