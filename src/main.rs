use calculator_rust::{banner, evaluate, extract_numbers, extract_operators, validate_equation};
use std::io;

fn main() {
    banner();
    loop {
        println!("➤  ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.to_lowercase() == "exit" {
            println!("Goodbye! Have a nice day!");
            break;
        }

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
            println!("  ├─ Numbers found:   {:?}", numbers);

            // Evaluate the equation
            let result = evaluate(&trimmed_input);
            println!("  └─ Result:   {:?}", result);

            println!();
        } else {
            println!("\n  ✗ Invalid equation format!");
            println!("  💡 Tip: Please enter equation like 3+5*2 or 10/2-3");
            println!();
        }

        println!("═══════════════════════════════════════════════════\n");
    }
}
