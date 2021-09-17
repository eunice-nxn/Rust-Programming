### Test

#### How to Write Test
* Test : Rust functions that verify the non-test code is funcitoning in the expected manner
* The actions of body for test. 
	1. Set up any needed data or state
	2. Run the code you want to test
	3. Assertt the results are what you expect

##### The Anatomy of a Test function
* A test in Rust : a function which is annotated with the ```ttest``` attribute
* Attribute 
	+ metadata about pieces of Rust code
	+ Ex) ```derive``` : we used this with structs

* To change a function into a test funciton, add ```#[test]``` on the line before ```fn```
* If we run our test with the ```cargo test``` command, Rust builds a test runner binary that runs the funcitons annotated with the ```test``` attribute and reports on whether each test function passes or fails.

#### Test Orgranization
* Unit Tests : Testing one module in isolation at a time and It can test private interfaces
* Integration Tests : Using only the public interface and potentially exercising multiple modules per test.

##### Unit Tests
* The purpose : Testing each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected. 
* The convention : Creating a module named tests in each file to contain the test functions and to annotate the module with ```cfg(test)```.

* The ```#[cfg[test]]``` annotation 
    + It tells Rust to compile and run the test code only when you run ```cargo test```,
      not when you run ```cargo build```.
    + This saves compile time when we only want to build the library and saves space 
    in the resulting compiled artifact because the tests are not included.
    + cfg stands for configuration.
 
##### Integration Tests
* In Rust, integration tests are entirely external to our library.
* Units of code that work correctly on their own could have problems when integrated, 
so test coverage of the integrated code is important as well.


#### Summary
* Rust's testing features provide a way to specify how code should function to ensure it continues
to work as we expect, even as we make changes
* Unit tests exercise different parts of a library separately and can test private implementation details
* Integration tests check that many parts of the library work together correctly, 
and they use the library's public API to test the code in the same way external code will use it.

