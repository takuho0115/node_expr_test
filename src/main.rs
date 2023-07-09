use std::env;

mod node;
use node::{*, tokenlist::*};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2{
		panic!("引数の個数が正しくありません");
	}
	let mut token = TokenList::new(args.get(1).unwrap().as_str()).peekable();
	let mut node:Node = Node { kind: NodeKind::NdFirst, lhs: None, rhs: None, val: None };
  let node = node.expr(&mut token);

  // アセンブリの前半部分を出力
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");

	node.gen();

	println!("  ret");
}
