/// 给定 s 和 t 两个字符串，当它们分别被输入到空白的文本编辑器后，如果两者相等，返回 true 。# 代表退格字符。

pub fn backspace_compare(s: String, t: String) -> bool {
    let mut sv = Vec::new();
    let mut tv = Vec::new();
    for c in s.chars() {
        if c == '#' {
            sv.pop();
        } else {
            sv.push(c);
        }
    }

    for c in t.chars() {
        if c == '#' {
            tv.pop();
        } else {
            tv.push(c);
        }
    }

    sv == tv
}

#[cfg(test)]
mod test {
    use super::*;

    // 输入：s = "ab#c", t = "ad#c"
    // 输出：true
    // 解释：s 和 t 都会变成 "ac"。
    #[test]
    fn case1() {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        assert_eq!(backspace_compare(s, t), true);
    }

    // 输入：s = "ab##", t = "c#d#"
    // 输出：true
    // 解释：s 和 t 都会变成 ""。
    #[test]
    fn case2() {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        assert_eq!(backspace_compare(s, t), true);
    }

    // 输入：s = "a#c", t = "b"
    // 输出：false
    // 解释：s 会变成 "c"，但 t 仍然是 "b"。
    #[test]
    fn case3() {
        let s = "a#c".to_string();
        let t = "b".to_string();
        assert_eq!(backspace_compare(s, t), false);
    }
}
