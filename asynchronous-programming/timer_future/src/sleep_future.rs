use std::time::Duration;
use futures_timer::Delay;
use futures::prelude::*;
use futures::executor::block_on;
async fn i_sleep() {
	Delay::new(Duration::from_secs(5)).await;
}


async fn how_long(){
	let x = i_sleep();
	let y = i_sleep();
	future::join(x, y).await;
}

fn main(){
	let future = how_long();
	block_on(future);
}
