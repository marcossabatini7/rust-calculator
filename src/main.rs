use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.next().unwrap();
    let second = args.next().unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    print!("{}", first_number);
    print!("{:?}", operator);
    print!("{}", second_number);
}
