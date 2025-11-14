use crate::queue_arr::Queue;
use std::{i32, mem};

/// 定义三元组（a,b, c）（a,b,c 均为正数）的距离 D=|a-b|+|b-c|+|c-a|。
/// 给定 3 个非空整数集合 S1, S2 ,S3, 按升序分别存储在 3 个数组中。
/// 请设计一个尽可能高效的算法，计算并输出所有可能的三元组（a, b, c）（a∈S1,b∈S2,c∈S3）中的最小距离。
/// 例如 S1={-1, 0, 9}, S2={-25，-10，10，11}，S3={2，9，17，30，41}，则最小距离为 2，相应的三元组为（9，10，9）。
fn distance(a: i32, b: i32, c: i32) -> i32 {
    (a - b).abs() + (b - c).abs() + (c - a).abs()
}

fn min_num(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    if a > b {
        mem::swap(&mut a, &mut b);
    }
    if a > c {
        mem::swap(&mut a, &mut c);
    }
    a
}

pub fn triple(mut que1: Queue<i32>, mut que2: Queue<i32>, mut que3: Queue<i32>) -> i32 {
    let mut ans = i32::MAX;
    while !que1.is_empty() && !que2.is_empty() && !que3.is_empty() {
        let a = que1.front().unwrap().clone();
        let b = que2.front().unwrap().clone();
        let c = que3.front().unwrap().clone();
        let temp_ans = distance(a, b, c);
        if ans > temp_ans {
            ans = temp_ans;
        }

        let min = min_num(a, b, c);
        if a == min {
            que1.pop();
        }
        if b == min {
            que2.pop();
        }
        if c == min {
            que3.pop();
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    fn vec_as_que<T>(ves: Vec<T>) -> Queue<T> {
        let mut que = Queue::new(ves.len());
        for v in ves {
            que.push(v).unwrap();
        }
        que
    }

    #[test]
    /// 例如 S1={-1, 0, 9}, S2={-25，-10，10，11}，S3={2，9，17，30，41}，则最小距离为 2，相应的三元组为（9，10，9）。
    fn case1() {
        let que1 = vec_as_que(vec![-1, 0, 9]);
        let que2 = vec_as_que(vec![-25, -10, 10, 11]);
        let que3 = vec_as_que(vec![2, 9, 17, 30, 41]);
        assert_eq!(triple(que1, que2, que3), 2);
    }

    #[test]
    fn case2() {
        let que1 = vec_as_que(vec![10]); // S1（单元素）
        let que2 = vec_as_que(vec![5, 10, 15]); // S2
        let que3 = vec_as_que(vec![8, 12]); // S3
        // 最小距离来自(10,10,8)或(10,10,12)，距离均为4
        assert_eq!(triple(que1, que2, que3), 4);
    }
}
