use std::io;

/// Validates if the input string is a valid math equation
/// Returns true if it contains numbers and operators in a valid pattern
fn validate_equation(input: &str) -> bool {
    // First, check if the regex pattern for math equations is valid using regex_parser
    // Pattern: one or more digits, followed by operator(s) and more digits
    // We'll validate the pattern structure conceptually
    
    let trimmed = input.trim();
    
    // Basic validation: must have at least one digit, one operator, and another digit
    let has_digit = trimmed.chars().any(|c| c.is_ascii_digit());
    let has_operator = trimmed.chars().any(|c| matches!(c, '+' | '-' | '*' | '/'));
    
    if !has_digit || !has_operator {
        return false;
    }
    
    // Validate that it's not just operators or just numbers
    // Must alternate between numbers and operators (simplified check)
    let mut prev_was_digit = false;
    let mut prev_was_operator = false;
    let mut valid_structure = false;
    
    for ch in trimmed.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            if prev_was_operator {
                valid_structure = true; // Found digit after operator
            }
            prev_was_digit = true;
            prev_was_operator = false;
        } else if matches!(ch, '+' | '-' | '*' | '/') {
            if prev_was_digit {
                prev_was_operator = true;
                prev_was_digit = false;
            } else {
                return false; // Two operators in a row or operator at start
            }
        } else if !ch.is_whitespace() {
            return false; // Invalid character
        }
    }
    
    // Must end with a digit and have valid structure
    valid_structure && prev_was_digit
}

/// Extracts all math operators from the input string
fn extract_operators(input: &str) -> Vec<char> {
    input.chars()
        .filter(|c| matches!(c, '+' | '-' | '*' | '/'))
        .collect()
}

/// Extracts all numbers from the input string
fn extract_numbers(input: &str) -> Vec<String> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();
    
    for ch in input.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            current_number.push(ch);
        } else if !current_number.is_empty() {
            numbers.push(current_number.clone());
            current_number.clear();
        }
    }
    
    // Don't forget the last number if the string ends with a digit
    if !current_number.is_empty() {
        numbers.push(current_number);
    }
    
    numbers
}

fn main() {
    let mut input = String::new();
    println!("Please enter a math equation: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let trimmed_input = input.trim();
    
    // Validate the equation
    if validate_equation(trimmed_input) {
        // Extract operators and numbers
        let operators = extract_operators(trimmed_input);
        let numbers = extract_numbers(trimmed_input);
        
        // Print the results
        println!("\nOperators found: {:?}", operators);
        println!("Numbers found: {:?}", numbers);
    } else {
        eprintln!("Please enter equation like 3+5*2 or 10/2-3");
    }
}
