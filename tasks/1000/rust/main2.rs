use std::io;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("correct input");

    let a: Vec<i32> = input.trim()
                           .split(' ')
                           .map(|a| a.trim().parse::<i32>().expect("parsed integer"))
                           .collect();

    println!("{}", a[0] + a[1]);
}
