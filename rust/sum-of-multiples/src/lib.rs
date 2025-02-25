use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Filter out 0 values and process only valid factors
    let valid_factors = factors.iter().filter(|&&factor| factor != 0);

    // Use HashSet to store unique multiples
    let mut multiples = HashSet::new();

    // Loop through each valid factor
    for &factor in valid_factors {
        // Generate multiples of the factor within the limit
        for multiple in (factor..limit).step_by(factor as usize) {
            multiples.insert(multiple);
        }
    }

    // Return the sum of all unique multiples
    multiples.iter().copied().sum()
}