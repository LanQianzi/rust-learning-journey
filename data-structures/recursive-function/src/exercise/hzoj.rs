pub fn hzoj_184(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    (hzoj_184(n - 1) + 1) * 2
}

pub fn hzoj_186(pot: usize, arr: Vec<usize>) -> usize {
    if pot >= arr.len() {
        return 0;
    }
    hzoj_186(pot + arr[pot], arr) + 1
}

fn print_235(arr: &Vec<usize>, i: usize) {
    for pot in 0..(i + 1) {
        print!("{} ", arr[pot]);
    }
    println!("")
}

pub fn hzoj_235(arr: &mut Vec<usize>, i: usize, j: usize, n: usize) {
    if j > n {
        return;
    }
    for k in j..(n + 1) {
        arr[i] = k;
        print_235(arr, i);
        hzoj_235(arr, i + 1, k + 1, n);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_184() {
        assert_eq!(hzoj_184(2), 4);
        assert_eq!(hzoj_184(3), 10);
        assert_eq!(hzoj_184(4), 22);
    }

    #[test]
    fn test_186() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(hzoj_186(0, arr), 3);
        let arr = vec![1, 2, 3, 4, 5, 1, 1, 1];
        assert_eq!(hzoj_186(0, arr), 4);
    }

    #[test]
    fn test_235() {
        let mut arr = vec![0usize; 10];
        hzoj_235(&mut arr, 0, 1, 6);
    }
}
