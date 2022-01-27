# A Timer Future
* Mutex around a boolean
* Spins up a new thread that sleeps for some amount of time
* When the thread wakes up, it sets the boolean to true and 'wakes up' the future
* Calls to poll check the boolean to see if we're done

## Four rules for using async/await
	
```rust
async fn foo(s:String) -> i32 {
	// ...
}

fn foo(s:String) -> impl Future<Output=i32>{
	// ...
}
```

1. If you have a Future<Output=i32> and you want an i32, use .await on it
2. You can only .await inside of an async fn or block
3. To start executing a Future, you pass it to an executor
4. Futures need to have poll() called over and over until a value is produced
		
```rust
let mut gen = || {
	let xs = vec![1, 2, 3];
	let mut sum = 0;
	for x in xs {
		sum += x;
		yield sum;
	}
}
```
* When we invoked gen for the first time, it would be basically run up to the yield and return at first value. 
* If we invoke the generator again, we would get second value for second iteration of gen's loop after continue processing execution

```rust
let xs = vec![1, 2, 3];
let mut gen = || {
	let mut sum = 0;
	for x in xs.iter() { // iter0
		sum += x;
		yield sum; // Suspended0
	}
	for x in cs.iter().rev() { // iter1
		sum -= x;
		yield sum; // Suspended1
	}
};
```

```rust
enum SumGenerator {
	Unresumed {
		xs: Vec<i32>,
	},
	Suspended0 {
		xs: Vec<i32>,
		iter0: Iter<'self, i32>,
		sum: i32,
	},
	Suspended1 {
		xs: Vec<i32>,
		iter1: Iter<'self, i32>,
		sum: i32,
	}
	Returned,
}
```
* Generator let you call yield over and over to get values
* async/await is a simpler syntax for a generator that implements the Future trait	
## Tasks, Executors, & Reactors
* Task : a unit of work to execute, a chain of Futures
* Executor : schedules tasks
* Reactor : notifies the executor that tasks are ready to execute
```rust
pub trait Future {
	type Output;
	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self:Output>;
```
* Executor calls poll, and provides a context
	+ Executor's job is that calling repeatedly poll and it make sure the futures are doing the work supposed to do.
* Context : interface to the reactor containg waker 

* The process of future
1. async function
```rust
async fn foo() {
	// ...
}
```
2. spawn
```rust
spawner.spawn(foo())
```
3. Executor task queue
4. Calls poll() on the Future
5. When the future returns ready state, it calls wake() (reactor)

## A quick aside about Pin\<P\>
* Before a future starts executing, we need to be able to move it around in memory. (For example, to create a task out of it, we need to it to the heap)
* Once a future starts executing, it must not move in memory.
(otherwise, borrows in the body of the future would become invalid)
* When we turn some sort of pointer type into a Pin\<P\>, we're promising what the pointer to will no longer move.
* Box\<T\> turns into Pin\<Box\<T\>\>
* There's an extra trait, "Unpin", that says "I don't care about this", similar to how Copy says "I don't care about move semantics".
