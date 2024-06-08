fn main() {
    let num1 = 10;
    let num2 = 2;

    println!("Addition result is: {}", calculate(num1, num2, '+'));
    println!("Subtraction result is: {}", calculate(num1, num2, '-'));
    println!("Multiplication result is: {}", calculate(num1, num2, '*'));
    println!("Division result is: {}", calculate(num1, num2, '/'));
    println!("Modulus result is: {}", calculate(num1, num2, '%'));
}

fn calculate(num1: i32, num2: i32, operator: char) -> i32 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '%' => num1 % num2,
        _ => {
            println!("Invalid operator");
            0
        }
    }
}
