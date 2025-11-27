/// Validates if the input string is a valid math equation
/// Returns true if it contains numbers and operators in a valid pattern
pub fn validate_equation(input: &str) -> bool {
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
pub fn extract_operators(input: &str) -> Vec<char> {
    input
        .chars()
        .filter(|c| matches!(c, '+' | '-' | '*' | '/'))
        .collect()
}

/// Extracts all numbers from the input string
pub fn extract_numbers(input: &str) -> Vec<String> {
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

/// Evaluates a mathematical expression and returns the result
/// Supports +, -, *, / operators with proper precedence (PEMDAS)
///
/// # Arguments
/// * `input` - A string slice containing the mathematical equation
///
/// # Returns
/// * `Ok(f64)` - The calculated result
/// * `Err(String)` - An error message if evaluation fails
///
/// # Examples
/// ```
/// let result = evaluate("3+5*2"); // Returns Ok(13.0)
/// let result = evaluate("10/2-3"); // Returns Ok(2.0)
/// ```
pub fn evaluate(input: &str) -> Result<f64, String> {
    if !validate_equation(input) {
        return Err("Invalid equation format".to_string());
    }

    let numbers = extract_numbers(input);
    let operators = extract_operators(input);

    // Parse numbers into f64
    let mut nums: Vec<f64> = Vec::new();
    for num_str in numbers {
        match num_str.parse::<f64>() {
            Ok(n) => nums.push(n),
            Err(_) => return Err(format!("Invalid number: {}", num_str)),
        }
    }

    if nums.is_empty() {
        return Err("No numbers found in equation".to_string());
    }

    if nums.len() != operators.len() + 1 {
        return Err("Mismatch between numbers and operators".to_string());
    }

    // First pass: handle multiplication and division (left to right)
    let mut values = vec![nums[0]];
    let mut ops = Vec::new();

    for (i, op) in operators.iter().enumerate() {
        match op {
            '*' => {
                let last = values.pop().unwrap();
                values.push(last * nums[i + 1]);
            }
            '/' => {
                let last = values.pop().unwrap();
                if nums[i + 1] == 0.0 {
                    return Err("Division by zero".to_string());
                }
                values.push(last / nums[i + 1]);
            }
            '+' | '-' => {
                values.push(nums[i + 1]);
                ops.push(*op);
            }
            _ => return Err(format!("Unknown operator: {}", op)),
        }
    }

    // Second pass: handle addition and subtraction (left to right)
    let mut result = values[0];
    for (i, op) in ops.iter().enumerate() {
        match op {
            '+' => result += values[i + 1],
            '-' => result -= values[i + 1],
            _ => return Err(format!("Unknown operator: {}", op)),
        }
    }

    Ok(result)
}

// Decoration banner
pub fn banner() {
    // Print decorative welcome banner
    println!("\n╔═══════════════════════════════════════════════════╗");
    println!("║                                                     ║");
    println!("║          🧮  RUST CALCULATOR v1.0  🧮              ║");
    println!("║                                                     ║");
    println!("║     Your friendly mathematical companion!           ║");
    println!("║                                                     ║");
    println!("╚═════════════════════════════════════════════════════╝");
    println!();

    // Decorative input prompt
    println!("┌───────────────────────────────────────────────────┐");
    println!("│  Please enter your math equation:                 │");
    println!("│  (e.g., 3+5*2, 10/2-3, 15.5+8.2)                  │");
    println!("└───────────────────────────────────────────────────┘");
}
