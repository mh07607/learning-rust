fn first_word(string: &String) -> String {
    let mut word = String::new();
    for &b in string.as_bytes() {
        if b == b' ' {
            break;
        }
        word.push(b as char);
    }
    word
}

fn main() {
    // do something
    let string = String::from("Hello world");
    let word = first_word(&string);
    println!("{word}");
}
