use std::io;

fn main() {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    // split input string
    let mut s = input.trim().split(' ');

    // read first number
    let a_str = s.next().unwrap();
    let a = a_str.trim().parse::<i32>().unwrap();

    // read second number
    let b_str = s.next().unwrap();
    let b = b_str.trim().parse::<i32>().unwrap();

    println!("{}", a + b);
}
