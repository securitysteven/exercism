use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Helper function to get a sorted version of the word
    fn sort_word(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    }

    // Get the sorted version of the target word
    let sorted_word = sort_word(word);

    // Create a HashSet to store valid anagrams
    let mut anagrams = HashSet::new();

    // Check each possible anagram
    for &candidate in possible_anagrams {
        if sorted_word == sort_word(candidate) {
            anagrams.insert(candidate);
        }
    }
    anagrams
}
