use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

type TreeLink<T> = Option<Box<TreeNode<T>>>;

struct TreeNode<T> {
    val: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct GBinaryTree<T> {
    root: TreeLink<T>,
}

impl<T: Ord + fmt::Display> GBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn un_gyb(gyb: &str) -> Self
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        Self {
            root: Self::deserialization(gyb),
        }
    }

    fn deserialization(gyb: &str) -> TreeLink<T>
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        if gyb.is_empty() || gyb == "()" {
            return None;
        }
        let paren_opt = gyb.find('(');
        if paren_opt.is_none() {
            return Some(Box::new(TreeNode::new(gyb.parse::<T>().unwrap())));
        }

        let root_str = &gyb[0..paren_opt.unwrap()];
        let root_val: T = root_str
            .parse()
            .expect(&format!("Failed to parse root node value '{}'", root_str));
        let mut root = Box::new(TreeNode::new(root_val));

        let mut stack = 1;
        let mut comma_opt = None;
        let son_str = &gyb[paren_opt.unwrap() + 1..gyb.len() - 1];
        for (i, c) in son_str.chars().enumerate() {
            if c == '(' {
                stack += 1;
            } else if c == ')' {
                stack -= 1;
            } else if c == ',' && stack == 1 {
                comma_opt = Some(i);
                break;
            }
        }

        if let Some(comma_idx) = comma_opt {
            let left_str = &son_str[..comma_idx];
            let right_str = &son_str[comma_idx + 1..];
            root.left = Self::deserialization(left_str);
            root.right = Self::deserialization(right_str);
        } else {
            // 只有左子树
            let left_str = son_str;
            root.left = Self::deserialization(left_str)
        }

        Some(root)
    }

    pub fn insert(&mut self, val: T) {
        self.root.tree_insert(val);
    }

    pub fn inorder(&self) -> Vec<&T> {
        let mut res = Vec::new();
        self.root.inorder(&mut res);
        res
    }

    pub fn gyb(&self) -> String {
        self.root.gyb()
    }
}

trait Insert<T: Ord> {
    fn tree_insert(&mut self, val: T);
}

impl<T: Ord> Insert<T> for TreeLink<T> {
    fn tree_insert(&mut self, val: T) {
        match self {
            None => *self = Some(Box::new(TreeNode::new(val))),
            Some(node_rc) => match node_rc.val.cmp(&val) {
                Ordering::Less => node_rc.right.tree_insert(val),
                Ordering::Greater => node_rc.left.tree_insert(val),
                Ordering::Equal => {}
            },
        }
    }
}

trait Traverse<T> {
    fn inorder<'a>(&'a self, res: &mut Vec<&'a T>);
}

impl<T> Traverse<T> for TreeLink<T> {
    fn inorder<'a>(&'a self, res: &mut Vec<&'a T>) {
        if let Some(node) = self {
            node.left.inorder(res);
            res.push(&node.val);
            node.right.inorder(res);
        }
    }
}

trait Serialize<T: fmt::Display> {
    fn gyb(&self) -> String;
}

impl<T: fmt::Display> Serialize<T> for TreeLink<T> {
    fn gyb(&self) -> String {
        if self.is_none() {
            return String::new();
        }
        let node = self.as_ref().unwrap();
        let left_str = node.left.gyb();
        let right_str = node.right.gyb();

        if left_str.is_empty() && right_str.is_empty() {
            return node.val.to_string();
        }
        if left_str.is_empty() && !right_str.is_empty() {
            return format!("{}(,{})", node.val, right_str);
        }

        format!("{}({},{})", node.val, left_str, right_str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    ///     5
    ///    / \
    ///   3   7
    ///  / \ / \
    /// 2  4 6  8

    #[test]
    fn case1() {
        let mut bst = GBinaryTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        // 测试遍历功能
        assert_eq!(bst.inorder(), vec![&2, &3, &4, &5, &6, &7, &8]);
        assert_eq!(bst.gyb(), "5(3(2,4),7(6,8))".to_string());
    }
    #[test]
    fn case2() {
        let mut bst = GBinaryTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        assert_eq!(bst.inorder(), vec![&2, &3, &4, &5, &6, &7, &8]);
        let gyb = bst.gyb();
        let bst2 = GBinaryTree::<i32>::un_gyb(&gyb);
        assert_eq!(bst2.inorder(), vec![&2, &3, &4, &5, &6, &7, &8]);
    }

    #[test]
    fn case3() {
        let mut bst = GBinaryTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        bst.insert(81);
        bst.insert(22);
        bst.insert(-22);
        bst.insert(-2);
        bst.insert(-3);
        bst.insert(-9);

        let gyb = bst.gyb();
        let bst2 = GBinaryTree::<i32>::un_gyb(&gyb);
        assert_eq!(bst.inorder(), bst2.inorder());
    }
}
