fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a = String::new();
    while a.len() < 100 {
        a.push_str("a");
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
