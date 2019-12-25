/**
 * URLify: Write a method to replace all spaces in a string with '%20'.
 * You may assume that the string has sufficient space at the end
 * to hold the additional characters, and that you are given the "true" length of the string.
 * (Note: If implementing in Java, please use a character array
 * so that you can perform this operation in place.)
*/
/**
 * Split the string on whitespace
 * Initialize the accumulator with the first slice
 * Insert placeholder before next iteration of the remaining slices
*/

fn urlify(url: &'static str) -> String {
    let placeholder = "%20";

    url.split_whitespace().fold(String::new(), |acc, s| {
        if acc.is_empty() {
            String::from(s)
        } else {
            acc + placeholder + s
        }
    })
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify(&"Mr John Smith    "), "Mr%20John%20Smith");
    }
}
fn main() {
    println!("{}", urlify(&"Mr John Smith    "));
}
