use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('{', '}');
    map.insert('[', ']');
    let mut stack = Vec::new();
    for c in s.chars() {
        if map.contains_key(&c) {
            stack.push(c);
            continue;
        }

        let right = match stack.pop() {
            Some(left) => map.get(&left).unwrap(),
            None => &'n',
        };

        if c != *right {
            return false;
        }
    }

    if !stack.is_empty() {
        return false;
    }

    return true;
}
