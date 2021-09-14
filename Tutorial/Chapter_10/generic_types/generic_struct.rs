#![allow(unused)]
struct Point1<T> {
	x: T,
	y: T,
}

struct Point2<T, U> {
	x: T,
	y: U,
}

fn main() {
		let integer = Point1 { x: 5, y: 10};
		let float = Point1 { x: 1.0, y: 4.0 };

		let both_integer = Point2 { x: 5, y: 10};
		let both_float = Point2 { x: 1.0, y: 4.0 };
		let integer_and_float = Point2 { x: 5, y: 4.0};
}
