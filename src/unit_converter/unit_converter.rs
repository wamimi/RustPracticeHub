use std::io; // Import the `io` module from the standard library for handling input and output.

fn main() {
    loop { // Start an infinite loop to keep the program running until the user decides to exit.
        println!("Unit Converter");
        println!("1. Kilometers to Miles");
        println!("2. Miles to Kilometers");
        println!("3. Celsius to Fahrenheit");
        println!("4. Fahrenheit to Celsius");
        println!("5. Exit");
        println!("Enter your choice:"); // Display the menu options to the user.

        let mut choice = String::new(); // Create a mutable String to store the user's choice.
        io::stdin().read_line(&mut choice).expect("Failed to read line"); // Read the user's input and store it in `choice`.
        let choice: u32 = choice.trim().parse().expect("Please enter a number"); // Parse the input as a u32 number.

        match choice { // Match the user's choice to the corresponding conversion function.
            1 => convert_km_to_miles(), // Call the function to convert kilometers to miles.
            2 => convert_miles_to_km(), // Call the function to convert miles to kilometers.
            3 => convert_celsius_to_fahrenheit(), // Call the function to convert Celsius to Fahrenheit.
            4 => convert_fahrenheit_to_celsius(), // Call the function to convert Fahrenheit to Celsius.
            5 => {
                println!("Exiting..."); // Print exit message.
                break; // Exit the loop and terminate the program.
            }
            _ => println!("Invalid choice, please try again."), // Handle invalid choices.
        }
    }
}

fn convert_km_to_miles() {
    let km = get_input("Enter kilometers:"); // Get the number of kilometers from the user.
    let miles = km * 0.621371; // Convert kilometers to miles.
    println!("{:.2} kilometers is {:.2} miles", km, miles); // Print the result.
}

fn convert_miles_to_km() {
    let miles = get_input("Enter miles:"); // Get the number of miles from the user.
    let km = miles / 0.621371; // Convert miles to kilometers.
    println!("{:.2} miles is {:.2} kilometers", miles, km); // Print the result.
}

fn convert_celsius_to_fahrenheit() {
    let celsius = get_input("Enter Celsius:"); // Get the temperature in Celsius from the user.
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0; // Convert Celsius to Fahrenheit.
    println!("{:.2} Celsius is {:.2} Fahrenheit", celsius, fahrenheit); // Print the result.
}

fn convert_fahrenheit_to_celsius() {
    let fahrenheit = get_input("Enter Fahrenheit:"); // Get the temperature in Fahrenheit from the user.
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0; // Convert Fahrenheit to Celsius.
    println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius); // Print the result.
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt); // Print the prompt message.
    let mut input = String::new(); // Create a mutable String to store the user's input.
    io::stdin().read_line(&mut input).expect("Failed to read line"); // Read the user's input and store it in `input`.
    input.trim().parse().expect("Please enter a valid number") // Parse the input as a f64 number.
}
