use ast;
use std::collections::HashMap;
pub enum IType {
        Atom(String),
        Function,
        List(Vec<IType>),
        True,
        False,
        Nil
}




static keywords: [&str; 6] =  ["define", "car", "cdr", "lambda", "nil", "empty-list"];


pub fn interpret(env: &mut HashMap<String, IType>, form: ast::SExpType) -> HashMap<String, IType> {
    HashMap::new()
}

/*
pub fn interpret(forms: &Vec<ast::SExpType>) -> String {

    let mut k :usize = 0;
    while k < forms.len() {

     let mut g =   match &forms[k] {
            ast::SExpType::Identifier(ref s) => {s.clone()}
            _ => {"not implemented yet".to_string()}

    }
    k += 1;
    }
    g
}
*/
