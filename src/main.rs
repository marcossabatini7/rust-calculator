use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.next().unwrap().chars().next().unwrap();
    let second = args.next().unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    print!("{}", first_number);
    print!("{:?}", operator);
    print!("{}", second_number);

    operate(operator, first_number, second_number);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    if operator == '+' {
        return first_number + second_number;
    } else if operator == '-' {
        return first_number - second_number;
    } else if operator == '*' {
        return first_number * second_number;
    } else if operator == '/' {
        return first_number / second_number;
    } else {
        return 0.0;
    }
}
