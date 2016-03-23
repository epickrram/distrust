extern crate distrust;

use distrust::buffer::*;
use std::thread;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::mem;
use std::ptr;
use std::mem::transmute;
use std::io::Write;

#[test]
fn test_transmuting() {
    let mut data: [u8; 8] = [42, 0, 0, 0, 0, 0, 0, 0];

    let ptr_to_atomic: &mut AtomicIsize = unsafe { mem::transmute(data.get_unchecked_mut(0)) };
    assert_eq!(42, ptr_to_atomic.load(Ordering::Acquire));
    ptr_to_atomic.fetch_add(37, Ordering::SeqCst);
    assert_eq!(42 + 37, ptr_to_atomic.load(Ordering::Acquire));
    data[0] = 17;
    assert_eq!(17, ptr_to_atomic.load(Ordering::Acquire));
}

#[test]
fn should_ensure_that_length_is_a_power_of_two() {
	let size: usize = 3;
	let container = vec![Item {id: 0, payload: 0}; size].into_boxed_slice();
	let mut sequencer = new_single_thread_sequencer();
	let optional_buffer = new_ring_buffer(container, sequencer);
	
	assert!(optional_buffer.is_none())
}

#[ignore]
#[test]
fn should_advance_sequence_number() {
	let (mut sequencer, container) = construct_params();
	let mut buffer = new_ring_buffer(container, sequencer).unwrap();
	
	assert_eq!(buffer.get_next_sequence(), 0);
	assert_eq!(buffer.get_next_sequence(), 1);
}

#[test]
fn should_advance_published_sequence() {
	let (mut sequencer, container) = construct_params();
	let mut buffer = new_ring_buffer(container, sequencer).unwrap();
	let item = Item {id: 7, payload: 11 };
	
	let sequence = buffer.get_next_sequence();
	buffer.publish(sequence, item);
	
	assert_published_sequence(&buffer, 0);
}

#[test]
fn should_be_readable_from_multiple_threads() {
	let (mut sequencer, container) = construct_params();
	
	let mut buffer = new_ring_buffer(container, sequencer).unwrap();
	
	let boxed_1 = Box::new(buffer);
	let raw = Box::into_raw(boxed_1);
	let mut boxed_2 : Box<RingBuffer<Item>>;
	let boxed_3 : Box<RingBuffer<Item>>;
	unsafe {
	boxed_2 = Box::from_raw(raw);
	boxed_3 = Box::from_raw(raw);
	}
	
	
	
	let child = thread::spawn(move || {
	    assert_eq!(0, boxed_2.get_next_sequence());
	    mem::forget(boxed_2);
	});
	
	let res = child.join();
}

struct Foo {
	sequence: AtomicIsize
}

impl Foo {
	fn next_available_sequence(&self) -> isize {
		self.sequence.fetch_add(1, Ordering::Release) + 1
	}
	
	fn current_sequence(&self) -> isize {
		self.sequence.load(Ordering::Acquire)
	}
}

#[test]
fn should_share() {
	let boxed_1 = Box::new(Foo{sequence: AtomicIsize::new(-1)});
	let raw = Box::into_raw(boxed_1);
	let mut boxed_2 : Box<Foo>;
	let mut boxed_3 : Box<Foo>;
	unsafe {
	boxed_2 = Box::from_raw(raw);
	boxed_3 = Box::from_raw(raw);
	}
	
	
	assert_eq!(boxed_2.next_available_sequence(), 0);
	
	
	let child = thread::spawn(move || {
			boxed_3.next_available_sequence();
			mem::forget(boxed_3);
    });
    let res = child.join();
    println!("res: {:?}", res);
    assert_eq!(1, boxed_2.current_sequence());
}


fn assert_published_sequence(buffer: &RingBuffer<Item>, expected_sequence: i64) {
	assert_eq!(expected_sequence, buffer.get_published_sequence());
}

fn construct_params() -> (SingleThreadSequence, Box<[Item]>) {
	let size: usize = 4;
	let container = vec![Item {id: 0, payload: 0}; size].into_boxed_slice();
	let sequencer = new_single_thread_sequencer();
	
	(sequencer, container)
}

struct Item {
	id: i64,
	payload: i64
}

impl Clone for Item {
	fn clone(&self) -> Item {
		Item {id: self.id, payload: self.payload}
	}
}
