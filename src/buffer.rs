use std::sync::atomic::{Ordering, fence};

pub fn new_ring_buffer<'a, T>(container: Box<[T]>, sequencer: &'a mut Sequencer) -> Option<RingBuffer<'a, T>> {
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

pub struct RingBuffer<'a, T> {
    values: Box<[T]>,
    sequencer: &'a mut Sequencer,
    length: i64,
    mask: i64,
    published_sequence: i64
}

impl<'a, T> RingBuffer<'a, T> {
	pub fn get_next_sequence(&mut self) -> i64 {
		self.sequencer.next_sequence()
	}
	
	pub fn publish(&mut self, sequence: i64, item: T) {
		let offset = sequence & self.mask;
		self.values[offset as usize] = item;
		self.published_sequence = sequence;
		fence(Ordering::Release);
	}
	
	pub fn get_published_sequence(&self) -> i64 {
		fence(Ordering::Acquire);
		self.published_sequence
	}
}

pub fn new_single_thread_sequencer() -> SingleThreadSequence {
	SingleThreadSequence {next_available_sequence: 0}
}

pub trait Sequencer {
	fn next_sequence(&mut self) -> i64;
}

pub struct SingleThreadSequence {
	next_available_sequence: i64
}

impl Sequencer for SingleThreadSequence {
	fn next_sequence(&mut self) -> i64 {
		let next_sequence = self.next_available_sequence;
		self.next_available_sequence += 1;
		next_sequence
	}
}