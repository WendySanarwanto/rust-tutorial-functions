use std::io;

fn do_add(left: i32, right: i32) -> i32 {
    left + right
}

fn print_total(total: i32) {
    println!("The total of number addition is {}", total);
}

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
