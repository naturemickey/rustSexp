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
	let s = "(\n\r\t\"\n\r\t\\t\\r\\n\"\t)";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![SNode::new_leaf(1, "\n\r\t\t\r\n".to_string())]);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works7() {
	// (@... "/*" (@* (@! "*/")) "*/")
	let s = "(@... \"/*\" (@* (@! \"*/\")) \"*/\")";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![
		SNode::new_leaf(0, "@...".to_string()),
		SNode::new_leaf(1, "/*".to_string()),
		SNode::new_node(vec![
			SNode::new_leaf(0, "@*".to_string()),
			SNode::new_node(vec![
				SNode::new_leaf(0, "@!".to_string()),
				SNode::new_leaf(1, "*/".to_string()),
			]),
		]),
		SNode::new_leaf(1, "*/".to_string()),
	]);
	println!("");
	println!("{}", n1);
	println!("{}", n2);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works8() {
	let s = "
		(define (cons x y)
			(lambda (m) (m x y)))
		(define (car z)
			(z (lambda (p q) p)))
		(define (cdr z)
			(z (lambda (p q) q)))";
	let vn1 = parse(s);
	let vn2 = vec![
		SNode::new_node(vec![
			SNode::new_leaf(0, "define".to_string()),
			SNode::new_node(vec![
				SNode::new_leaf(0, "cons".to_string()),
				SNode::new_leaf(0, "x".to_string()),
				SNode::new_leaf(0, "y".to_string()),
			]),
			SNode::new_node(vec![
				SNode::new_leaf(0, "lambda".to_string()),
				SNode::new_node(vec![
					SNode::new_leaf(0, "m".to_string()),
				]),
				SNode::new_node(vec![
					SNode::new_leaf(0, "m".to_string()),
					SNode::new_leaf(0, "x".to_string()),
					SNode::new_leaf(0, "y".to_string()),
				]),
			]),
		]),
		SNode::new_node(vec![
			SNode::new_leaf(0, "define".to_string()),
			SNode::new_node(vec![
				SNode::new_leaf(0, "car".to_string()),
				SNode::new_leaf(0, "z".to_string()),
			]),
			SNode::new_node(vec![
				SNode::new_leaf(0, "z".to_string()),
				SNode::new_node(vec![
					SNode::new_leaf(0, "lambda".to_string()),
					SNode::new_node(vec![
						SNode::new_leaf(0, "p".to_string()),
						SNode::new_leaf(0, "q".to_string()),
					]),
					SNode::new_leaf(0, "p".to_string()),
				]),
			]),
		]),
		SNode::new_node(vec![
			SNode::new_leaf(0, "define".to_string()),
			SNode::new_node(vec![
				SNode::new_leaf(0, "cdr".to_string()),
				SNode::new_leaf(0, "z".to_string()),
			]),
			SNode::new_node(vec![
				SNode::new_leaf(0, "z".to_string()),
				SNode::new_node(vec![
					SNode::new_leaf(0, "lambda".to_string()),
					SNode::new_node(vec![
						SNode::new_leaf(0, "p".to_string()),
						SNode::new_leaf(0, "q".to_string()),
					]),
					SNode::new_leaf(0, "q".to_string()),
				]),
			]),
		]),
	];
	assert_eq!(vn1.len(), vn2.len());
	println!("");
	for i in 0 .. vn1.len() {
		println!("1: {}", vn1[i]);
		println!("2: {}", vn2[i]);
		assert!(vn1[i].equals(&vn2[i]));
	}
}
#[test]
fn it_works9() {
	let s = "(我是 猪头 三)";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![
		SNode::new_leaf(0, "我是".to_string()),
		SNode::new_leaf(0, "猪头".to_string()),
		SNode::new_leaf(0, "三".to_string()),
	]);
	assert!(n1.equals(&n2));
}
#[test]
fn it_works10() {
	let s = "(+ 1 (* 2 3))";
	let vn:Vec<SNode> = parse(s);
	let n1:&SNode = &vn[0];
	let n2:SNode = SNode::new_node(vec![
		SNode::new_leaf(0, "+".to_string()),
		SNode::new_leaf(0, "1".to_string()),
		SNode::new_node(vec![
			SNode::new_leaf(0, "*".to_string()),
			SNode::new_leaf(0, "2".to_string()),
			SNode::new_leaf(0, "3".to_string()),
		]),
	]);
	assert!(n1.equals(&n2));
}