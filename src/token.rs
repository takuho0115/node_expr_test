#[derive(PartialEq,Clone)]
pub enum TokenKind {
	TK_RESERVED,
	TK_NUM,
	TK_EOF,
}

#[derive(Clone)]
pub struct Token{
	pub kind: TokenKind,
	pub val: Option<usize>,
	pub str: Option<char>,
	pub pos: Option<usize>, 
}

impl Token {
	pub fn new(kind: TokenKind, str: &char, pos: &usize)->Self{
		Token { kind: kind, val: None, str: Some(*str), pos:Some(*pos) }
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
