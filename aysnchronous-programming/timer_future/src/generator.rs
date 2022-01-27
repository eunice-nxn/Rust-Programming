#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;


fn main(){

		let xs = vec![1, 2, 3];
		let mut gen = || {
			let mut sum = 0;
			for x in xs.iter() { // iter0
				sum += x;
				yield sum;
			}
			for x in cs.iter().rev() { // iter1
				sum -= x;
				yield sum;
			}
		};
}
