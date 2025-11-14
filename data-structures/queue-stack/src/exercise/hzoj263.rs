use std::fmt::Debug;

/// 有 n 列火车按 1 到 n
/// 的顺序从东方左转进站，这个车站是南北方向的，它虽然无限长，只可惜是一个死胡同，
/// 而且站台只有一条股道，火车只能倒着从西方出去，而且每列火车必须进站，先进后出。

fn recursive_enum<T: Clone + Debug + Eq + Ord>(
    cur: Vec<T>,
    rest: Vec<T>,
    result: &mut Vec<Vec<T>>,
) {
    if rest.is_empty() {
        result.push(cur);
        return;
    }

    for i in 0..rest.len() {
        let mut new_cur = cur.clone();
        new_cur.push(rest[i].clone());
        // 2. 构建新的剩余元素（排除第i个元素）
        let new_rest: Vec<T> = rest[0..i] // 取第i个元素之前的部分
            .iter()
            .cloned()
            .chain(rest[i + 1..].iter().cloned()) // 拼接第i个元素之后的部分
            .collect();
        recursive_enum(new_cur, new_rest, result);
    }
}

fn is_stack_enum<T: Clone + Debug + Eq + Ord>(em: Vec<T>, rest: &Vec<T>) -> bool {
    let mut stack = Vec::new();
    let mut tmp_vec = rest.clone();
    tmp_vec.reverse();
    for v in em {
        while !tmp_vec.is_empty() && (stack.is_empty() || v > *stack.last().unwrap()) {
            stack.push(tmp_vec.pop().unwrap());
        }

        if v == *stack.last().unwrap() {
            stack.pop();
        } else {
            return false;
        }
    }

    true
}

pub fn train_in_stack(car_num: i32) {
    let mut rest = Vec::new();
    for i in 1..=car_num {
        rest.push(i);
    }
    let mut all_enum = Vec::new();
    recursive_enum(Vec::new(), rest.clone(), &mut all_enum);

    for em in all_enum {
        if is_stack_enum(em.clone(), &rest) {
            println!("{em:?}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_enum() {
        let mut result = Vec::new();
        recursive_enum(Vec::new(), vec![1, 2, 3], &mut result);
        for v in result {
            println!("{v:?}");
        }
    }

    #[test]
    fn case1() {
        train_in_stack(3);
    }

    #[test]
    fn case2() {
        train_in_stack(4);
    }
}
