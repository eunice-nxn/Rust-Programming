### Ownership
* In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks at compile time
#### Ownership Rules
* Each value in Rust has a variable that's called its owner
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped 

#### Variable Scope
* When a variable comes into scope ( is declared ), it is valid
* It remains valid until it goes out of scope

#### Ways variables and data interact
* Move
	'''
	let s1 = String::from("hello");
	let s2 = s1;
	println!("{}, world!", s1); // This won't work
	'''
	Because Rust considers s1 to no longer to be valid for memory safety and preventing double free error 
* Clone
	'''
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("s1 = {}, s2 = {}", s1, s2);
	'''

* Copy ( Stack-Only Data )
	+ All the integer types, such as u32
	+ The boolean type, bool with values true and false
	+ All the floating types, such as f64
	+ The character type, char
	+ Tuples, if they only contain types that also implement copy ( For example, (i32, i32) implements Copy, but (i32, String) does not

#### References and Borrowing
* At any given time, either one mutable reference or any number of immutable references can be available
* References must always be valid

#### The Slice Type
* slice : another data type that does not have ownership
* string slice : reference to part of a String
* It can be available that creating slices using a range within brackets by specifying [starting\_index..ending\_index]
	+ starting\_index : first position in the slice
	+ ending\_index : one more than the last position in the slice
* string literals are slices of string slices already

#### Summary
* The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time

