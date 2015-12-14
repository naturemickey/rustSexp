extern crate rust_s_exp;

use rust_s_exp::SNode::{Node, Leaf};
use rust_s_exp::parse;

#[test]
fn it_works() {
	let s = "(abc)";
	assert!(parse(s) == Node(vec![Leaf(0, "abc".to_string())]));
}
