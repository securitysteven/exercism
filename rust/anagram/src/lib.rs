use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    // Helper function to get a sorted version of the word
    fn sorted_word(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    }

    // Get the sorted version of the target word
    let sort_word = sorted_word(word);

    // Create a HashSet to store valid anagrams
    let mut anagrams = HashSet::new();

    // Check each possible anagram
    for &candidate in possible_anagrams {
        if sort_word == sorted_word(candidate) {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
