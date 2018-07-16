use std::fmt;
use std::result;
use tokenizer;
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpType {
    Identifier(String),
    Exp(Vec<SExpType>),
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
                        if parsing_sexp {
                            exp_vec.push(SExpType::Identifier(s.clone()))
                        } else {
                            identifier_only = Ok(SExpType::Identifier(s.clone()))
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
