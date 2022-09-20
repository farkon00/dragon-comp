enum TreeNode { 
    Join { freq: u32, left: Box<TreeNode>, right: Box<TreeNode> },
    Value { freq: u32, value: u8 }
}

fn increase_tabs(s: String) -> String {
    let mut res: String = String::new();
    for line in s.lines() {
        res = res + "    " + line + "\n";
    }
    return res;
}

impl TreeNode {
    fn to_string(&self) -> String {
        return match self {
            TreeNode::Join {freq, left, right} => freq.to_string() + ":\n" + &increase_tabs(left.to_string() + "\n" + &right.to_string() + "\n"),
            TreeNode::Value {freq, value} => freq.to_string() + ":" + &value.to_string()
        };
    }
}

fn main() {
    let tree = TreeNode::Join {
        freq: 123,
        left: Box::new(TreeNode::Value {freq:42, value: 69}),
        right: Box::new(TreeNode::Join {
            freq: 96,
            left: Box::new(TreeNode::Value {freq: 35, value: 34}),
            right: Box::new(TreeNode::Value {freq: 34, value: 35})
        })
    };
    println!("{}", tree.to_string());
}
