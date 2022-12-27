use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let words = words.split([' ', ',', '\n']);
    for word in words {
        let word = word.to_ascii_lowercase();
        let mut word = word.replace(&['"', '(', ')', ',', '!', '.', ';', ':', '@', '$', '&', '^', '%'], "");

        if word.starts_with('\'') && word.ends_with('\'') {
            word.pop();
            word.remove(0);
        }
        // This seems shady

        if word == String::from("") { continue }
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    counts
}
