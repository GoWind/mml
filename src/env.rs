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
    List(Vec<Rc<IType>>),
    QuotedList(ast::SExpType),
    True,
    False,
    Nil
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
        _     =>  {false}
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

pub fn eval(env: &mut HashMap<String, Rc<IType>>, exp: &ast::SExpType) -> Result<Rc<IType>, &'static str> {
    match *exp {
        ast::SExpType::Identifier(ref name) => {
            if is_atom(name) {
                Ok(Rc::new(IType::Atom(name.to_string())))
            } else {
                // see if the variable is define
                if env.contains_key(name) {
                    let val = env.get(name).unwrap();
                    Ok(Rc::clone(&val))
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
                                                     Ok(ref l) => {Ok(Rc::new(IType::List(vec![Rc::clone(i), Rc::clone(l)])))}
                                                    Err(k) => { Err(k) }
                                                 }
                                              }
                                Err(k) => { Err(k) }
                            }
                        }
                      } // end of cons interpretation

            "list"   => {  Err("list not implemented yet")}
            "define" => { if n.len() != 3 {
                            return Err("invalid number  of arguments passed to define");
                          }
                          if !is_symbol(&n[1]) {
                            Err("variable name not a symbol")
                          } else {
                            let var = &n[1];
                            let val = eval(env, &n[2]);
                            match val {
                                Ok(ref k) => { 
                                               env.insert(var.to_string(), Rc::clone(k));
                                               Ok(Rc::clone(k))}
                                Err(s)    => Err(s)
                            }
                          }
            }
                            
            _        => {Err("not implemented yet")}
        }
        }
    }

}
