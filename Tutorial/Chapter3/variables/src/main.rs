use std::io;

fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS : {}", MAX_POINTS);
    let a = 5;
    let a = a + 1;
    let a = a * 2;

    println!("The value of a is {}", a);

    let b = 2.0;  //f64
    let c: f32 = 3.0;  //f32

    println!("The value of b is {}", b);
    println!("The value of c is {}", c);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("sum : {} difference : {} product : {} quotient : {} remainder : {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!(" t : {}, f : {}" , t, f);

    let c = 'z';
    let z = 'Z';

    println!(" c: {}, z : {}", c, z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let ( first_tup, second_tup , third_tup  ) = tup;
    println!("fist_tup {} second_tup {} third_tup {}", first_tup, second_tup, third_tup);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundered {}, six_point_four {} one {}", five_hundred, six_point_four, one);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!(
        "The value of the element at index {} is : {}",
        index, element
        );

}
