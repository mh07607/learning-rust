
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let string = String::from("Hello world!");
    let word = first_word(&string);
    println!("{}", &string[0..word]);
}

