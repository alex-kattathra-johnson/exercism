use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter().filter_map(|possible_anagram| is_annagram(word, possible_anagram)).collect()
}

pub fn is_annagram<'a>(word: &str, possible_anagram: &'a str) -> Option<&'a str> {
    let mut word_chars = word.to_lowercase().chars().collect::<Vec<char>>();
    let mut possible_anagram_chars = possible_anagram.to_lowercase().chars().collect::<Vec<char>>();
    if word_chars == possible_anagram_chars {
        return None
    }

    word_chars.sort();
    possible_anagram_chars.sort();
    if word_chars == possible_anagram_chars {
        return Some(possible_anagram);
    }
    None
}