#![allow(unused)]

fn main() {

    /*
     * This enum is Option<T>, and is defined by the standart library
        enum Option<T> {
            Some(T),
            None,
        }
    */
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
