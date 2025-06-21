use std::{collections::HashMap };
pub fn ef1() {
    // f1_3();
    // f1_4();
}

#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

fn f1_3() {
    let mut json = Json::Object(
        HashMap::from([
            ("name".to_string(), Json::String("Alice".to_string())),
            ("age".to_string(), Json::Number(20.0)),
            ("active".to_string(), Json::Bool(true)),
            ("address".to_string(), Json::Null),
        ]),
    );
    println!("{:?}", json);
}

#[derive(Debug)]
enum BinaryTree<T> { 
    Empty,
    NotEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn f1_4() {
    use BinaryTree::*;
    let jup_tree = NotEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));
    let mer = NotEmpty(Box::new(TreeNode {
        element: "mercury",
        left: Empty,
        right: Empty,
    }));
    let mars = NotEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jup_tree,
        right: mer,
    }));
    println!("{mars:?}");
}


