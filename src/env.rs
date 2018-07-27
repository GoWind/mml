use ast;
use std::collections::{HashMap, HashSet};
use std::option;
use std::rc::Rc;
use std::result;
use tokenizer;

#[derive(Debug, PartialEq)]
pub enum IType {
    Atom(String),
    Function(Rc<ast::SExpType>, Rc<ast::SExpType>, usize),
    List(Vec<Rc<IType>>),
    QuotedList(ast::SExpType),
    True,
    False,
    Nil,
}

impl IType {
  pub fn get_fn(&self) -> Option<&IType> {
    match *self {
      IType::Function(_, _, _) => Some(self),
      _  => None,
    }
  }
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

fn is_fn(f: &IType) -> bool {
  match f {
    IType::Function(_, _, _) => true,
    _ => false,
  }
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



fn substitute(body: &ast::SExpType, substitute_map: &HashMap<String, String>) ->Result<ast::SExpType, String>{
  let some_k = body.get_exp(); // return Vec<SExpType>
  if some_k.is_none() {
    return Err("cannot substitute on type Identifier".to_string());
  } else {
    let k = some_k.unwrap();
    let mut new_sub_exp : Vec<ast::SExpType> = Vec::new();

    for sub_exp in k {
        if sub_exp.is_identifier() {
          let ident_name = sub_exp.get_identifier_name().unwrap();
          if substitute_map.contains_key(&ident_name) {
            let replacement = ast::SExpType::Identifier(substitute_map.get(&ident_name).unwrap().to_string());
            new_sub_exp.push(replacement);
          } else {
            new_sub_exp.push(sub_exp.clone());
          }
        } else {
          let replaced_sub_exp = substitute(&sub_exp, substitute_map)?;
          new_sub_exp.push(replaced_sub_exp);
        }
    }
    return Ok(ast::SExpType::Exp(new_sub_exp));
  }
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
                "cdr" => {
                    if n.len() != 2 {
                        return Err("invalid no. or arguments to cdr");
                    } else {
                        let list = eval(env, &n[1]);
                        match list {
                            Ok(g) => match *g {
                                IType::List(ref k) => {
                                    if (*k).len() < 1 {
                                        Err("cannot cdr on empty list")
                                    } else if (*k).len() == 1 {
                                        return Ok(Rc::new(IType::Nil));
                                    } else {
                                        // create a new copy of the list excluding
                                        // the first item, stupid I know but ok
                                        // for a hobby implementation
                                        let new_k = &(*k)[1..];
                                        return Ok(Rc::new(IType::List(new_k.to_vec())));
                                    }
                                }
                                _ => {
                                    return Err("cannot cdr on list");
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
                        return Err("invalid number of arguments to lambda. Expected 3");
                    } else {
                        let lambda_args = &n[1];
                        if let ast::SExpType::Exp(args) = lambda_args {
                            //iter over all arguments and ensure that each of them is an identifier
                            for arg in args {
                                if !ast::is_identifier(&arg) {
                                    return Err(
                                        "cannot have a non-identifier as a formal arg in lambda",
                                    );
                                } else {
                                    continue;
                                }
                            }
                        } else {
                            return Err("lambda arguments must be a list");
                        }
                        let lambda_body = &n[2];
                        if let ast::SExpType::Exp(_) = lambda_body {
                            return Ok(Rc::new(IType::Function(
                                Rc::new(lambda_args.clone()),
                                Rc::new(lambda_body.clone()),
                                lambda_args.len().unwrap(),
                            )));
                        } else {
                            return Err("lambda body must be a function");
                        }
                    }
                }
              // it could be fn application
              _ => {
                     let func = eval(env, &n[0])?;
                     match *func {
                       IType::Function(ref formal_args_list, ref body, ref arity) => {
                         //evaluate the arguments
                         if n.len() -1 != *arity {
                           return Err("incorrect no. of args to fn");
                         }
                         let formal_args = formal_args_list.get_exp().unwrap();

                         let mut substitute_map : HashMap<String, String> = HashMap::new();
                         for (idx,arg) in formal_args.iter().enumerate() {
                           let evaluated_arg_value = eval(env, &n[idx+1])?;
                           //replace formal_args with arg from lambda application
                           let actual_arg_name = format!("formal{}{}",idx, arg.get_identifier_name().unwrap());
                           substitute_map.insert(arg.get_identifier_name().unwrap(), actual_arg_name.clone());
                           env.insert(actual_arg_name, evaluated_arg_value);
                         }
                         let substituted_body = substitute(&body, &substitute_map); 
                         if substituted_body.is_err() {
                          return Err("cannot do a lambdada");
                         }else {
                           let value = eval(env, &substituted_body.unwrap());
                           return value;
                         }

                     }
                     _ => { return Err("cannot apply non-function");}
                     }
              }
        }
    }
}
}
