

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
	STR,
	NUM,
	BOOL,

	TYPE,
	
	ASSIGN,
	PLUS,
	MINUS,
	MULTIPLY,
	DIVIDE,

	PRINT,
	IF,
	ELSE,
	ELIF,

	IDENT,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub tok_type: TokenType,
	pub literal: String
}

impl Token {
	fn new(tokType: TokenType, lit: String) -> Token {
		Token { tok_type: tokType, literal: lit}
	}
}

pub fn tokenize(source: String) -> Vec<Token> {
	let mut _tokens: Vec<Token> = Default::default();
	let mut _current_tok: String = Default::default();
	

	for c in source.chars() {
		if c != ' ' && c != '\n' && c != '\r' && c != '\0' {
			_current_tok.insert(_current_tok.len(), c);
		}
		else {
			_tokens.push(eval_tok(_current_tok.clone()));
			_current_tok = Default::default();
		}
	}
	if _current_tok != "" {
		_tokens.push(eval_tok(_current_tok.clone()));
		_current_tok = Default::default();
	}

	_tokens
}

fn eval_tok(tok: String) -> Token {
	match tok.as_str() {

		"print" => Token::new(TokenType::PRINT, tok),
		"=" => Token::new(TokenType::ASSIGN, tok),

		_ => {
			if tok.parse::<f64>().ok().is_some() {
				Token::new(TokenType::NUM, tok)
			} else {
				Token::new(TokenType::IDENT, tok)
			}
		}
		
	}

}