use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.next().unwrap().chars().next().unwrap();
    let second = args.next().unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);

    println!("{}", output(operator, first_number, second_number, result));
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn output(operator: char, first_number: f32, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}
