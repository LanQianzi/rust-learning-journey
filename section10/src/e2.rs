use rand::Rng;
pub fn ef2() {
    f1();
}

fn f1() {
    let number = rand::random_range(0..6);
    match number {
        0 => {},
        1 => println!("A rabbit is nosing around in th clover."),
        n => println!("There are {n} rabbits hopping about in the meadow."),
    }
}

struct Print {
    x: i32,
    y: i32,
    ss: String
}

fn f4() {
    let p1 = Print{x: 20, y: 30, ss: "aaa".to_string()};
    match p1 {
        Print{x, y, ss} => {
            println!("x: {x}, y: {y}");
        },
    }
    // println!("{}", p1.ss)
}