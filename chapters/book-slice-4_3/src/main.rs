
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_word_end_index = s.len();
    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            first_word_end_index = i;
            break;
        }
    }
    &s[..first_word_end_index]
}

fn main() {
    let string = String::from("Hello world!");
    let word = first_word(&string);
    println!("{}", word); 
}

