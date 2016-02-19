extern crate distrust;

use distrust::buffer::*;

#[test]
fn should_ensure_that_length_is_a_power_of_two() {
	let size: usize = 3;
	let container = vec![Item {id: 0, payload: 0}; size].into_boxed_slice();
	let mut sequencer = new_single_thread_sequencer();
	let optional_buffer = new_ring_buffer(container, &mut sequencer);
	
	assert!(optional_buffer.is_none())
}

#[test]
fn should_advance_sequence_number() {
	let (mut sequencer, container) = construct_params();
	let mut buffer = new_ring_buffer(container, &mut sequencer).unwrap();
	
	assert_eq!(buffer.get_next_sequence(), 0);
	assert_eq!(buffer.get_next_sequence(), 1);
}

#[test]
fn should_advance_published_sequence() {
	let (mut sequencer, container) = construct_params();
	let mut buffer = new_ring_buffer(container, &mut sequencer).unwrap();
	let item = Item {id: 7, payload: 11 };
	
	let sequence = buffer.get_next_sequence();
	buffer.publish(sequence, item);
	
	assert_published_sequence(&buffer, 0);
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