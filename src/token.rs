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