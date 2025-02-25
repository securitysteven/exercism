pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Filter out 0 values from the factors
    let factors = factors.iter().filter(|&&factor| factor != 0).collect::<Vec<_>>();

    let mut multiples = std::collections::HashSet::new();

    // Loop through each factor
    for &factor in factors {
        let mut current_value = factor;

        // Loop to generate multiples of the factor within the limit
        while current_value < limit {
            multiples.insert(current_value);  // Insert the multiple into the HashSet
            current_value += factor;         // Move to the next multiple
        }
    }

    // Sum all the unique multiples
    multiples.iter().copied().sum()
}