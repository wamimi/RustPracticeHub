// My First attempt at executing a simple calculator

fn main() {
    let num1 = 10;
    let num2 = 2;

    println!("Addition result is: {}", addition(num1, num2));
    println!("Subtraction result is: {}", subtraction(num1, num2));
    println!("Multiplication result is: {}", multiplication(num1, num2));
    println!("Division result is: {}", division(num1, num2));
    println!("Modulus result is: {}", modulus(num1, num2));
}

fn addition(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtraction(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn division(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

fn modulus(num1: i32, num2: i32) -> i32 {
    num1 % num2
}
