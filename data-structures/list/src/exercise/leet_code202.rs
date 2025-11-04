fn next(n: i32) -> i32 {
    let mut sum = 0;
    let mut num = n;
    while num != 0 {
        let tmp = num % 10;
        sum += tmp * tmp;
        num /= 10;
    }
    sum
}

pub fn is_happy(n: i32) -> bool {
    let mut num1 = n;
    let mut num2 = n;
    while num2 != 1 {
        num1 = next(num1);
        num2 = next(next(num2));
        if num1 == num2 && num2 != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(is_happy(10), true);
        assert_eq!(is_happy(2), false);
    }
}
