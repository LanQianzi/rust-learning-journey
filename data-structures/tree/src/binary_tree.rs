use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;

type TreeLink<T> = Option<Box<TreeNode<T>>>;

#[derive(Debug)]
struct TreeNode<T: Debug> {
    val: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
}

impl<T: Debug> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinaryTree<T: Debug> {
    root: TreeLink<T>,
}

impl<T: Debug + Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) {
        self.root.tree_insert(val);
    }

    pub fn preorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.root.preorder(&mut result);
        result
    }

    pub fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.root.inorder(&mut result);
        result
    }

    pub fn postorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.root.postorder(&mut result);
        result
    }

    pub fn lever_order<'a>(&'a self) -> Vec<&'a T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut queue = VecDeque::new();
        queue.push_back(self.root.as_ref().unwrap());

        let mut result = Vec::new();

        while !queue.is_empty() {
            let tmp = queue.pop_front().unwrap();
            result.push(&tmp.val);

            if let Some(ln) = tmp.left.as_ref() {
                queue.push_back(ln);
            }
            if let Some(rn) = tmp.right.as_ref() {
                queue.push_back(rn);
            }
        }
        result
    }
}

trait Insert<T: Ord> {
    fn tree_insert(&mut self, val: T);
}

impl<T: Debug + Ord> Insert<T> for TreeLink<T> {
    fn tree_insert(&mut self, val: T) {
        match self {
            None => *self = Some(Box::new(TreeNode::new(val))),
            Some(node) => match val.cmp(&node.val) {
                Ordering::Less => node.left.tree_insert(val),
                Ordering::Greater => node.right.tree_insert(val),
                Ordering::Equal => (),
            },
        }
    }
}

trait Traverse<T> {
    fn preorder<'a>(&'a self, result: &mut Vec<&'a T>); // 前序遍历
    fn inorder<'a>(&'a self, result: &mut Vec<&'a T>); // 中序遍历
    fn postorder<'a>(&'a self, result: &mut Vec<&'a T>); // 后序遍历
}

impl<T: Debug> Traverse<T> for TreeLink<T> {
    fn preorder<'a>(&'a self, result: &mut Vec<&'a T>) {
        if let Some(node) = self {
            result.push(&node.val);
            node.left.preorder(result);
            node.right.preorder(result);
        }
    }

    fn inorder<'a>(&'a self, result: &mut Vec<&'a T>) {
        if let Some(node) = self {
            node.left.inorder(result);
            result.push(&node.val);
            node.right.inorder(result);
        }
    }

    fn postorder<'a>(&'a self, result: &mut Vec<&'a T>) {
        if let Some(node) = self {
            node.left.postorder(result);
            node.right.postorder(result);
            result.push(&node.val);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut bst = BinaryTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        // 测试遍历功能
        assert_eq!(bst.preorder(), vec![&5, &3, &2, &4, &7, &6, &8]);
        assert_eq!(bst.inorder(), vec![&2, &3, &4, &5, &6, &7, &8]);
        assert_eq!(bst.postorder(), vec![&2, &4, &3, &6, &8, &7, &5]);
        assert_eq!(bst.lever_order(), vec![&5, &3, &7, &2, &4, &6, &8]);
    }
}
