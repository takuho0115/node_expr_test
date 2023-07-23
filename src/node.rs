use std::iter::Peekable;
#[path="./tokenlist.rs"]
pub mod tokenlist;
use tokenlist::*;
#[derive(PartialEq,Clone, Copy, Debug)]
pub enum NodeKind {
	NdAdd, // +
	NdSub, // -
	NdMul, // *
	NdDiv, // /
	NdNum, // 0123456879
	NdEql, // ==
	NdNeq, // !=
	NdGrt, // >
	NdGeq, // >=
	NdLst, // <
	NdLeq, // <=
	NdBnk, 
}

#[derive(Clone, Debug)]
pub struct Node{
	pub kind: NodeKind,
	pub lhs: Option<Box<Node>>,
	pub rhs: Option<Box<Node>>,
	pub val: Option<usize>,
}


impl Node{
	pub fn new(kind: NodeKind, lhs: Node, rhs: Node)->Self{
		Self { kind: kind, lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), val: None }
	}

	pub fn new_num(val: &usize)->Self{
		Self { kind: NodeKind::NdNum, lhs:None, rhs:None, val: Some(*val) }
	}

	pub fn expr(&mut self, tok: &mut Peekable<TokenList>)->Self{
		self.equality(tok)
	}

	pub fn equality(&mut self, tok: &mut Peekable<TokenList>)->Self{
		let mut node = self.relational(tok);
		loop{
			if let Some(c_tok) = tok.peek() {
				if c_tok.consume(&"==") {
					tok.next();
					node = Self::new(NodeKind::NdEql, node, self.relational(tok));
				}else if c_tok.consume(&"!="){
					tok.next();
					node = Self::new(NodeKind::NdNeq, node, self.relational(tok));
				}else{
					return node;
				}
			}else{
				return node;
			}
		}
	}

	pub fn relational(&mut self, tok: &mut Peekable<TokenList>)->Self{
		let mut node = self.add(tok);
		loop{
			if let Some(c_tok) = tok.peek() {
				if c_tok.consume(&'<') {
					tok.next();
					node = Self::new(NodeKind::NdLst, node, self.add(tok));
				}else if c_tok.consume(&"<="){
					tok.next();
					node = Self::new(NodeKind::NdLeq, node, self.add(tok));
				}else if c_tok.consume(&'>'){
					tok.next();
					node = Self::new(NodeKind::NdGrt, node, self.add(tok));
				}else if c_tok.consume(&">="){
					tok.next();
					node = Self::new(NodeKind::NdGeq, node, self.add(tok));
				}else{
					return node;
				}
			}else{
				return node;
			}
		}
	}

	pub fn add(&mut self, tok: &mut Peekable<TokenList>)->Self{
		let mut node = self.mul(tok);
		loop{
			if let Some(c_tok) = tok.peek() {
				if c_tok.consume(&'+') {
					tok.next();
					node = Self::new(NodeKind::NdAdd, node, self.mul(tok));
				}else if c_tok.consume(&'-'){
					tok.next();
					node = Self::new(NodeKind::NdSub, node, self.mul(tok));
				}else{
					return node;
				}
			}else{
				return node;
			}
		}
	}

	pub fn mul(&mut self, tok: &mut Peekable<TokenList>)->Self{
		let mut node = self.unary(tok);
		loop {
			if let Some(c_tok) = tok.peek(){
				if c_tok.consume(&'*') {
					tok.next();
					node = Self::new(NodeKind::NdMul, node, self.unary(tok));
				}else if c_tok.consume(&'/'){
					tok.next();
					node = Self::new(NodeKind::NdDiv, node, self.unary(tok));
				}else{
					return node;
				}
			}else{
				return node;
			}
		}

	}

	pub fn unary(&mut self, tok: &mut Peekable<TokenList>)->Self{
		if tok.peek().unwrap().consume(&'+'){
			tok.next();
			return self.primary(tok);
		}else if tok.peek().unwrap().consume(&'-'){
			tok.next();
			return Self::new(NodeKind::NdSub, Self::new_num(&0), self.primary(tok));
		}
		return self.primary(tok);
	}

	pub fn primary(&mut self, tok: &mut Peekable<TokenList>)->Self{
		if tok.peek().unwrap().consume(&'('){
			tok.next();
			let node = self.expr(tok);
			tok.next().unwrap().expect(&')');
			node
		}else{
			if self.kind == NodeKind::NdBnk {
				self.kind = NodeKind::NdNum;
				self.val = Some(tok.next().unwrap().expect_number());
				self.clone()
			}else{
				Self::new_num(&tok.next().unwrap().expect_number())
			}
		}
	}

	pub fn swap(&self)->Self{
		Self { kind: self.kind, lhs: self.rhs.clone(), rhs: self.lhs.clone(), val: self.val }
	}

	pub fn gen(&self){
		if self.kind == NodeKind::NdNum {
			println!("  push {}", self.val.unwrap());
			return;
		}

		self.lhs.as_ref().unwrap().gen();
		self.rhs.as_ref().unwrap().gen();

		println!("  pop rdi");
		println!("  pop rax");
		
		match self.kind {
			NodeKind::NdAdd => println!("  add rax, rdi"),
			NodeKind::NdSub => println!("  sub rax, rdi"),
			NodeKind::NdMul => println!("  mul rax, rdi"),
			NodeKind::NdDiv => {
				println!("  cqo");
				println!("  idiv rdi");
			},
			NodeKind::NdEql |
			NodeKind::NdNeq |
			NodeKind::NdGrt |
			NodeKind::NdGeq |
			NodeKind::NdLst |
			NodeKind::NdLeq =>{
				println!("  cmp rdi rax");
				match self.kind {
					NodeKind::NdEql => println!("  sete al"),
					NodeKind::NdNeq => println!("  setne al"),
					NodeKind::NdGrt => println!("  setg al"),
					NodeKind::NdGeq => println!("  setge al"),
					NodeKind::NdLst => println!("  setl al"),
					NodeKind::NdLeq => println!("  setle al"),
					_ => ()
				}
				println!("  movzb rax, al");
			}
			_ => ()
		};

		println!("  push rax");
		
	}
}