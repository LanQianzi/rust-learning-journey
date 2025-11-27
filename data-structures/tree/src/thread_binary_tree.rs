use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

type TreeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;
type ThreadLink<T> = Option<Weak<RefCell<TreeNode<T>>>>;

struct TreeNode<T> {
    val: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
    predecessor: ThreadLink<T>,
    successor: ThreadLink<T>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
            predecessor: None,
            successor: None,
        }
    }
}

pub struct ThreadBinaryTree<T> {
    root: TreeLink<T>,
}

impl<T: Ord + Clone> ThreadBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) {
        self.root = Self::insert_recursive(self.root.take(), val);
        self.thread();
    }

    fn insert_recursive(node: TreeLink<T>, val: T) -> TreeLink<T> {
        match node {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node_rc) => {
                let current_value = node_rc.borrow().val.clone();
                match (current_value).cmp(&val) {
                    Ordering::Less => {
                        let current_right = node_rc.borrow_mut().right.take();
                        node_rc.borrow_mut().right = Self::insert_recursive(current_right, val);
                        Some(node_rc)
                    }
                    Ordering::Greater => {
                        let current_left = node_rc.borrow_mut().left.take();
                        node_rc.borrow_mut().left = Self::insert_recursive(current_left, val);
                        Some(node_rc)
                    }
                    Ordering::Equal => Some(node_rc),
                }
            }
        }
    }

    fn inorder_collect(&self, node: &TreeLink<T>, result: &mut Vec<Rc<RefCell<TreeNode<T>>>>) {
        if let Some(n) = node {
            self.inorder_collect(&n.borrow().left, result);
            result.push(n.clone());
            self.inorder_collect(&n.borrow().right, result);
        }
    }

    pub fn thread(&mut self) {
        let mut nodes = Vec::new();
        self.inorder_collect(&self.root, &mut nodes);
        for i in 0..nodes.len() {
            let node = nodes[i].clone();
            let mut nmb = node.borrow_mut();

            if i > 0 {
                nmb.predecessor = Some(Rc::downgrade(&nodes[i - 1]))
            } else {
                nmb.predecessor = None
            }

            if i < nodes.len() - 1 {
                nmb.successor = Some(Rc::downgrade(&nodes[i + 1]))
            } else {
                nmb.successor = None
            }
        }
    }

    pub fn lever_order<'a>(&'a self) -> Vec<T> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        queue.push_back(self.root.as_ref().unwrap().clone());
        while !queue.is_empty() {
            let tmp = queue.pop_front().unwrap();
            result.push(tmp.borrow().val.clone());
            if let Some(l) = tmp.borrow().left.clone() {
                queue.push_back(l);
            }
            if let Some(r) = tmp.borrow().right.clone() {
                queue.push_back(r);
            }
        }
        result
    }

    pub fn thread_inorder(&self) -> Vec<T>
    where
        T: Clone,
    {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut left_node = self.root.clone().unwrap();
        while left_node.borrow().left.is_some() {
            let next_node = {
                // 在这个作用域内获取借用，结束后自动释放
                let borrowed = left_node.borrow();
                borrowed.left.as_ref().unwrap().clone()
            };
            left_node = next_node;
        }
        let mut current = Some(left_node);
        let mut result = Vec::new();
        while let Some(node) = current {
            result.push(node.borrow().val.clone());
            // 获取后继节点
            let successor = if let Some(succ) = node.borrow().successor.as_ref() {
                succ.upgrade()
            } else {
                None
            };

            current = successor;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut bst = ThreadBinaryTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        assert_eq!(bst.lever_order(), vec![5, 3, 7, 2, 4, 6, 8]);
    }

    #[test]
    fn case2() {
        let mut bst = ThreadBinaryTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        // 测试遍历功能
        assert_eq!(bst.thread_inorder(), vec![2, 3, 4, 5, 6, 7, 8]);
    }
}
