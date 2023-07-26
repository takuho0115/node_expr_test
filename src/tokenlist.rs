use std::{iter::{Peekable, Enumerate}, str::Chars};

#[path="./token.rs"]
mod token;
use token::*;

#[derive(Debug)]
pub struct TokenList{
	pub original: String,
	pub list: Vec<Token>,
	pub current: usize,
}

impl TokenList{
	pub fn new(s: impl Into<String>)->Self{
	 	Self { original: s.into(), list: Vec::new(), current: 0 }
	}

	fn at_error(&self, pos: usize, e_message: &str){
		println!("{}", self.original);
		println!("{}^ {}", " ".repeat(pos), e_message);
		panic!("{}", e_message);
	}

	fn read_ahead<F>(c: &char, iter: &mut Peekable<Enumerate<Chars>>, f: F)->String
		where F:Fn(&char)->bool{
		let mut join_str = c.to_string();
		while let Some((_i, p)) = iter.peek() {
			if f(p){
				join_str.push(*p);
				iter.next();
			}else{
				break;
			}
		}
		join_str
	}
}

impl Iterator for TokenList {
	type Item = Token;
	fn next(&mut self) -> Option<Self::Item> {
		let is_oparator = |c: &char| "+-*/();".contains(*c);
		let is_comparator = |c: &char| "!=<>".contains(*c);
		let is_alphabets = |c: &char| String::from_utf8((b'A'..=b'z').collect()).unwrap().contains(*c);
		let is_digit = |c: &char| !c.to_digit(10).is_none();

		let mut chars = self.original.chars().enumerate().peekable();

		
		while let Some((i, c)) = chars.nth(self.current + 1) {
			self.current += 1;
			if c.is_whitespace(){
				continue;
			}
			for reader in [is_oparator,is_comparator,is_alphabets]{
				if reader(&c){
					let mut tok = Token::new(TokenKind::TkReserved, &c, &i);
					tok.str = Some(Self::read_ahead(&c, &mut chars, reader));
					return Some(tok);
				}
			}
			if is_digit(&c){
				let mut tok = Token::new(TokenKind::TkNum, &c, &i);
				tok.val = Some(Self::read_ahead(&c, &mut chars, is_digit)
					.parse::<usize>().expect("out of range"));
				return Some(tok);
			}
			self.at_error(i, "トークナイズできません。");
		}
		return Some(Token::new(TokenKind::TkEof, &"\0", &chars.count()));
	}
}
