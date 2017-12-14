use ast;
use tokenizer;
use std::collections::HashMap;
use std::option;
use std::result;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum IType {
    Atom(String),
    Function,
    List(Vec<IType>),
    QuotedList(ast::SExpType),
    True,
    False,
    Nil
}

// use this for quote
pub fn ast_to_list(ast: &ast::SExpType) -> IType {
    match ast {
        &ast::SExpType::Identifier(ref name) => {IType::Atom(name.clone())}
        &ast::SExpType::Exp(ref v) => { let mut iv = Vec::new();
                                        for j in v {
                                            iv.push(ast_to_list(&j));
                                        }
                                        IType::List(iv)
                                     }

    }
}


pub fn create_env() -> HashMap<String, IType> {
    let mut k =  HashMap::new();
    k
}


pub fn is_symbol(exp: &ast::SExpType) -> bool {
    match exp {
        &ast::SExpType::Identifier(ref n) => { !is_atom(n)}
        _ => {false}
    }
}


pub fn truthy(st: &String) -> bool {
    match st.as_ref() {
        "True" => { true}
        "False" => { true}
        _ => {false}
    }
    
}

pub fn is_nil(st: &String) -> bool {
    match st.as_ref() {
        "Nil" => {true}
        _ => {false}
    }
}

pub fn is_atom(exp: &String) -> bool {
    exp.char_indices().count() > 1 && exp.chars().next() == Some(':')
}

pub fn is_quote(exp : &ast::SExpType) -> bool {
    get_first_term(exp) == "quote"
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
        ast::SExpType::Identifier(ref n) => { n.clone()}
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
                    Ok(Rc::clone(env.get(name).unwrap()))
                } else {
                    Err("undefined variable")
                }
            }
        }
        ast::SExpType::Exp(ref n) => { 
            match get_first_term(&n[0]).as_ref()  {
            "quote" => {  if n.len() != 2 {
                                   Err("incorrect number of arguments to quote. should be (quote sexp)")
                            } else {
                                Ok(Rc::new(IType::QuotedList(n[1].clone())))
                            }
                          
                       } // end of quote interpretation

            "cons" => {
                        if n.len() != 3 {
                            Err("incorrect number of arguments to cons")
                        } else {   // figure out the type of item of item and the list
                            let item = eval(env, &n[1]);
                            match item {
                                Ok(ref i) =>  {
                                                  let list = eval(env, &n[2]);
                                                  match list {
                                                     Ok(ref l) => {  let mut cons_list = Vec::new();
                                                                     cons_list.push(i);
                                                                     cons_list.push(l);
                                                                     Ok(Rc::new(IType::List(vec![*i, l])))
                                                                  }
                                                    Err(k) => { Err(k) }
                                                 }
                                              }
                                Err(k) => { Err(k) }
                            }
                        }
                      } // end of cons interpretation

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
        assert_eq!(true, env::is_atom(&String::from(":a")));
        assert_eq!(true, val2.is_ok());
        assert_eq!(true, env::truthy(&String::from("True")));
        assert_eq!(true, env::truthy(&String::from("False")));
        assert_eq!(false, env::truthy(&String::from("Frue")));
        let tok_stream_3 = tokenizer::parse_string(&"(define a :b)".to_string());
        let ast3 = ast::stream_to_ast(&tok_stream_3).unwrap();
        let v = env::eval(&mut env, &ast3);
        assert_eq!(Rc::try_unwrap(v.unwrap()), Ok(env::IType::Nil));
        let tok_stream_4 = tokenizer::parse_string(&"a".to_string());
        let ast4 = ast::stream_to_ast(&tok_stream_4).unwrap();
        let v = env::eval(&mut env, &ast4);
        assert_eq!(true, match v {
                            Ok(s) => { assert_eq!(*s, env::IType::Atom(":b".to_string())); true}
                            Err(g) =>{ false}
        });



    }

}
