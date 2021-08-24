## 3.1 Variables and Mutability

### Variables and Mutability
* Variables are immutable by default
* By adding 'mut' in front of the variable name, Variables can be mutable
* mutable
	+ allows the value of this variable to change
	+ conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable's value

### Differences Between Variables and Constants

#### Constants
* It is not allowed to use 'mut' with constants
* Declare constants using the 'const' keyworld instead of the let keyword
* The type of value must be annotated in declaring constants
* Rust naming convention of constants : use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability

### Shadowing
* We can shadow a variable by using the same variable's name and repeating the use of the 'let' keyword
* It is different from marking a variables as mut
* By using 'let', we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed
* The other difference between mut and shadowing is that because we're effectively creating a new variable when we use the 'let' keyword again
* If we try to use 'mut' for shadowing, we'll get compile error because shadowing is not allowed to mutate a variable's type

## 3.2 Data Types

### Rust
* Rust is a statically typed language, which means that it must know the types of all variables at compile time
* The compiler can usually infer what type we want to use based on the value and how we use it

### Scalar Types
* A scalar type represents a single value
* integers, floating-point numbers, Booleans and characters

#### Integer Types
* integer : a number without a fractional component
* signed integer types start with i, unsigned interger types start with u

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

#### Floating-Point Types
* Rust has two primitive types for floating-point numbers which are numbers with decimal points
* Rust's floating point types are f32 and f64
*  The default type is f64
* The f32 type is a single-precision float, and f64 has double precision

#### Numeric Operations
* Rust supports addition, subtraction, multiplication, division and remainder

#### The Boolean Type
* In Rust, two possible types : true, false

#### The Character Type
* Rust's char type is the language's most primitive alphabetic type
* char type is 4 bytes in size and represents a Unicode Scalar Value

#### Compound Types
* Compound types can group multiple values into one type.
* Rust has two primitive compound types: tuples and arrays
* The Tuple Type
	+ A tuple : a general way for grouping a number of values with a variety of types into one compound type.
	+ A fixed length : Once declared, they cannot grow or shrink in size

* The Array Type
	+ Array : Every element of an array must have the same type
	+ Arrays in Rust are different from arrays in some other languages because in Rust have a fixed length, like tuples
	+ By using square brackets, we can mark the type of each element, a semicolon, and the number of elements in the array
