#### Module System
* Packages : A Cargo feature that lets you build, test, and share crates
* Crates : A tree of modules that produces a library or executable
* Modules and use : Let you control the organization, scope, and privacy of paths
* Paths : A way of naming an item, such as struct, function, module

#### Packages and Crates
* Crate : a binary or library
	+ crate root : a source file that Rust compiler starts from and makes up the root module of one's crate 
* Package : contains a cargo.toml file that describes how to build these crates
	+ A package must contain zero or one library crates, and no more
	+ It can contain many binary crates as we'd like but it must contain at least one crate (either library or binary)
* Cargo 
	+ src/main.rs : a crate root of a binary crate with the same name as the package
		- A package can have multiple binary crates by placing files in the src/bin directory
		- each file will be a seperate binary crate
	+ src/lib.rs : a crate root of a library crate with the same name as the package
	+ Cargo passes the crate root files to rustc to build the library or binary

#### Defining modules to control scope and privacy
* module tree : crate's module structure at root of crate

#### Paths for Referring to an Item in the Module Tree
* If we want to call a function, we need to know its path
* two forms
	+ absolute path : It starts from a crate root by using a crate name or a literal crate
	+ relative path : It starts from the current module and uses self, super, or and identifier in the current module
