use std::{iter::{Peekable, Enumerate}, str::Chars};

#[derive(PartialEq,Clone, Copy)]
pub enum TokenKind {
	TK_RESERVED,
	TK_NUM,
	TK_EOF,
}

#[derive(Clone)]
pub struct Token{
	kind: TokenKind,
	val: Option<usize>,
	str: Option<char>,
}

impl Token {
	pub fn new(kind: TokenKind, str: &char, pos: &usize)->Self{
		Token { kind: kind, val: None, str: Some(*str) }
	}

	pub fn consume(&self, op:char)->bool{
		!(self.kind != TokenKind::TK_RESERVED || self.str.unwrap() != op)
	}

	pub fn expect(&self, op:char){
		if !self.consume(op) {
			panic!("'{}'ではありません", op);
		}
	}

	pub fn expect_number(&self)->usize{
		if self.kind != TokenKind::TK_NUM{
			panic!("数ではありません");
		}
		self.val.unwrap()
	}

	pub fn at_eof(&self)->bool{
		self.kind == TokenKind::TK_EOF
	}
}

struct TokenList{
	original: &'static str,
	list: Vec<Token>,
	current: usize,
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
		let mut current = chars.next();
		while !current.is_none() {
			let (i, c) = current.unwrap();
			if c.is_whitespace(){
				current = chars.next();
				continue;
			}

			if "+-*/()".contains(c) {
				self.list.push(Token::new(TokenKind::TK_RESERVED, &c, &i));
				current = chars.next();
				continue;
			}

			if !c.to_digit(10).is_none(){
				let mut tok = Token::new(TokenKind::TK_NUM, &c, &i);
				tok.val = Some(TokenList::read_num(&c, &mut chars));
				self.list.push(tok);
				current = chars.next();
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

