// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words_available = HashMap::new();
    for word in magazine {
        let ocurrence = words_available.entry(word).or_insert(0);
        *ocurrence += 1;
    }
    for word in note {
        match words_available.entry(word).or_insert(0) {
            0 => return false,
            n => *n -= 1,
        };
    }
    return true
}
