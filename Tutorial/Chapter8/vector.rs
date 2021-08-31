#![allow(unused)]
fn main() {
    // Creating a new vector
    let v: Vec<i32> = Vec::new();
    // Creating a vector using macro vec!
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v2 = Vec::new();
    v2.push(5); // update start
    v2.push(6);
    v2.push(7);
    v2.push(8); // update end

    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
  
    // let does_not_exist = &v3[100]; 
    // -> cause panic of program because it references a nonexistent element
    // let does_not_exist = v3.get(100);
    // -> return None without panicking

    let first = &v3[0];

    v.push(6); // In this point, the reference to the first element (line 29) would be pointing to deallocated memory
    println!("The first element is : {}", first);

    // Iterating over the values in a vecter
    // over immutable references 
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }
    // over mutable references
    let mut v5 = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];



} // v, v2, v3 goes out of scope and is freed here
