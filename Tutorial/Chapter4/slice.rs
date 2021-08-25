fn main() {


    let s = String::from("hello");
    let fw = first_word(&s);
    println!("fw : {} ", fw);

    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("hello : {}", hello);

    let world = &s[6..11];
    println!("world : {}", world);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("slice : {}", slice);
    let slice = &s[..2];
    println!("slice : {}", slice);

    let s = String::from("hello");
    let len = s.len();

    let slice = &s[3..len];
    println!("slice : {} len : {}", slice, len);
    let slice = &s[3..];
    println!("slice : {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("slice : {} len : {}", slice, len);
    let slice = &s[..];
    println!("slice: {}", slice);


    let my_string = String::from("hello world");
    
    // first_word works on slices of 'String's
    let word = first_word_general(&my_string[..]);
    println!("word : {}", word);

    //first_word works on slices of string literals
    let my_string_literal = "hello world";

    //Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_general(&my_string_literal[..]);
    println!("word_literal : {} ", word);
}

fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();

    for( i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
    
fn first_word_general(s: &str) -> &str { // defining a function to take a string slice instead of a reference to a String

    let bytes = s.as_bytes();

    for( i, &item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}



