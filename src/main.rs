use calculator_rust::{banner, extract_numbers, extract_operators, validate_equation};
use std::io;

fn main() {
    banner();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    println!();
    println!("═══════════════════════════════════════════════════");

    // Validate the equation
    if validate_equation(trimmed_input) {
        // Extract operators and numbers
        let operators = extract_operators(trimmed_input);
        let numbers = extract_numbers(trimmed_input);

        // Print the results with decorative formatting
        println!("\n  ✓ Valid equation detected!");
        println!("\n  📊 Analysis:");
        println!("  ├─ Operators found: {:?}", operators);
        println!("  └─ Numbers found:   {:?}", numbers);
        println!();
    } else {
        println!("\n  ✗ Invalid equation format!");
        println!("  💡 Tip: Please enter equation like 3+5*2 or 10/2-3");
        println!();
    }

    println!("═══════════════════════════════════════════════════\n");
}
