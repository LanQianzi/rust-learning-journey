/// 946. 验证栈序列

/// 给定 pushed 和 popped 两个序列，每个序列中的 值都不重复，
/// 只有当它们可能是在最初空栈上进行的推入 push 和弹出 pop 操作序列的结果时，返回 true；否则，返回 false 。
pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    let mut re_pop = popped;
    re_pop.reverse();

    let mut stack = Vec::new();
    for ps in pushed {
        stack.push(ps);
        while !stack.is_empty() && stack.last() == re_pop.last() {
            stack.pop();
            re_pop.pop();
        }
    }
    stack.is_empty() && re_pop.is_empty()
}

#[cfg(test)]
mod test {
    use crate::exercise::leet_code946::validate_stack_sequences;

    /// 示例 1：
    /// 输入：pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
    /// 输出：true
    /// 解释：我们可以按以下顺序执行：
    /// push(1), push(2), push(3), push(4), pop() -> 4,
    /// push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
    #[test]
    fn case1() {
        assert_eq!(
            validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
            true
        )
    }

    /// 示例 2：
    /// 输入：pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
    /// 输出：false
    /// 解释：1 不能在 2 之前弹出。
    #[test]
    fn case2() {
        assert_eq!(
            validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
            false
        );
    }
}
