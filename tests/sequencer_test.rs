extern crate distrust;

use distrust::buffer::*;

#[test]
fn should_advance_single_thread_sequence() {
	let mut sequence = new_single_thread_sequencer();
	
	assert_eq!(0, sequence.next_sequence());
	assert_eq!(1, sequence.next_sequence());
	assert_eq!(2, sequence.next_sequence());
}