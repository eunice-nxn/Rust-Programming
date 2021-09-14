### Generic Data Types
* Rust's type naming convetion : CamelCase

#### Performance of Code Using Generics
* Rust implements generics in such a way that a code doesn't run any slower using generic types than it would be with concrete types

* Rust accomplishes this by performing monomorphization of the code that is using generics at compile time

* Monomorphization : a process of turning generic code into specific code by filling in the concrete typs that are used when compiled

* Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cose for using generics

* The process of monomorphization makes Rust's generics extremely efficient at runtime

#### Traits: Defining Shared Behavior
* Trait : It tells the Rust compiler about functionality a particular type has and can share with other types
	+ We can use this to define shared behavior in an abstract way

* Trait Bound Syntax
	
	```rust
	pub fn notify(item: &impl Summary) {
		println!("Breaking news! {}", item.summarize());
	}

	pub fn notify<T: Summary>(item: &T) {
		println!("Breaking news! {}", item.summarize());
	}
	```

	```rust
	pub fn notify(item1: &impl Summary, item2: &impl Summary) {
	```
	
	```rust
	pub fn notify<T: Summary>(item1: &T, item2: &T) {
	```
	
* Specifying Multiple Trait Bounds with the + Syntax
	
	```rust
	pub fn notify(item: &(impl Summary + Display)) {
	```

	```rust
	pub fn notify<T: Summary + Display>(item: &T) {
	```

* Clearer Trait Bounds with where Clauses

	+ Using too many trait bounds has its downsides
	+ Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of t. it bound information between the function's name and its parameter list, making the function signature hard to read
	+ For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature
	```rust
	fn some_function<T:Display + Clone + Debug>(t: &T, u: &U) -> i32 {
	```

	```rust
	fn some_function<T, U>(t: &T, u: &T) -> i32
		where T: Display + Clone,
	{
	```
* Using Trait Bounds to Conditionally Implement Methods
	+ blanket implementations : Impelmentations of a trait on any type that satisfies the trait bounds
		- This is extencively used in the Rust standard library
		- Example : ToString 
		```rust
		impl<T: Display> ToString for T {
			// --snip--
		}
		```
		The standard library has this balnket impelementation, 
		we can call the `to_string` method defined by the `ToString` trait on any ttype 
		that implements the `Display` trait.
		For example, we can turn integers into 
		their corresponding `String` values like this 
		because integers impelment `Display`.
		```rust
		let s  = 3.to_string();
		```

#### Generics and Traits
* Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior

#### Lifetimes
* Lifetimes eusure that references are valid as long as we need them to be
* Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid

* The main aim of lifetimes : to prevent dangling references which causes a program to reference data other than the data it's intended to reference

* borrow checker : compares scopes to determine whether all borrows are valid

* Lifetime Annotation Syntax
	+ functions can accept any type when the signature specifies 
	a generic type parameter, functions can accept references 
	with any lifetime by specifying a generic lifetime parameter.
	+ Lifetime annotations describe the relationships of the lifetimes
	of multiple references to each other without affecting the lifetimes.
	+ Lifetime annotation syntax 
		- a name of lifetime parameter must start with an apostrophe(')
		and are usuually all lowercase and very short like generic types
		- We place lifetime parameter annotations after the & of a reference, using a space to seperate the annotation from the reference's type
		- Example 
		```rust
		&i32	// a reference
		&'a i32 // a reference with an explicit lifetime
		&'a mut i32 // a mutable reference with an explicit lifetime
		```

#### Lifetime Elision Rules
* The pattern programmed into Rust's analysis of references
* A set of particular cases that the compiler will consider
* The elision rules don't provide full inference. 
Because there is stil ambiguity as to what lifetimes the references have, 
the compiler won't guess what the lifetime of the remaining references should be
* Instead of guessing, the compiler will give you an error 
that you can resolve by adding the lifetime annotations 
that specify how the references relate to each other 
* input lifetimes : Lifetimes on function or method parameters
	+ The first rule is applied to this
* output lifetimes : Lifetimes on return values
	+ The second rule and the third rule are applied to this
* The compiler uses three rules to figure out what lifetimes references have when there arn't explicit annotations

* The first rule : Each parameter that is a reference gets its own lifetime parameter
	+ a function with one parameter gets one lifetime parameter
	```rust
	fn foo<'a'>(x: &'a i32);
	```
	+ a function with two parameters gets two seperate lifetime parameters
	```rust
	fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
	```
* The second rule : If there is exactly ont input lifetime parameter, that lifetime is assigned to all output parameters	+ 
	```rust
	fn foo<'a>(x: &'a i32) -> &'a i32
	```
* The third rule : If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters
	+ This rule makes methods much nicer to read and write because fewer symbols are necessary

#### The Static Lifetime
* static lifetime: 'static 
	+ this reference can live for the entire duration of the program
	+ All string literals have the ```'static```lifetime
		```rust 
		let s: &'static str = "I have a static lifetime.";
		```

