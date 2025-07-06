use std::sync::{Arc, Mutex};
use std::thread;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn test_mutex() {
    let list = Arc::new(Mutex::new(vec![1]));
    let t1_list = list.clone();
    let t2_list = list.clone();
    let t1 = thread::spawn(move || {
        let mut v = t1_list.lock().unwrap();
        for i in 0..10 {
            v.push(i);
        }
    });

    let t2 = thread::spawn(move || {
        let mut v = t2_list.lock().unwrap();
        for i in 10..20 {
            v.push(i);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!("{list:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        test_mutex();
    }
}
