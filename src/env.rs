use ast;
use std::collections::HashMap;
use std::option;
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
    Unbound
}




static keywords: [&str; 6] =  ["define", "car", "cdr", "lambda", "nil", "empty-list"];


pub fn is_define(exp:  &ast::SExpType) -> bool {
    match exp {
        &ast::SExpType::Exp(ref form)  => {
            match &form[0] {
                &ast::SExpType::Identifier(ref name) => { *name == "define".to_string()}
                _ => { false}
            }
        }
        _ => {false}
    }
}

pub fn is_car(exp : &ast::SExpType) -> bool {
    match exp {
        &ast::SExpType::Exp(ref form)  => {
            match &form[0] {
                &ast::SExpType::Identifier(ref name) => { *name == "car".to_string()}
                _ => { false}
            }
        }
        _ => {false}
    }
}


