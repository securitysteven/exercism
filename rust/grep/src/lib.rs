use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Flags {
    show_line_numbers: bool,
    list_files_only: bool,
    case_insensitive: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut show_line_numbers = false;
        let mut list_files_only = false;
        let mut case_insensitive = false;

        // Iterate over the flags and set the appropriate options
        for &flag in flags {
            match flag {
                "-n" => show_line_numbers = true,
                "-l" => list_files_only = true,
                "-i" => case_insensitive = true,
                _ => {}
            }
        }

        // If both flags are provided, prioritize `-l` over `-n`
        if list_files_only {
            show_line_numbers = false; // Invalidate `-n` if `-l` is present
        }

        Flags {
            show_line_numbers,
            list_files_only,
            case_insensitive,
        }
    }

    pub fn should_show_line_numbers(&self) -> bool {
        self.show_line_numbers
    }

    pub fn should_list_files_only(&self) -> bool {
        self.list_files_only
    }

    pub fn is_case_insensitive(&self) -> bool {
        self.case_insensitive
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>> {
    let mut results = Vec::new();

    for &file in files {
        let path = Path::new(file);

        // Attempt to open the file
        let file = File::open(path)
            .with_context(|| format!("Failed to open file: {}", file))?;

        let reader = io::BufReader::new(file);
        let mut found_match = false;

        // If case-insensitive flag is set, make the pattern lowercase
        let pattern = if flags.is_case_insensitive() {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        // Iterate through each line of the file
        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;

            // Perform case-insensitive match if the flag is set
            let line_to_check = if flags.is_case_insensitive() {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            // Check if the line contains the pattern
            if line_to_check.contains(&pattern) {
                found_match = true;
                if flags.should_show_line_numbers() {
                    // If `-n` flag is set, include line numbers in the result
                    results.push(format!("{}:{}:{}", path.display(), line_number + 1, line));
                }
            }
        }

        // If `-l` flag is set, we only care about filenames, not line numbers
        if flags.should_list_files_only() && found_match {
            results.push(path.display().to_string());
        }
    }

    Ok(results)
}