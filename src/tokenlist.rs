use std::{iter::{Peekable, Enumerate}, str::Chars};

#[path="./token.rs"]
mod token;
use token::*;
pub struct TokenList{
	pub original: String,
	pub list: Vec<Token>,
	pub current: Option<usize>,
}

impl TokenList{
	pub fn new(s: impl Into<String>)->Self{
	 	Self { original: s.into(), list: Vec::new(), current: None }.tokenize()
	}

	fn tokenize(mut self)->Self{
		let mut chars = self.original.chars().enumerate().peekable();
		
		// while chars.next() != None
		while let Some(current) = chars.next() {
			let (i, c) = current;
			if c.is_whitespace(){
				continue;
			}

			if "+-*/()".contains(c) {
				self.list.push(Token::new(TokenKind::TkReserved, &c, &i));
				continue;
			}

			if !c.to_digit(10).is_none(){
				let mut tok = Token::new(TokenKind::TkNum, &c, &i);
				tok.val = Some(TokenList::read_num(&c, &mut chars));
				self.list.push(tok);
				continue;
			}
			self.at_error(i, "トークナイズできません。");
		}

		self.list.push(Token::new(TokenKind::TkEof, &'\0', &chars.count()));
		self
	}

	fn at_error(&self, pos: usize, e_message: &str){
		println!("{}", self.original);
		println!("{}^ {}", " ".repeat(pos), e_message);
		panic!("{}", e_message);
	}

	fn read_num(c:&char, iter:&mut Peekable<Enumerate<Chars>>)->usize{
		let mut join_str = String::new();
		join_str.push(*c);
		while let Some((_i, p)) = iter.peek() {
			if p.to_digit(10).is_none(){
				break;
			}
			join_str.push(*p);
			iter.next();
		}
		join_str.parse::<usize>().unwrap()
	}
}

impl Iterator for TokenList {
	type Item = Token;
	fn next(&mut self) -> Option<Self::Item> {
		self.current = 
		if self.current.is_none(){
			Some(0)
		}else{
			Some(self.current.unwrap() + 1)
		};
		
		self.list.get(self.current.unwrap()).cloned()
	}
}