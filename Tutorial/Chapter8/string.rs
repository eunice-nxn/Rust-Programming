fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    //the method also works on a listeral directly
    let s  = "initial contents".to_string();
    let s  = String::from("initial contents");
    
    // Strings are UTF-8 encoded
    let hello = String::from("안녕하세요");
    let hello = String::from("He;;p");
    
    // Appending to a String with push_str and push
    let mut s0 = String::from("foo");
    s0.pust_str("bar");
    println!("s0 is {}", s0);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    // Concatenation with the + Operator or the format! Macro
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // note s1 has been moved here and can no longer be used

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = Stirng::from("toe");

    let s10 = s7 + "-" + &s8 + "-" + &s9;
    let s10 = format!("{}-{}-{}", s7, s8, s9);
    println!("s10 is {}", s10);

    let hello = String::from("Hola");
    for c in "hello".chars() {
        println!("{}", c);
    }

    for b in "hello".bytes() {
        println!("{}", b);
    }






}
