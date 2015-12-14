
use std::option::Option::{None, Some};

pub enum SNode {
	// 0.标识， 1.字符串（不含双引号），后续有需要再加字段叶子类型
	Leaf(i8, String),
	Node(Vec<SNode>)
}

use SNode::{Leaf, Node};
use std::cmp::Eq;
use std::cmp::PartialEq;

impl PartialEq for SNode {
	fn eq(&self, other:&SNode) -> bool {
		true
	}
	fn ne(&self, other:&SNode) -> bool {
		true
	}
}
impl Eq for SNode {
	
}

pub fn parse(exp:&str) -> Vec<SNode> {
	let vc = exp.chars().collect();
	let (_, _, res) = read_snodes(&vc, 0, 0);
	res
}

fn read_snodes(vc:&Vec<char>, idx:usize, deep:usize) -> (usize, usize, Vec<SNode>) {
	if idx == vc.len() {
		(idx, deep, Vec::new())
	} else {
		let mut snodes = Vec::new();
		let mut i = idx;
		let mut d = deep;
		while let Some((new_idx, new_deep, snode)) = read_snode(vc, i, d) {
			i = new_idx;
			d = new_deep;
			snodes.push(snode);
		}
		(i, d, snodes)
	}
}

///
/// return (new_idx, new_deep, SNode)
///
fn read_snode(vc:&Vec<char>, idx:usize, deep:usize) -> Option<(usize, usize, SNode)> {
	let idx = pass_blank(vc, idx);
	if idx == vc.len() || vc[idx] == ')' {
		None
	} else {
		let c = vc[idx];
		if c == '(' {
			let (idx, _, vn) = read_snodes(vc, idx + 1, deep + 1);
			if vc[idx] != ')' {
				panic!("expect ')' but find {}", vc[idx]);
			}
			Some((idx + 1, deep, SNode::Node(vn)))
		} else {
			let (i, n) = read_leaf(vc, idx);
			Some((i, deep, n))
		}
	}
}

fn read_leaf(vc:&Vec<char>, idx:usize) -> (usize, SNode) {
	let c = vc[idx];

	if c == '"' {
		let mut s:String = String::new();
		let mut idx_to:usize = idx + 1;
		while idx_to < vc.len() {
			if vc[idx_to] == '\\' {
				if idx_to + 1 < vc.len() {
					idx_to += 1;
					s.push(vc[idx_to]);
				} else {
					panic!("unexpect EOF");
				}
			} else if vc[idx_to] != '"' {
				s.push(vc[idx_to]);
			} else {
				break;
			}
			idx_to += 1;
		}
		if idx_to == vc.len() {
			panic!("expect '\"' but find EOF");
		}
		(idx_to + 1, SNode::Leaf(1, s))
	} else {
		let mut s:String = String::new();
		let mut idx_to:usize = idx + 1;
		while idx_to < vc.len() {
			if vc[idx_to] == '\\' {
				if idx_to + 1 < vc.len() {
					idx_to += 1;
					s.push(vc[idx_to]);
				} else {
					panic!("unexpect EOF");
				}
			} else if is_not_blank(vc[idx_to]) && vc[idx_to] != ')' {
				s.push(vc[idx_to]);
			} else {
				break;
			}
			idx_to += 1
		}
		(idx_to, SNode::Leaf(1, s))
	}
}

fn pass_blank(vc:&Vec<char>, idx:usize) -> usize {
	for i in idx .. vc.len() {
		let c = vc[i];
		if is_not_blank(c) {
			return i;
		}
	}
	vc.len()
}

fn is_blank(c:char) -> bool {
	(c == ' ' || c == '\t' || c == '\n' || c == '\r')
}

fn is_not_blank(c:char) -> bool {
	!is_blank(c)
}