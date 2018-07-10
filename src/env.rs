use ast;
use std::collections::{HashMap, HashSet};
use std::option;
use std::rc::Rc;
use std::result;
use tokenizer;

#[derive(Debug, PartialEq)]
pub enum IType {
    Atom(String),
    Function(Rc<ast::SExpType>, Rc<ast::SExpType>),
    List(Vec<Rc<IType>>),
    QuotedList(ast::SExpType),
    True,
    False,
    Nil,
}

const KEYWORDS: [&'static str; 13] = [
    "false", "true", "nil", "quote", "car", "cdr", "cons", "equal", "atom", "cond", "label",
    "lambda", "defun",
];
lazy_static! {
    static ref KEYWORD_SET: Vec<String> = KEYWORDS.iter().clone().map(|x| x.to_string()).collect();
}

fn is_keyword(k: &String) -> bool {
    KEYWORD_SET.contains(k)
}

pub fn make_env() -> HashMap<String, Rc<IType>> {
    let v = HashMap::new();
    v
}
pub fn is_symbol(exp: &ast::SExpType) -> bool {
    match exp {
        &ast::SExpType::Identifier(ref n) => !is_keyword(n),
        _ => false,
    }
}

pub fn truthy(st: &String) -> bool {
    match st.as_ref() {
        "True" => true,
        "False" => true,
        _ => false,
    }
}

pub fn is_nil(st: &String) -> bool {
    match st.as_ref() {
        "Nil" => true,
        _ => false,
    }
}

pub fn is_atom(exp: &String) -> bool {
    !KEYWORD_SET.contains(exp)
}

pub fn is_quote(exp: &ast::SExpType) -> bool {
    get_first_term(exp) == "quote"
}

pub fn is_car(exp: &ast::SExpType) -> bool {
    get_first_term(exp) == "car"
}

pub fn is_cdr(exp: &ast::SExpType) -> bool {
    get_first_term(exp) == "cdr"
}

pub fn is_cons(exp: &ast::SExpType) -> bool {
    get_first_term(exp) == "cons"
}

pub fn is_list(exp: &ast::SExpType) -> bool {
    get_first_term(exp) == "list"
}

pub fn is_lambda(exp: &ast::SExpType) -> bool {
    get_first_term(exp) == "lambda"
}

pub fn get_first_term(exp: &ast::SExpType) -> String {
    match *exp {
        ast::SExpType::Exp(ref form) => match (*form)[0] {
            ast::SExpType::Identifier(ref name) => name.clone(),
            _ => "".to_string(),
        },
        ast::SExpType::Identifier(ref n) => n.clone(),
    }
}

pub fn eval(
    env: &mut HashMap<String, Rc<IType>>,
    exp: &ast::SExpType,
) -> Result<Rc<IType>, &'static str> {
    match *exp {
        ast::SExpType::Identifier(ref name) => {
            // if name is True, False or Nil, return that
            // if it is a variable, return the value of the variable
            // else if name is not a keyword, return Atom(name
            // return error other wise
            let v = if is_keyword(name) {
                match name.as_str() {
                    "true" => Ok(Rc::new(IType::True)),
                    "false" => Ok(Rc::new(IType::False)),
                    "nil" => Ok(Rc::new(IType::Nil)),
                    _ => Err("cannot eval keyword"),
                }
            } else if env.contains_key(name) {
                let val = env.get(name).unwrap();
                Ok(Rc::clone(val))
            } else {
                Ok(Rc::new(IType::Atom(name.to_string())))
            };
            return v;
        }
        ast::SExpType::Exp(ref n) => {
            match get_first_term(&n[0]).as_ref() {
                "quote" => {
                    if n.len() != 2 {
                        Err("incorrect number of arguments to quote. should be (quote sexp)")
                    } else {
                        Ok(Rc::new(IType::QuotedList(n[1].clone())))
                    }
                } // end of quote interpretation

                "cons" => {
                    if n.len() != 3 {
                        Err("incorrect number of arguments to cons")
                    } else {
                        // figure out the type of item of item and the list
                        let item = eval(env, &n[1]);
                        match item {
                            Ok(ref i) => {
                                let list = eval(env, &n[2]);
                                match list {
                                    Ok(ref l) => {
                                        Ok(Rc::new(IType::List(vec![Rc::clone(i), Rc::clone(l)])))
                                    }
                                    Err(k) => Err(k),
                                }
                            }
                            Err(k) => Err(k),
                        }
                    }
                } // end of cons interpretation

                "list" => {
                    if n.len() < 2 {
                        return Err("cannot make a list without arguments");
                    } else {
                        let mut m: Vec<Rc<IType>> = Vec::new();
                        for i in &n[1..] {
                            let item = eval(env, i);
                            match item {
                                Ok(g) => {
                                    m.push(g);
                                }
                                Err(k) => {
                                    return Err(k);
                                }
                            }
                        }
                        return Ok(Rc::new(IType::List(m)));
                    }
                }
                "car" => {
                    if n.len() != 2 {
                        return Err("invalid no. of arguments to car");
                    } else {
                        let arg = eval(env, &n[1]);
                        match arg {
                            Ok(g) => match *g {
                                IType::List(ref k) => {
                                    if k.len() == 0 {
                                        return Err("cannot car on empty list");
                                    } else {
                                        return Ok(Rc::clone(&k[0]));
                                    }
                                }
                                _ => {
                                    return Err("argument is not a list");
                                }
                            },
                            Err(err_msg) => {
                                return Err(err_msg);
                            }
                        }
                    }
                }
                "label" => {
                    if n.len() != 3 {
                        return Err("invalid number  of arguments passed to label");
                    }
                    if !is_symbol(&n[1]) {
                        Err("variable name not a symbol")
                    } else {
                        let var = &n[1];
                        let val = eval(env, &n[2]);
                        match val {
                            Ok(ref k) => {
                                env.insert(var.to_string(), Rc::clone(k));
                                Ok(Rc::clone(k))
                            }
                            Err(s) => Err(s),
                        }
                    }
                }
              "lambda" => {
                  if n.len() != 3 {
                    println!("got arguments for lambda {:?}", n);
                    return Err("invalid number of arguments to lambda. Expected 3");
                  } else {
                    let lambda_args = &n[1];  
                    if let ast::SExpType::Exp(args) = lambda_args {
                        //iter over all arguments and ensure that each of them is an identifier
                        for arg in args {
                          if !ast::is_identifier(&arg) {
                            return Err("cannot have a non-identifier as a formal arg in lambda");
                          } else {
                            continue;
                          }
                        }
                    } else {
                      return  Err("lambda arguments must be a list");
                    }
                    let lambda_body = &n[2];
                    if let ast::SExpType::Exp(_) = lambda_body {
                      return Ok(Rc::new(IType::Function(Rc::new(lambda_args.clone()), Rc::new(lambda_body.clone()))));
                    } else {
                      return Err("lambda body must be a function");
                    }
                  }
              }

                _ => Err("not implemented yet"),
            }
        }
    }
}
