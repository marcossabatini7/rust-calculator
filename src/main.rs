use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.next().unwrap();
    let second = args.next().unwrap();
}
