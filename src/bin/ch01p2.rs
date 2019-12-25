/**
 * Check Permutation:
 * Given two strings, write a method to decide if one is a permutation of the other.
 */
/**
 * Iterate through both strings
 * Keep a count of how many times each character appears
 * If the counts are the same at the end of the loop, strings are a permutation, return true
 */
use std::collections::HashMap;
fn count_chars(s: &str) -> HashMap<char,i32> {
    let mut characters: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        if characters.contains_key(&c) {
            if let Some(x) = characters.get_mut(&c) {
                *x += 1;
            }
        } else {
            characters.insert(c, 1);
        }
    }
    return characters;
}
fn is_permutation(s1: &str, s2: &str) -> bool {
    let characters_1 = count_chars(s1);
    let characters_2 = count_chars(s2);

    for key in characters_1.keys() {
        if !characters_2.contains_key(&key) {
            return false;
        }
        if characters_1.get(&key) != characters_2.get(&key) {
            return false;
        }
    }
    for key in characters_2.keys() {
        if !characters_1.contains_key(&key) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("tac")),
            true
        );
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("dog")),
            false
        );
    }
}
fn main() {
    println!(
        "{}",
        is_permutation(&String::from("cat"), &String::from("tac"))
    );
}
