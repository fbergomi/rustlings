// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut isFirst = true;
    let mut result:String = String::new();

    loop {
        match c.next() {
            None => break,
            Some(current_char) => { 
                if isFirst {
                    let first_char_str = String::from(current_char).to_uppercase();
                    result.push_str(&first_char_str);
                    isFirst = false;
                }
                else{
                    result.push(current_char);
                }
            }
        }
    }

    result
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let iterator = words.iter();
    for current in iterator {
        let tmp = capitalize_first(&current);
        result.push(tmp);
    }
    result
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut capitalizedVector = capitalize_words_vector(words);

    let iterator = capitalizedVector.iter();
    let mut result: String = String::new();

    for current in iterator {
        result.push_str(&current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
