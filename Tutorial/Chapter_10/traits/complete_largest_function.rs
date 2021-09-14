/*
	To call this code with only those types that implements the Copy trait, 
	we can add to the trait bounds of T! 
	This code shows the complete code of a generic largest function 
	that will compile as long as the types of the values in the slice 
	that we pass into the function implement the PartialOrd and Copy traits, 
	like i32 and char do
*/
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list {
		if item > largest {
			largest = item;
		}
	}

	largest
}

fn main() {
	let number_list = vec![34, 50, 24, 100, 65];

	let result = largest(&number_list);
	println!("The largest number is {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];

	let result = largest(&char_list);
	println!("The largest char is {}", result);
}

