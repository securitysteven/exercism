pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string(); // Convert the number to a string
    let num_digits = num_str.len() as u32; // Get the number of digits

    // Sum the powers of each character converted to a digit
    let sum_of_powers: u32 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits)) // Convert char to digit and raise to num_digits
        .sum();

    sum_of_powers == num // Return true if sum_of_powers equals the original number
}
