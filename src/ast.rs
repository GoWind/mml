use std::fmt;
use std::option;
use std::result;
use tokenizer;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpType {
    Identifier(String),
    Number(i64),
    Exp(Vec<SExpType>),
}

impl SExpType {
    pub fn get_exp(&self) -> Option<&Vec<SExpType>> {
        match *self {
            SExpType::Exp(ref sub_exp) => Some(sub_exp),
            _ => None,
        }
    }
    pub fn is_exp(&self) -> bool {
        match *self {
            SExpType::Exp(_) => true,
            _ => false,
        }
    }

    pub fn is_identifier(&self) -> bool {
        match *self {
            SExpType::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn get_identifier_name(&self) -> Option<String> {
        match *self {
            SExpType::Identifier(ref name) => Some(name.clone()),
            _ => None,
        }
    }
    pub fn len(&self) -> Option<usize> {
        match *self {
            SExpType::Exp(ref sub_exp_vec) => Some(sub_exp_vec.len()),
            _ => None,
        }
    }
}

impl fmt::Display for SExpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SExpType::Identifier(ref s) => write!(f, "{}", s.to_string()),
            SExpType::Exp(ref v) => {
                write!(f, "{}", "(");
                for (_, item) in v.iter().enumerate() {
                    write!(f, " {}", format!("{}", item.to_string()));
                }
                write!(f, ")")
            }
            SExpType::Number(no) => write!(f, "{}", no.to_string()),
        }
    }
}

pub fn is_identifier(sexp: &SExpType) -> bool {
    match sexp {
        SExpType::Identifier(_) => true,
        _ => false,
    }
}

pub fn stream_to_ast(tokenv: &Vec<tokenizer::TokenType>) -> Result<SExpType, &'static str> {
    let mut index: usize = 0;
    make_ast(tokenv, &mut index)
}

pub fn make_ast(
    tokenv: &Vec<tokenizer::TokenType>,
    sindex: &mut usize,
) -> Result<SExpType, &'static str> {
    if tokenv.len() == 0 {
        Err("token stream is empty")
    } else {
        let mut parsing_sexp = false;
        let mut exp_parsed = false;
        let mut found_exp = false;
        let mut identifier_only = Ok(SExpType::Identifier("".to_string()));
        let mut exp_vec = Vec::new();
        if *sindex == tokenv.len() {
            Err("reached end of stream")
        } else {
            while *sindex < tokenv.len() {
                match &tokenv[*sindex] {
                    &tokenizer::TokenType::o_brace => {
                        if !parsing_sexp {
                            parsing_sexp = true;
                            found_exp = true;
                        } else {
                            let m = make_ast(tokenv, sindex);
                            match m {
                                Ok(sub_exp) => {
                                    exp_vec.push(sub_exp);
                                }
                                Err(s) => {
                                    return Err(s);
                                }
                            }
                        }
                    }
                    &tokenizer::TokenType::c_brace => {
                        if !parsing_sexp {
                            panic!(
                                "found a closing brace without an opening brace at index {}",
                                *sindex
                            )
                        }
                        if exp_vec.is_empty() {
                            return Err("found an empty expression");
                        } else {
                            exp_parsed = true;
                            break;
                        }
                    }

                    &tokenizer::TokenType::Identifier(ref s) => {
                        // instead of pushing out an identifier here, check if this is a number
                        // or a string and then push the right type into the ast
                        let copy_of_s = s.clone();
                        let maybeNumber = copy_of_s.parse::<i64>();
                        if maybeNumber.is_ok() {
                            if parsing_sexp {
                                exp_vec.push(SExpType::Number(maybeNumber.unwrap()))
                            } else {
                                identifier_only = Ok(SExpType::Number(maybeNumber.unwrap()))
                            }
                        } else {
                            if parsing_sexp {
                                exp_vec.push(SExpType::Identifier(s.clone()))
                            } else {
                                identifier_only = Ok(SExpType::Identifier(s.clone()))
                            }
                        }
                    }
                }
                *sindex += 1;
            }
            if found_exp && !exp_parsed {
                panic!("reached end of stream before finding a closing parenthesis");
            }
            if parsing_sexp {
                Ok(SExpType::Exp(exp_vec))
            } else {
                identifier_only
            }
        }
    }
}
