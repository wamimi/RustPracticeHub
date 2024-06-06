// My Second  attempt at executing a simple calculator using enums and match
fn main() {
    let num1 = 10;
    let num2 = 2;

    println!("Addition result is: {}", calculator(num1, num2, Operation::Add));
    println!("Subtraction result is: {}",calculator(num1, num2,Operation::Substract ));
    println!("Multiplication result is: {}",calculator(num1, num2, Operation::Divide));
    println!("Division result is: {}", calculator(num1, num2, Operation::Multiply));
    println!("Modulus result is: {}", calculator(num1, num2, Operation::Modulus));
}

enum Operation {
    Add,
    Substract,
    Divide,
    Multiply,
    Modulus,
}
fn calculator(num1:i32, num2:i32, operation:Operation) -> i32{
    match operation{
        Operation::Add => num1 + num2,
        Operation::Substract => num1 - num2,
        Operation::Divide => num1 / num2,
        Operation::Multiply => num1 * num2,
        Operation::Modulus => num1 % num2,

    }


}    


