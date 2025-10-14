pub fn stage_multiplication(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        stage_multiplication(n - 1) * n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sm() {
        let n = 10;
        println!("n{} = {}", n, stage_multiplication(n))
    }
}
