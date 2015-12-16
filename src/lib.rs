
use std::option::Option::{None, Some};

pub struct SNode {
	// 0.标识
	// 1.字符串（不含双引号)
	// 2.数字
	// 后续有需要再加字段叶子类型
	_type:i8,
	token:String,
	children:Vec<SNode>
}

impl SNode {
	pub fn new_leaf(_type:i8, token:String) -> SNode {
		SNode{_type:_type, token:token, children:Vec::new()}
	}
	pub fn new_node(children:Vec<SNode>) -> SNode {
		SNode{_type:-1, token:"".to_string(), children:children}
	}
	pub fn is_leaf(&self) -> bool {
		self._type != -1
	}
	pub fn equals(&self, other:&SNode) -> bool {
		if self._type != other._type {
			false
		} else if self.token != other.token {
			false
		} else if self.children.len() != other.children.len() {
			false
		} else {
			let len = self.children.len();
			for i in 0 .. len {
				if !self.children[i].equals(&other.children[i]) {
					return false;
				}
			}
			true
		}
	}
	pub fn to_string(&self) -> String {
		let mut res = String::new();
		match self._type {
			-1 => {
				res.push('(');
				if self.children.len() > 0 {
					for sn in &self.children {
						res.push_str(&sn.to_string());
						res.push(' ');
					}
					res.pop();
				}
				res.push(')');
			},
			1 => {
				res.push('"');
				res.push_str(&self.token);
				res.push('"');
			},
			_ => {
				res.push_str(&self.token);
			}
		}
		res
	}
}

impl std::fmt::Display for SNode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
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
		// 破绽在这里
		// None并没有办法返回idx和deep
		None
	} else {
		let c = vc[idx];
		if c == '(' {
			let (idx, _, vn) = read_snodes(vc, idx + 1, deep + 1);
			let idx = pass_blank(vc, idx); // 为了解决上面的破绽。
			if vc[idx] != ')' {
				panic!("expect ')' but find '{}'", vc[idx]);
			}
			Some((idx + 1, deep, SNode::new_node(vn)))
		} else {
			let (i, n) = read_leaf(vc, idx);
			Some((i, deep, n))
		}
	}
}

fn read_leaf(vc:&Vec<char>, idx:usize) -> (usize, SNode) {
	let c = vc[idx];

	let read_fn:fn(&Vec<char>, usize) -> (usize, SNode) = match c {
		'"' => read_an_string,
		_ => read_an_id,
	};
	read_fn(vc, idx)
}

fn read_an_id(vc:&Vec<char>, idx:usize) -> (usize, SNode) {
	let mut s:String = String::new();
	let mut idx_to:usize = idx;
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
	if is_number(&s) {
		(idx_to, SNode::new_leaf(2, s))
	} else {
		(idx_to, SNode::new_leaf(0, s))
	}
}

fn read_an_string(vc:&Vec<char>, idx:usize) -> (usize, SNode) {
	let mut s:String = String::new();
	let mut idx_to:usize = idx + 1;
	while idx_to < vc.len() {
		if vc[idx_to] == '\\' {
			if idx_to + 1 < vc.len() {
				idx_to += 1;
				s.push(match vc[idx_to] {
					'n' => '\n',
					'r' => '\r',
					't' => '\t',
					x => x,
				});
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
	(idx_to + 1, SNode::new_leaf(1, s))
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

pub fn is_number(s:&str) -> bool {
	// F : 2, 4, 5
	// 0 --[+-]--> 1
	// 0 --[0-9]-> 2
	// 0 --[.]---> 3
	// 1 --[0-9]-> 2
	// 1 --[.]---> 3
	// 2 --[.]---> 4
	// 2 --[0-9]-> 2
	// 3 --[0-9]-> 5
	// 4 --[0-9]-> 5
	// 5 --[0-9]-> 5
	let mut state = 0;
	for c in s.chars() {
		state = match state {
			0 => {
				if is_sign(c) {
					1
				} else if is_dox_num(c) {
					2
				} else if c == '.' {
					3
				} else {
					return false;
				}
			},
			1 => {
				if is_dox_num(c) {
					2
				} else if c == '.' {
					3
				} else {
					return false;
				}
			},
			2 => {
				if is_dox_num(c) {
					2
				} else if c == '.' {
					4
				} else {
					return false;
				}
			},
			3 => if is_dox_num(c) { 5 } else { return false; },
			4 => if is_dox_num(c) { 5 } else { return false; },
			5 => if is_dox_num(c) { 5 } else { return false; },
			_ => return false,
		}
	}
	(state == 2 || state == 4 || state == 5)
}

fn is_sign(c:char) -> bool {
	(c == '+' || c == '-')
}
fn is_dox_num(c:char) -> bool {
	('0' <= c && c <= '9')
}