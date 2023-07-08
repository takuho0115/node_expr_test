#[path="./token.rs"]
mod token;
use token::*;
#[derive(PartialEq,Clone, Copy)]
pub enum NodeKind {
	ND_ADD,
	ND_SUB,
	ND_MUL,
	ND_DIV,
	ND_NUM,
}

#[derive(Clone)]
pub struct Node{
	kind: NodeKind,
	lhs: Option<Box<Node>>,
	rhs: Option<Box<Node>>,
	val: Option<usize>,
}

impl Node{
	pub fn new(kind: NodeKind, lhs: Node, rhs: Node)->Self{
		Self { kind: kind, lhs: Some(Box::new(lhs)), rhs: Some(Box::new(rhs)), val: None }
	}

	pub fn new_num(val: &usize)->Self{
		Self { kind: NodeKind::ND_NUM, lhs:None, rhs:None, val: Some(*val) }
	}

	pub fn expr(&self, tok: &Token)->Self{
		let mut node = self.primary(tok);
		loop {
			if tok.consume('+') {
				node = Self::new(NodeKind::ND_ADD, node, self.primary(tok));
			}else if tok.consume('-'){
				node = Self::new(NodeKind::ND_SUB, node, self.primary(tok));
			}else{
				return node;
			}
		}
	}

	pub fn mul(&self, tok: &Token)->Self{
		let mut node = self.primary(tok);
		loop {
			if tok.consume('*') {
				node = Self::new(NodeKind::ND_MUL, node, self.primary(tok));
			}else if tok.consume('/'){
				node = Self::new(NodeKind::ND_DIV, node, self.primary(tok));
			}else{
				return node;
			}
		}

	}

	pub fn primary(&self, tok: &Token)->Self{
		if tok.consume('('){
			let node = self.expr(tok);
			tok.expect(')');
			node
		}else{
			Self::new_num(&tok.expect_number())
		}
	}

	pub fn gen(&self){
		if self.kind == NodeKind::ND_NUM {
			println!("  push {}", self.val.unwrap());
			return;
		}

		self.lhs.as_ref().unwrap().gen();
		self.rhs.as_ref().unwrap().gen();

		println!("  pop rdi");
		println!("  pop rax");
		
		match self.kind {
			NodeKind::ND_ADD => println!("  add rax, rdi"),
			NodeKind::ND_SUB => println!("  sub rax, rdi"),
			NodeKind::ND_MUL => println!("  mul rax, rdi"),
			NodeKind::ND_DIV => {
				println!("  cqo");
				println!("  idiv rdi")
			},
			_ => {}
		};

		println!("  push rax");
		
	}
}