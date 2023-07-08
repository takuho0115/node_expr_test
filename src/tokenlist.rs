use std::{iter::{Peekable, Enumerate}, str::Chars};

#[path="./token.rs"]
mod token;
use token::*;
pub struct TokenList{
	pub original: &'static str,
	pub list: Vec<Token>,
	pub current: usize,
}

impl TokenList{
	pub fn new(s: &'static str)->Self{
	 	Self { original: s, list: Vec::new(), current: 0 }.tokenize()
	}

	fn tokenize(mut self)->Self{
		let mut chars = self
			.original
			.chars()
			.enumerate()
			.peekable();
		
		loop {
			let current = chars.next();
			if current.is_none(){
				break;
			}
			let (i, c) = current.unwrap();
			if c.is_whitespace(){
				continue;
			}

			if "+-*/()".contains(c) {
				self.list.push(Token::new(TokenKind::TK_RESERVED, &c, &i));
				continue;
			}

			if !c.to_digit(10).is_none(){
				let mut tok = Token::new(TokenKind::TK_NUM, &c, &i);
				tok.val = Some(TokenList::read_num(&c, &mut chars));
				self.list.push(tok);
				continue;
			}
			self.at_error(i, "トークナイズできません。");
			panic!("トークナイズできません。");
			
		}
		self.list.push(Token::new(TokenKind::TK_EOF, &'\0', &chars.count()));
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
		while !iter.peek().is_none() {
			let (_i, p) = iter.peek().unwrap();
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
		self.current += 1;
		self.list.get(self.current).cloned()
	}
}