
use std::thread;
use crossbeam_channel::unbounded;

use crate::core::batch::PinBatch;
use super::{ Checker, View, Args, TraceInitializer };
// use super::classic::ClassicChecker;
use super::vectorized::VectorizedChecker;

enum ThreadJob {
	Done,
	Work( (View, Args, PinBatch, TraceInitializer) ),
}

pub struct ThreadedChecker {
	recieve_rx: crossbeam_channel::Receiver< PinBatch >,
	send_tx:    crossbeam_channel::Sender< ThreadJob >,
	threads:    Vec< std::thread::JoinHandle< () > >
}

impl ThreadedChecker {
	pub fn new(args: Args) -> ThreadedChecker {
		let (recieve_tx, recieve_rx) = unbounded::< PinBatch >();
		let (send_tx, send_rx)       = unbounded::< ThreadJob >();

		let mut thread_checker = ThreadedChecker {
			recieve_rx,
			send_tx: send_tx.clone(),
			threads: vec![],
		};

		let num_threads = if args.thread_num > 0 { args.thread_num } else { num_cpus::get() };

		for _thread_id in 0..num_threads {

			let send_tx = send_tx.clone();
			let tx = recieve_tx.clone();
			let rx = send_rx.clone();

			let handle = thread::spawn(move || {
				// let mut checker = ClassicChecker::new();
				let mut checker = VectorizedChecker::new();
				let mut done = false;

				while !done {
					let job = rx.recv().unwrap();
					match job {
						ThreadJob::Work( (view, args, batch, trace_init) ) => {
							checker.push_batch(&view, &args, batch, trace_init);
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

	fn push_batch(&mut self, view: &View, args: &Args, batch: PinBatch, trace_init: TraceInitializer) {
		self.send_tx.send(ThreadJob::Work((view.clone(), args.clone(), batch, trace_init))).unwrap();
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

