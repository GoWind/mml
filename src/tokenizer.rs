#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
	o_brace,
	c_brace,
	Identifier(String)
}	


pub fn parse_string(input: &String) -> Vec<TokenType> {
	let mut tokens = Vec::new();
	let mut current_identifier = String::new();
	for c in input.chars() {
		if c == '(' {
			if current_identifier.len() != 0 {
					tokens.push(TokenType::Identifier(current_identifier.clone()));
					current_identifier.clear();
			}
			tokens.push(TokenType::o_brace);
		}	else if c == ')' {
			
			if current_identifier.len() != 0 {
					tokens.push(TokenType::Identifier(current_identifier.clone()));
					current_identifier.clear();
			}
			tokens.push(TokenType::c_brace);
		}
		else if c.is_whitespace() {
			if current_identifier.len() != 0 {
					tokens.push(TokenType::Identifier(current_identifier.clone()));
					current_identifier.clear();
			}
		} else {
				current_identifier.push(c);
		}
		
			

	}
    if current_identifier.len() != 0 {
        tokens.push(TokenType::Identifier(current_identifier.clone()));
        current_identifier.clear();
    }
	tokens
}
