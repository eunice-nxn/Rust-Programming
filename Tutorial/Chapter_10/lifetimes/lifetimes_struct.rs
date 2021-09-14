#![allow(unused)]
struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
		fn level(&self) -> i32 {
				  3
		}
		// The example where the third lifetime elision rule applies
		fn announce_and_return_part(&self, announcement: &str) -> &str {
				println!("Attention please: {}", announcement);
				self.part
		  }
		  // the return type gets the lifetime of &self, and all lifetimes have been accounted for
}

fn main() {
	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let i = ImportantExcerpt{
		part: first_sentence,
	};
}
