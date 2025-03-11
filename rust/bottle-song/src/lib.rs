/// Generates the lyrics for the "99 Bottles of Beer" song.
///
/// # Arguments
/// * `start_bottles` - The number of bottles to start with.
/// * `take_down` - The number of verses to generate.
///
/// # Returns
/// A formatted string containing the lyrics.
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles.saturating_sub(i))) // Prevents underflow
        .collect::<Vec<_>>()
        .join("\n\n") // Ensures correct paragraph spacing
}

/// Generates a single verse of the song based on the number of bottles remaining.
///
/// # Arguments
/// * `n` - The current number of bottles.
///
/// # Returns
/// A string containing the formatted verse.
fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
              Go to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
              Take it down and pass it around, no more bottles of beer on the wall.".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
              Take one down and pass it around, 1 bottle of beer on the wall.".to_string(),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottles of beer on the wall.",
            n,
            n - 1
        ),
    }
}

/// Main function for testing purposes.
fn main() {
    let lyrics = recite(99, 5); // Generates 5 verses starting from 99
    println!("{}", lyrics);
}