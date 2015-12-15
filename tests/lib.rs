extern crate rust_s_exp;

use rust_s_exp::SNode;
use rust_s_exp::parse;

#[test]
fn it_works1() {
	let s = "(abc )";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![SNode::new_leaf(0, "abc".to_string())]);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works2() {
	let s = "(\"abc\")";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![SNode::new_leaf(1, "abc".to_string())]);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works3() {
	let s = "(abc \"def\")";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode =
		SNode::new_node(vec![
			SNode::new_leaf(0, "abc".to_string()),
			SNode::new_leaf(1, "def".to_string())
		]);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works4() {
	let s = "(abc (def))";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode =
		SNode::new_node(vec![
			SNode::new_leaf(0, "abc".to_string()),
			SNode::new_node(vec![
				SNode::new_leaf(0, "def".to_string())
			])
		]);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works5() {
	let s = " ( abc ) ( +-*/ ) ";
	let vn:Vec<SNode> = parse(s);
	let vn2:Vec<SNode> = vec![
		SNode::new_node(vec![
			SNode::new_leaf(0, "abc".to_string())
		]),
		SNode::new_node(vec![
			SNode::new_leaf(0, "+-*/".to_string())
		]),
	];
	assert_eq!(vn.len(), vn2.len());
	for i in 0 .. vn.len() {
		assert!(vn[i].equals(&vn2[i]));
	}
}
#[test]
fn it_works6() {
	let s = "(\n\r\t\"\n\r\t\\t\\r\\n\")";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![SNode::new_leaf(1, "\n\r\t\t\r\n".to_string())]);
	assert!(n1.equals(&n2));
}