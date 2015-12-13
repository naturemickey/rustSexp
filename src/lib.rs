
pub enum SNode {
	Leaf(i8, String),
	Node(Box<Vec<SNode>>)
}

pub fn parse(exp:&str) -> SNode {
	let vc = exp.chars().collect();
	parse_chars(vc, 0, 0)
}

fn parse_chars(vc:Vec<char>, idx:usize, mut deep:usize) -> SNode {
	let c = vc[idx];
	if c == '(' {
		deep += 1;
	}
	SNode::Leaf(0, "".to_string())
}
