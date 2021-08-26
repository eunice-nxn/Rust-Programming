#### Struct Method Syntax
* Method : It is defined within the context of a struct and is declared with the `fn` keyword and their name It can have parameters and a return value
* Method's first parameter should be always `self` which represents the instance of the struct the method is being called on
	+ `&self` : only reading the data in the struct, not write to it
	+ `&mut self` : changing the instance by writing to the instance
	+ `self` : taking ownership of the instance
		   It is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation

#### Associated functions
* Within `impl` blocks, a function which doesn't take `self` as a parameter
* Not method, they're functions which just are associated with struct
* They don't have an instance of the struct to work with
* Associated functions are often used for constructors which will return a new instance of the struct 
* Associated functions is namespaced by the struct
	+ `Rectangle::square(3)`
	+ `::` syntax is used for both associated functions and namespaces created by modules

#### Multiple impl Blocks
* Each struct is allowed to have multiple `impl` blocks
