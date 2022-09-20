enum TreeNode { 
    Join { freq: u32, left: Box<TreeNode>, right: Box<TreeNode> },
    Value { freq: u32, value: u8 }
}

fn increase_tabs(s: String) -> String {
    let mut res: String = String::new();
    for line in s.lines() {
        res = res + "\t" + line + "\n";
    }
    //println!("Incr {}", res);
    return res;
}

impl TreeNode {
    fn to_string(&self) -> String {
        return match self {
            TreeNode::Join {left, right} => increase_tabs(left.to_string() + "\n" + &right.to_string() + "\n"),
            TreeNode::Value {value} => value.to_string()  
        };
    }
}

fn main() {
    let tree = TreeNode::Join {
        left: Box::new(TreeNode::Value {value: 69}),
        right: Box::new(TreeNode::Join {
            left: Box::new(TreeNode::Value {value: 34}),
            right: Box::new(TreeNode::Value {value: 35})
        })
    };
    println!("{}", tree.to_string());
}
