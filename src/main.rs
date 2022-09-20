use std::collections::HashMap;

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
            TreeNode::Join {freq, left, right} => freq.to_string() + 
                ":\n" + &increase_tabs(left.to_string() + "\n" + &right.to_string() + "\n"),
            TreeNode::Value {freq, value} => freq.to_string() + ":" + &value.to_string()
        };
    }

    fn freq_from_bytes(bytes: Vec<u8>) -> HashMap<u8, u32> {
        let mut res = HashMap::new();

        for byte in bytes {
            match res.get(&byte) {
                None => res.insert(byte, 1),
                Some(curr) => res.insert(byte, curr+1)
            };
        }

        return res;
    }

    fn from_bytes(bytes: Vec<u8>) -> TreeNode {
        for (key, item) in TreeNode::freq_from_bytes(bytes).iter() {
            println!("{}", key.to_string() + " : " +  &item.to_string())
        }
        return TreeNode::Value { freq:0, value:0 };
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
    TreeNode::from_bytes(vec![1, 2, 3, 56, 2, 2, 3]);
}
