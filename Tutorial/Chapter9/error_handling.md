### Error Handling

#### Two kinds of error in Rust
* Recoverable error 
	+ It's resonable to the report problem to the user and retry operation
	+ EX) file not found error
	+ Result\<T, E\>
* Unrecoverable error
	+ Symptoms of bugs
	+ EX) trying to access a location beyond the end of array
	+ panic! macro

#### Unrecoverable errors with panic!
#### Recoverable errors with Result \<T, E\>


		enum Result<T, E> {
			Ok(T),
			Err(E),
		}

* T : The type of the value that will be returned in a success case
* E : The type of the error that will be returned in a failure case within the Err variant



