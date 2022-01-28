use {
	futures::{
		prelude::*,
		future::{BoxFuture, FutureExt},
		task::{waker_ref, ArcWake},
		executor::block_on,
	},
	futures_timer::Delay,
	std::{
		future::Future,
		sync::mpsc::{sync_channel, Receiver, SyncSender},
		sync::{Arc, Mutex},
		task::{Context, Poll, Waker},
		time::Duration,
		pin::Pin,
		thread,
	}
};
// 2. timer future
pub struct TimerFuture{
	shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
	// Whether or not the sleep time has elapsed
	completed: bool,
	// the "waker" to wake up the future
	waker: Option<Waker>,
}

impl Future for TimerFuture {
	type Output = ();
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let mut shared_state = self.shared_state.lock().unwrap();
		if shared_state.completed {
			Poll::Ready(())
		} else {
			shared_state.waker = Some(cx.waker().clone());
			Poll::Pending
		}
	}
}

impl TimerFuture {
	pub fn new(duration: Duration) -> Self {
		let shared_state = Arc::new(Mutex::new(SharedState {
			completed: false,
			waker: None,
		}));

		let thread_shared_state = shared_state.clone();
		thread::spawn(move || {
			thread::sleep(duration);
			let mut shared_state = thread_shared_state.lock().unwrap();
			shared_state.completed = true;

			if let Some(waker) = shared_state.waker.take() {
				waker.wake()
			}
		});

		TimerFuture { shared_state }
	}
}

// Task executor that receives tasks off of a channel and runs them.
struct Executor {
	ready_queue: Receiver<Arc<Task>>,
}

// A future that can reschedule itself to be polled by an 'Executor'.
struct Task {
	// In-progess future that should be pushed to complete
	future: Mutex<Option<BoxFuture<'static, ()>>>,
	
	// Handle tot place the task itself back onto the task queue
	task_sender: SyncSender<Arc<Task>>,
}

// 'Spawner' spawns new futures onto the task channel
#[derive(Clone)]
struct Spawner {
	task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner){
	//Maximum number of tasks to allow queueing in the channel at once.
	const MAX_QUEUED_TASKS: usize = 10_000;
	let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
	(Executor { ready_queue }, Spawner{ task_sender })
}

impl Spawner {
	fn spawn(&self, future: impl Future<Output = ()> + 'static + Send){
		let future = future.boxed();
		let task = Arc::new(Task {
			future: Mutex::new(Some(future)),
			task_sender: self.task_sender.clone(),
		});
		self.task_sender.send(task).expect("too many tasks queued");
	}
}

impl ArcWake for Task {
	fn wake_by_ref(arc_self: &Arc<Self>){
		let cloned = arc_self.clone();
		arc_self.task_sender.send(cloned).expect("too many tasks queued");
	}
}

impl Executor {
	fn run(&self){
		while let Ok(task) = self.ready_queue.recv(){
			// Takes the future, and if it has not yet completed (is still Some), 
			// poll it in an attempt to complete it
			let mut future_slot = task.future.lock().unwrap();
			if let Some(mut future) = future_slot.take() {
				// Create a 'LocalWaker' from the task itself
				let waker = waker_ref(&task);
				let context = &mut Context::from_waker(&*waker);

				if let Poll::Pending = future.as_mut().poll(context) {
					// We're not done processing the future, so put it
					// back in its task to be run again the future.
					*future_slot = Some(future);
				}
			}
		}
	}
}

// 1. sleep future
async fn i_sleep() {
	Delay::new(Duration::from_secs(5)).await;
}


async fn how_long(){
	let x = i_sleep();
	let y = i_sleep();
	future::join(x, y).await;
}

fn main(){
	// 1. sleep future
	let future = how_long();
	block_on(future);
	

	// 2. timer_future	
	let (executor, spawner) = new_executor_and_spawner();

	//Spawn a task to pring before and after waiting on a timer
	spawner.spawn(async {
			println!("howdy!");

			//Wait for our timer future to complete after two seconds
			TimerFuture::new(Duration::new(2, 0)).await;
			println!("done!");
	});

	//Drop the spawner so that our executor knows it is finished and won't 
	//receive more incoming tasks to run
	drop(spawner);

	//Run the executor until the task queue is empty
	//This will print "howdy!", pause, and then print "done!".
	executor.run();
}

