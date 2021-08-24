### Functions

#### Convetions for naming functions and variables
* Rust code uses snake cases which is that all letters are lowecase and underscores seperate words

#### Statements and Expressions
* Rust is expression-based language
* Statements : instructions that perform some action and do not return a value 
	+ Creating a value and assigning a value to it with the let keyword
	+ Function definitions
* Expressions : evaluate to a resulting value
	+ Calling a function
	+ Calling a macro
	+ The block that creates new scopes {}
	+ Expressions do not include ending semicolons
	+ If a semicolon is added to the end of an expression, it is turned into a statement which will not return a value
* Return value 
	+ We don't name return values, but we should mark the type of return value after an arrow (->)
	+ In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
	+ Most functions return last expression implicitly
	+ Explicitly returning value is also allowed with return keyword
