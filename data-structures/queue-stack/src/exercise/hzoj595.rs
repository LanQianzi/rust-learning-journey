pub fn program_call(calls: Vec<String>, fc: &str) {
    let mut stack = Vec::new();
    for c in calls {
        if c == "return" {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    while let Some(f) = stack.pop() {
        if f == fc {
            break;
        }
    }

    if stack.is_empty() {
        println!("NOT REFERENCED");
        return;
    }

    let mut call = String::new();
    for f in stack {
        call.push_str(&f);
        call.push_str("->");
    }
    call.push_str(fc);
    println!("{call}")
}

#[cfg(test)]
mod test {
    use super::*;

    // 输入fun1() fun2() return fun3() fun4() | fun4()
    // 预计输出: fun1()->fun3()->fun4()
    #[test]
    fn case1() {
        let calls = vec![
            "fun1()".to_string(),
            "fun2()".to_string(),
            "return".to_string(),
            "fun3()".to_string(),
            "fun4()".to_string(),
        ];

        program_call(calls, "fun4()");
    }

    // 输入fun1() fun2() return fun3() fun4() fun5()  | fun3()
    // 预计输出: fun1()->fun3()
    #[test]
    fn case2() {
        let calls = vec![
            "fun1()".to_string(),
            "fun2()".to_string(),
            "return".to_string(),
            "fun3()".to_string(),
            "fun4()".to_string(),
            "fun5()".to_string(),
        ];

        program_call(calls, "fun3()");
    }

    // 输入fun1() fun2() return fun3() fun4() fun5()  | fun2()
    // 预计输出: NOT REFERENCED
    #[test]
    fn case3() {
        let calls = vec![
            "fun1()".to_string(),
            "fun2()".to_string(),
            "return".to_string(),
            "fun3()".to_string(),
            "fun4()".to_string(),
            "fun5()".to_string(),
        ];

        program_call(calls, "fun2()");
    }
}
