
use std::thread;
use crossbeam_channel::unbounded;

use crate::core::batch::PinBatch;
use super::{ Checker, View, SearchParameters };
use super::classic::ClassicChecker;

const NUM_THREADS:usize = 15;

enum ThreadJob {
	Done,
	Work( (View, SearchParameters, PinBatch) ),
}

pub struct ThreadedChecker {
	recieve_rx: crossbeam_channel::Receiver< PinBatch >,
	send_tx:    crossbeam_channel::Sender< ThreadJob >,
	threads:    Vec< std::thread::JoinHandle< () > >
}

impl ThreadedChecker {
	pub fn new() -> ThreadedChecker {
		let (recieve_tx, recieve_rx) = unbounded::< PinBatch >();
		let (send_tx, send_rx)       = unbounded::< ThreadJob >();

		let mut thread_checker = ThreadedChecker {
			recieve_rx,
			send_tx: send_tx.clone(),
			threads: vec![],
		};

		for _thread_num in 0..NUM_THREADS {

			let send_tx = send_tx.clone();
			let tx = recieve_tx.clone();
			let rx = send_rx.clone();

			let handle = thread::spawn(move || {
				let mut checker = ClassicChecker::new();
				let mut done = false;

				while !done {
					let job = rx.recv().unwrap();
					match job {
						ThreadJob::Work( (view, search_param, batch) ) => {
							checker.push_batch(&view, &search_param, batch);
							tx.send(checker.collect_batch()).unwrap();
						}
						ThreadJob::Done => {
							send_tx.send(ThreadJob::Done).unwrap();
							done = true;
						}
					}
				}
			});
			thread_checker.threads.push(handle);
		}

		thread_checker
	}
}



impl Checker for ThreadedChecker {
	fn get_batch_ideal_capacity(&self) -> usize { self.threads.len() + 1 }

	fn push_batch(&mut self, view: &View, search_param: &SearchParameters, batch: PinBatch) {
		self.send_tx.send(ThreadJob::Work((view.clone(), search_param.clone(), batch))).unwrap();
	}

	fn collect_batch(&mut self) -> PinBatch {
		self.recieve_rx.recv().unwrap()
	}

	fn done(&mut self) {
		self.send_tx.send(ThreadJob::Done).unwrap();
		while let Some( handle ) = self.threads.pop() {
			handle.join().unwrap()
		}
	}
}

