use std::io;

// Add 2 numbers, specified in method's arguments and returns the result of addition immediately.
fn do_add(left: i32, right: i32) -> i32 {
    left + right
}

// Prints the number specified in the method's argument.
fn print_total(total: i32) {
    println!("The total of number addition is {}", total);
}

// Wait for user's input, parse the entered input as number in a loop. It breaks from loop if the entered number is valid.
fn read_line_number() -> i32 {
    let result: i32;
    let stdin = io::stdin();

    loop {
        let mut number = String::new();
        stdin.read_line(&mut number)
            .expect("Failed to read input !");
        result = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    result
}

// The main function
fn main() {
    println!("\nRust Function's Demo.");
    println!("=====================\n");

    let left: i32;
    let right: i32;

    println!("Enter left-side number: ");
    left = read_line_number();

    println!("Enter right-side number: ");
    right = read_line_number();

    let total = do_add(left, right);
    
    print_total(total);
}
