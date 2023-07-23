use std::{iter::{Peekable, Enumerate}, str::Chars};

#[path="./token.rs"]
mod token;
use token::*;

#[derive(Debug)]
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
		let is_oparator = |c: &char| "+-*/();".contains(*c);
		let is_comparator = |c: &char| "!=<>".contains(*c);
		let is_alphabets = |c: &char| String::from_utf8((b'A'..=b'z').collect()).unwrap().contains(*c);
		let is_digit = |c: &char| !c.to_digit(10).is_none();
		
		// while chars.next() != None
		'o:while let Some((i, c)) = chars.next() {
			if c.is_whitespace(){
				continue;
			}
			for reader in [is_oparator,is_comparator,is_alphabets]{
				if reader(&c){
					let mut tok = Token::new(TokenKind::TkReserved, &c, &i);
					tok.str = Some(Self::read_ahead(&c, &mut chars, reader));
					self.list.push(tok);
					continue 'o;
				}
			}
			if is_digit(&c){
				let mut tok = Token::new(TokenKind::TkNum, &c, &i);
				tok.val = Some(Self::read_ahead(&c, &mut chars, is_digit)
					.parse::<usize>().expect("out of range"));
				self.list.push(tok);
				continue;
			}
			self.at_error(i, "トークナイズできません。");
		}

		self.list.push(Token::new(TokenKind::TkEof, &"\0", &chars.count()));
		self
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
		self.current = 
		if self.current.is_none(){
			Some(0)
		}else{
			Some(self.current.unwrap() + 1)
		};

		self.list.get(self.current.unwrap()).cloned()
	}
}
