
use std::thread;

#[derive(Debug)]
struct City {
    name: String,
    key: i32
}

impl City {
    fn new(name: &str) -> Self {
        let k = rand::random_range(0..100);
        City { name: name.to_string(), key: k }
    }

}

pub fn ef1() {
    let citys = vec![
        City::new("aaa"), 
        City::new("bbb"), 
        City::new("CCC"),
        City::new("fff"),
        City::new("333"),
        City::new("2233"),
                                ];
    let v = sort_thread(citys);
    println!("执行完成!");
    v.join();
}

fn sort_thread(mut citys: Vec<City>) -> thread::JoinHandle<Vec<City>>{
    let fn_key = |city: &City| -> i32 {
        city.key
    };
    thread::spawn(move || {
        citys.sort_by_key(fn_key);
        println!("{citys:?}\n");
        citys
    })
}