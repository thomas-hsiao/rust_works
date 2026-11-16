
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let wd = first_word(&s).to_string(); // make a copy to heap
    s.clear();

    println!("oh, {}", wd);
    
    let hello = &s[0..5]; // error:out of bound, because s is cleared.
    let word = &s[6..11];
    println!("hey, s: {}, h: {}, w: {}", s, hello, word);
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
