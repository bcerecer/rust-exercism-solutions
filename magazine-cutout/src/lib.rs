// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut available_words = HashMap::new();
    for word in magazine {
        let entry = available_words.entry(word).or_insert(0);
        *entry += 1;
    }

    for word in note {
        let entry = available_words.entry(word).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }
    true
}
