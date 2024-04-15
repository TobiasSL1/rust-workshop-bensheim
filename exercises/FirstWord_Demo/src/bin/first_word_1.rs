// Here’s a small programming problem: write a function that takes a string of words separated by
// spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word,
// so the entire string should be returned.


fn main() {
    let mut s = String::from("hello world");

    let end_index_first_word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("End index of first word is: {}", end_index_first_word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
