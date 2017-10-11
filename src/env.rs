use ast;
pub enum ITypes {
        Var(String),
        Symbol(String),
        Function,
        Nil,
        EmptyList
}


static keywords: [&str; 6] =  ["define", "car", "cdr", "lambda", "nil", "empty-list"];



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
