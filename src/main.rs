use std::fs;

fn main() {
    let input = read_input().unwrap();
    //loop over each line in the input and print the first and last number
    let mut sum = 0;
    for line in input.lines() {
        let first = find_first_number(&line).unwrap();
        let last = find_last_number(&line).unwrap();
        println!("Line: {line} {} {}", first, last);
        sum = sum + (first*10 + last);
    }
    println!("Sum: {}", sum);
}

fn read_input() -> std::io::Result<String> {
    fs::read_to_string("./src/input-1")
}

// declare a map of substrings to numbers to values
static NUM_MAP:[(&str, u32); 20] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("0", 0),
];

// itereate over each character in a string and find any of a set of substrings
fn find_first_number(input: &str) -> Option<u32> {
    let mut result: (Option<usize>, u32) = (None, 0);
    for s in NUM_MAP {
        let position = input.find(s.0);
        if position.is_some() && (result.0.is_none() || position < result.0) {
            result = (position, s.1);
        }
    }
    if result.0.is_none() {
        return None;
    }
    Some(result.1)
}

fn find_last_number(input: &str) -> Option<u32> {
    let mut result: (Option<usize>, u32) = (None, 0);
    for s in NUM_MAP {
        let position = input.rfind(s.0);
        if position.is_some() && (result.0.is_none() || position > result.0) {
            result = (position, s.1);
        }
    }
    if result.0.is_none() {
        return None;
    }
    Some(result.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_number() {
        assert_eq!(find_first_number("one"), Some(1));
        assert_eq!(find_first_number("asas one two three"), Some(1));
        assert_eq!(find_first_number("1one two three four"), Some(1));
        assert_eq!(find_first_number("sasdaslkone1"), Some(1));
    }

    #[test]
    fn test_find_last_number() {
        assert_eq!(find_last_number("one"), Some(1));
        assert_eq!(find_last_number("asas one two three"), Some(3));
        assert_eq!(find_last_number("1one two three four"), Some(4));
        assert_eq!(find_last_number("sasdaslkone1"), Some(1));
    }
}