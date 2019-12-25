/**
 * Palindrome Permutation:
 * Given a string, write a function to check if it is a permutation of a palin-drome.
 * A palindrome is a word or phrase that is the same forwards and backwards.
 * A permutation is a rearrangement of letters.
 * The palindrome does not need to be limited to just dictionary words.
*/
/**
 * Split the string on whitespace
 * Initialize the accumulator with the first slice
 * Insert placeholder before next iteration of the remaining slices
*/
use std::collections::HashMap;
fn count_chars(s: &str) -> HashMap<char, i32> {
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

fn palindrome_permutation(s: &str) -> bool {
    let normalized_string = s
        .to_lowercase()
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s);
    let character_counts = count_chars(&normalized_string);
    let is_even = normalized_string.len() % 2 == 0;
    let mut has_odd = false;

    for value in character_counts.values() {
        if value % 2 != 0 {
            if is_even || has_odd {
                return false;
            } else {
                has_odd = true;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        assert_eq!(palindrome_permutation("Tact Coa"), true);
    }
}
fn main() {
    println!("{}", palindrome_permutation("Tact Coa"));
}
