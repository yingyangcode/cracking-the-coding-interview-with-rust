// Is Unique: Implement an algorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?
fn all_unique_chars_part_a(s: &str) -> bool {
    use std::collections::HashSet;
    let mut characters: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if characters.contains(&c) {
            return false;
        }
        characters.insert(c);
    }
    return true; 
}

fn all_unique_chars_part_b(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    // integer representation of the 'a' char
    let a_int_char: i16 = 'a' as i16;
    for c in s.chars(){
        // Convert char to an integer representation
        let mut int_char : i16 = c as i16;
        int_char -= a_int_char;
        
        // Check if bit is set in bitfield with a bitwise AND
        // If bit is set, char was already present, string not unique, return false
        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        // set bit of current char with a bitwise OR
        // Take the number 1 and rotate it by int_char bits
        bitfield |= 1 << int_char;
    }
    return true;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(all_unique_chars_part_a(&String::from("abcdefg")), true);
        assert_eq!(all_unique_chars_part_a(&String::from("abcdefga")), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_unique_chars_part_b(&String::from("abcdefg")), true);
        assert_eq!(all_unique_chars_part_b(&String::from("abcdefga")), false);
    }
}

fn main() {
    println!("{}", all_unique_chars_part_a(&String::from("helloworld")));
    println!("{}", all_unique_chars_part_b(&String::from("helloworld")));
}
