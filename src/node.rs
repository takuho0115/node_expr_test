use std::iter::Peekable;
#[path="./tokenlist.rs"]
pub mod tokenlist;
use tokenlist::*;
#[derive(PartialEq,Clone, Copy)]
pub enum NodeKind {
	NdAdd,
	NdSub,
	NdMul,
	NdDiv,
	NdNum,
	NdFirst,
}

#[derive(Clone)]
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
		let mut node = self.mul(tok);
		loop {
			if let Some(c_tok) = tok.peek() {
				if c_tok.consume('+') {
					tok.next();
					node = Self::new(NodeKind::NdAdd, node, self.mul(tok));
				}else if c_tok.consume('-'){
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
				if c_tok.consume('*') {
					tok.next();
					node = Self::new(NodeKind::NdMul, node, self.unary(tok));
				}else if c_tok.consume('/'){
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
		if tok.peek().unwrap().consume('+'){
			tok.next();
			return self.primary(tok);
		}else if tok.peek().unwrap().consume('-'){
			tok.next();
			return Self::new(NodeKind::NdSub, Self::new_num(&0), self.primary(tok));
		}
		return self.primary(tok);
	}

	pub fn primary(&mut self, tok: &mut Peekable<TokenList>)->Self{
		if tok.peek().unwrap().consume('('){
			tok.next();
			let node = self.expr(tok);
			tok.next().unwrap().expect(')');
			node
		}else{
			if self.kind == NodeKind::NdFirst {
				self.kind = NodeKind::NdNum;
				self.val = Some(tok.next().unwrap().expect_number());
				self.clone()
			}else{
				Self::new_num(&tok.next().unwrap().expect_number())
			}
		}
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
				println!("  idiv rdi")
			},
			_ => {}
		};

		println!("  push rax");
		
	}
}