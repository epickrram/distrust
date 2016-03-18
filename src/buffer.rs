use std::sync::atomic::{Ordering, fence};
use std::mem;
use std::thread;

pub fn new_ring_buffer<T>(container: Box<[T]>, sequencer: SingleThreadSequence) -> Option<RingBuffer<T>> {
	let buffer_length = container.len() as i64;
	
	if buffer_length.count_ones() != 1 {
		return None;
	}
    
	
	Some(RingBuffer {
        values: container,
        sequencer: sequencer,
        length: buffer_length,
        mask: buffer_length - 1,
        published_sequence: -1
	})
}

pub fn configure_event_processing<T>(container: Box<[T]>, sequencer: SingleThreadSequence, event_processors: &[EventProcessor<T>]) -> Option<Box<RingBuffer<T>>> {
	let rb_option = new_ring_buffer(container, sequencer);
	if rb_option.is_none() {
		return None;
	}
	
	let mut buffer = rb_option.unwrap();
	let boxed_1 = Box::new(buffer);
	let raw = Box::into_raw(boxed_1);
	let mut boxed_2 : Box<RingBuffer<T>>;
	let boxed_3 : Box<RingBuffer<T>>;
	unsafe {
	boxed_2 = Box::from_raw(raw);
	boxed_3 = Box::from_raw(raw);
	}
	
	
	/*
	let child = thread::spawn(move || {
	    assert_eq!(0, boxed_2.get_next_sequence());
	    mem::forget(boxed_2);
	});
    */

	Some(boxed_3)
}


pub struct RingBuffer<T> {
    values: Box<[T]>,
    sequencer: SingleThreadSequence,
    length: i64,
    mask: i64,
    published_sequence: i64
}

impl<T> RingBuffer<T> {
	pub fn get_next_sequence(&mut self) -> i64 {
//		let mut seq = self.sequencer;
//		unsafe {
//			seq.next_sequence()
//		}
		0
	}
	
	pub fn publish(&mut self, sequence: i64, item: T) {
		let offset = sequence & self.mask;
		self.values[offset as usize] = item;
		self.published_sequence = sequence;
		fence(Ordering::Release);
	}
}

pub trait EventProcessor<T> {
	fn on_thread_start(&self);
	fn on_shutdown(&self);
	fn on_event(&self, sequence: i64, event: T, end_of_batch: bool);
}

pub trait ReadableRingBuffer<T> {
	fn get_published_sequence(&self) -> i64;
}

impl<T> ReadableRingBuffer<T> for RingBuffer<T> {
	fn get_published_sequence(&self) -> i64 {
		fence(Ordering::Acquire);
		self.published_sequence
	}
}

pub fn new_single_thread_sequencer() -> SingleThreadSequence {
	SingleThreadSequence {next_available_sequence: 0, published_sequence: -1}
}

pub trait Sequencer {
	fn next_sequence(&mut self) -> i64;
	fn publish_sequence(&mut self, sequence: i64);
	fn published_sequence(self) -> i64;
}

pub struct SingleThreadSequence {
	next_available_sequence: i64,
	published_sequence: i64
}

impl Sequencer for SingleThreadSequence {
	fn next_sequence(&mut self) -> i64 {
		let next_sequence = self.next_available_sequence;
		self.next_available_sequence += 1;
		next_sequence
	}
	
	fn publish_sequence(&mut self, sequence: i64) {
		self.published_sequence = sequence;
		fence(Ordering::Release);
	}
	
	fn published_sequence(self) -> i64 {
		fence(Ordering::Acquire);
		self.published_sequence
	}
}
