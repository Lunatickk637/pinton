use crate::interpreter::tokenizer;

use super::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
	Print(Box<Expr>),
	Identifier(String),
	Assignment(String, Box<Expr>),
	StrLiteral(String),
	NumLiteral(f64),
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Vec<Expr> {
	let mut expressions: Vec<Expr> = Default::default();

	let mut i: usize = 0;
	while i < tokens.len() {
		let token = tokens[i].clone();

		if token.tok_type == tokenizer::TokenType::PRINT {
			if i + 1 < tokens.len() {
				let next_tok: Token = tokens[i + 1].clone();

				match next_tok.tok_type {
					tokenizer::TokenType::NUM => 
						expressions.push(Expr::Print(Box::new(Expr::NumLiteral(next_tok.literal.parse::<f64>().unwrap())))),
					_ => ()
				}
			}
		
		}
		i += 1
	}
	

	expressions
}