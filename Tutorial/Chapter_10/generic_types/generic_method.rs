#![allow(unused)]
struct Point<T> {
	x: T,
	y: T,
}

/* 
   By declaring T as a generic type after impl, 
   Rust can identify that the type in the angle brackets 
   is a generic type rather than a concrete type.
 */
impl<T> Point<T> {
	fn x(&self) -> &T {
			&self.x
	}
}


/*
   Point<f32> will have a method named distance_from_origin
   and other instances of Point<T> where T is not of type f32
   will not have this method defined
*/

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}
fn main() {
	let p = Point { x: 5, y: 10};
	
	println!("p.x = {}", p.x());
}
