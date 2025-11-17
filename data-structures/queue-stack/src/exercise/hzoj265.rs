use std::collections::HashMap;

/// #265. 括号画家
/// 题目描述
/// Candela是一名漫画家，她有一个奇特的爱好，就是在纸上画括号。这一天，刚刚起床的 Candela
/// 画了一排括号序列，其中包含小括号 ()、中括号 [] 和大括号 {}，总长度为 N 。这排随意绘制的括号序列显得杂乱无章，于是 Candela
/// 定义了什么样的括号序列是美观的：

/// 1. 空的括号序列是美观的；
/// 2. 若括号序列 `A` 是美观的，则括号序列 `(A)、[A]、{A}` 也是美观的；
/// 3. 若括号序列 `A、B` 都是美观的，则括号序列 `AB` 也是美观的；
/// 例如 [(){}]() 是美观的括号序列，而 )({)[}]( 则不是。

/// 现在 Candela 想在她绘制的括号序列中，找出其中连续的一段，满足这段子序列是美观的，并且长度尽量大。你能帮帮她吗？

pub fn bracket_painter(brackets: &str) -> usize {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    let bac_vec: Vec<char> = brackets.chars().collect();
    let mut stack = Vec::new();
    let mut match_arr = Vec::new();
    match_arr.resize(bac_vec.len(), usize::MAX);

    for i in 0..bac_vec.len() {
        if map.contains_key(&bac_vec[i]) {
            stack.push(i);
        } else {
            // 栈顶元素对应的右括号是否等于判断的右括号
            let idx = match stack.last() {
                Some(idx) => *idx,
                None => {
                    stack.push(i);
                    continue;
                }
            }; // 获取栈顶元素的左括号的位置

            // 通过这个位置寻找对应的右括号
            let right = match map.get(&bac_vec[idx]) {
                Some(r) => *r,
                None => 'n',
            };

            if bac_vec[i] == right {
                // 当前要判断的右括号和栈顶元素对应的右括号是否相等
                match_arr[stack.pop().unwrap()] = i
            } else {
                stack.push(i);
            }
        }
    }
    // for i in 0..match_arr.len() {
    //     println!("{}:{}", i, match_arr[i])
    // }

    let mut ans = 0;
    let mut ans_temp = 0;
    let mut i = 0;
    while i < match_arr.len() {
        if match_arr[i] != usize::MAX {
            ans_temp += match_arr[i] - i + 1;
            i = match_arr[i] + 1;
        } else {
            i += 1;
            ans_temp = 0;
        }

        if ans_temp > ans {
            ans = ans_temp
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(bracket_painter("[[[[]]{}]]"), 10);
    }

    #[test]
    fn case2() {
        assert_eq!(bracket_painter(")[[[[]]{}))()()}}"), 6);
    }
}
