#[derive(PartialEq,Clone,Debug)]
pub enum TokenKind {
	TkReserved,
	TkNum,
	TkEof,
}

#[derive(Clone, Debug)]
pub struct Token{
	pub kind: TokenKind,
	pub val: Option<usize>,
	pub str: Option<String>,
	pub pos: Option<usize>,
}

impl Token{
	pub fn new(kind: TokenKind, str: &impl ToString, pos: &usize)->Self
	{
		Token { kind: kind, val: None, str: Some(str.to_string()), pos:Some(*pos) }
	}

	pub fn consume(&self, op:&impl ToString)->bool{
		self.kind == TokenKind::TkReserved && self.str == Some(op.to_string())
	}

	pub fn expect(&self, op:&impl ToString){
		if !self.consume(op) {
			panic!("'{}'ではありません", op.to_string());
		}
	}

	pub fn expect_number(&self)->usize{
		if self.kind != TokenKind::TkNum{
			panic!("数ではありません");
		}
		self.val.unwrap()
	}

	pub fn at_eof(&self)->bool{
		self.kind == TokenKind::TkEof
	}
}