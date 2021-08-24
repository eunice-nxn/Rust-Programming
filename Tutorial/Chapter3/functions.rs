fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("x is: {} y is: {}", x, y);

    let five = five(5);
    println!("The value of five is : {}", five);

    let _lucky_number = 7; // I'm feeling lucky today

}

fn another_function(x: i32, y: i32) {
    println!("Another function");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five(x: i32) -> i32 {
    x + 1
}
