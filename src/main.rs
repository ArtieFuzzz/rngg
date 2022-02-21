#![feature(is_some_with)]
use rand::Rng;
use std::{env, num::IntErrorKind};

fn main() {
    let arg = env::args().nth(1).unwrap_or("".to_string());

    if arg.is_empty() {
        return println!("You must specify a number");
    }

    let limit = arg.parse::<i32>();

    if limit.is_err_with(|e| e.kind() == &IntErrorKind::PosOverflow) {
        return println!("You have reached the limit for i32 positive numbers");
    }

    let num = rand::thread_rng().gen_range(0..limit.unwrap());

    println!("{}", num);
}
