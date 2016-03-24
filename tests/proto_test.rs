extern crate distrust;
extern crate memmap;

use distrust::buffer::*;
use distrust::proto::*;

use std::fs::File;
use std::fs::OpenOptions;
/*
use std::thread;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::mem;
use std::ptr;
use std::mem::transmute;
//use std::io::Write;
use std::error::Error;
*/
use std::error::Error;
use std::io::prelude::*;
use std::sync::atomic::Ordering;
use memmap::{Mmap, Protection};

#[test]
fn test_counter() {
    let mut f = File::create("/dev/shm/foo.txt").unwrap();
    let data: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0];
    match f.write_all(data) {
        Err(eh) => panic!("Failed to write: {}", Error::description(&eh)),
        Ok(_) => println!("Wrote data")
    }

    let file = OpenOptions::new().write(true).read(true).open("/dev/shm/foo.txt").unwrap();
    println!("Mapping....");
    let mut mem = match memmap::Mmap::open(&file, memmap::Protection::ReadWrite) {
        Err(reason) => panic!("Failed to map file: {}", reason),
        Ok(memFile) => memFile          
    };

    let counter: MappedAtomicIsize = create_mapped_atomic(&mut mem, 0);
    println!("Mapped!");
    
    assert_eq!(0, counter.load(Ordering::Acquire));
    assert_eq!(0, counter.fetch_add(5, Ordering::AcqRel));

    assert_eq!(5, counter.load(Ordering::Acquire));

}


