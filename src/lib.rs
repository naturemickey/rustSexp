
pub enum SNode {
	// 0.标识， 1.字符串（不含双引号），后续有需要再加字段叶子类型
	Leaf(i8, String),
	Node(Box<Vec<SNode>>)
}

pub fn parse(exp:&str) -> SNode {
	let vc = exp.chars().collect();
	parse_chars(vc, 0, 0)
}

fn read_snodes(vc:Vec<char>, mut idx:usize, mut deep:usize) -> Vec<SNode> {
	idx = pass_blank(&vc, idx);
	if idx == vc.len() {
		Vec::new()
	} else {
		let snodes = Vec::new();
		let c = vc[idx];
		if c != '(' {
			panic!("S expression must be start with '('");
		}
		snodes
	}
}

fn read_snode(Vec:Vec<char>, mut idx:usize) -> SNode {
	SNode::Leaf(0, "".to_string())
}

fn parse_chars(vc:Vec<char>, mut idx:usize, mut deep:usize) -> SNode {
	idx = pass_blank(&vc, idx);
	let c = vc[idx];
	if c != '(' {
		panic!("S expression must be start with '('");
	}
	deep += 1;
	SNode::Leaf(0, "".to_string())
}

fn pass_blank(vc:&Vec<char>, idx:usize) -> usize {
	for i in idx .. vc.len() {
		let c = vc[i];
		if !(c == ' ' || c == '\t' || c == '\n' || c == '\r') {
			return i;
		}
	}
	vc.len()
}
