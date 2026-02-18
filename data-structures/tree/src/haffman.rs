use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};
use std::path::Path;
use std::{cmp, collections::HashMap, fs::File};

type Link = Option<Box<Node>>;
#[derive(Clone)]
struct Node {
    freq: usize,
    ch: Option<char>,
    left: Link,
    right: Link,
}

pub struct Haffman {
    root: Link,
    codes: HashMap<char, String>,
}

impl Node {
    fn new(ch: Option<char>, freq: usize) -> Self {
        Self {
            freq,
            ch,
            left: None,
            right: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq.eq(&other.freq)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl Haffman {
    pub fn new(num_map: HashMap<char, usize>) -> Self {
        let mut min_heap = BinaryHeap::new();
        for (k, v) in num_map {
            min_heap.push(Reverse(Node::new(Some(k), v)));
        }

        while min_heap.len() > 1 {
            let Reverse(ln) = min_heap.pop().unwrap();
            let Reverse(rn) = min_heap.pop().unwrap();
            let mut new_node = Node::new(None, ln.freq + rn.freq);
            new_node.left = Some(Box::new(ln));
            new_node.right = Some(Box::new(rn));
            min_heap.push(Reverse(new_node));
        }

        if min_heap.is_empty() {
            Self {
                root: None,
                codes: HashMap::new(),
            }
        } else {
            let Reverse(node) = min_heap.pop().unwrap();
            Self {
                root: Some(Box::new(node)),
                codes: HashMap::new(),
            }
        }
    }

    pub fn generate_huffman_codes(&mut self) -> &HashMap<char, String> {
        let op_node = self.root.clone();
        self.dfs(&op_node, String::new());
        &self.codes
    }

    fn dfs(&mut self, op_node: &Link, cur: String) {
        if op_node.is_none() {
            return;
        }
        let node = op_node.as_ref().unwrap();
        if let Some(c) = node.ch {
            self.codes.insert(c, cur);
            return;
        }
        self.dfs(&node.left, cur.clone() + "0");
        self.dfs(&node.right, cur.clone() + "1");
    }
}

fn char_ratio<P: AsRef<Path>>(fp: P) -> io::Result<HashMap<char, usize>> {
    let mut file = File::open(fp)?;
    let mut buf = [0_u8; 1024];
    let mut num_map = HashMap::new();
    while let Ok(len) = file.read(&mut buf) {
        if len == 0 {
            break;
        }
        for i in 0..len {
            let c = buf[i] as char;
            *num_map.entry(c).or_insert(0) += 1;
        }
    }

    Ok(num_map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_char_ratio() {
        match char_ratio("/home/sr/.config/alacritty/alacritty.toml") {
            Ok(ok) => println!("{ok:?}"),
            Err(err) => eprintln!("{err:?}"),
        }
    }

    #[test]
    fn test_haffman() {
        let ratio_map =
            char_ratio("/home/sr/my_dev/rust-learning-journey/data-structures/tree/target/sys.log")
                .unwrap();
        for (k, v) in &ratio_map {
            println!("{k}:{v}")
        }

        println!("=====================================================");
        let mut haff = Haffman::new(ratio_map);
        for (k, v) in haff.generate_huffman_codes() {
            println!("{k}:{v}")
        }
    }
}
