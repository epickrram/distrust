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
use std::thread;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::sync::atomic::Ordering;
use std::time::Duration;

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

#[test]
fn test_discouraged_concurrent_access() {

    let h1 = thread::spawn(move || {
        let mut m1 = map_file(&Path::new("/dev/shm/map1.bin"));
        let c1 = create_mapped_atomic(&mut m1, 0);
        assert_eq!(0, c1.load(Ordering::Acquire));
        let mut previous = c1.load(Ordering::Acquire);
        for i in 0..10 {
            thread::sleep(Duration::from_millis(150));
            let current = c1.fetch_add(17, Ordering::AcqRel);
            if previous != current {
                println!("Oh noes! Concurrent modification observed by {:?}! Expected: {}, but was: {}", thread::current(), previous, current);
            }
            previous = current;
        }
    });
    let h2 = thread::spawn(move || {
        let mut m2 = map_file(&Path::new("/dev/shm/map1.bin"));
        let c2 = create_mapped_atomic(&mut m2, 0);
        let mut previous = c2.load(Ordering::Acquire);
        for i in 0..10 {
            thread::sleep(Duration::from_millis(150));
            let current = c2.fetch_add(17, Ordering::AcqRel);
            if previous != current {
                println!("Oh noes! Concurrent modification observed by {:?}! Expected: {}, but was: {}", thread::current(), previous, current);
            }
            previous = current;
        }
    });


    match h1.join() {
        Err(msg) => panic!("Join failed: {:?}", &msg),
        Ok(_) => println!("Join ok")
    }
    match h2.join() {
        Err(msg) => panic!("Join failed: {:?}", &msg),
        Ok(_) => println!("Join ok")
    }
}

fn map_file(path: &Path) -> memmap::Mmap {
    let mut f = File::create(path).unwrap();
    let data: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0];
    match f.write_all(data) {
        Err(eh) => panic!("Failed to write: {}", Error::description(&eh)),
        Ok(_) => println!("Wrote data")
    }

    let file = OpenOptions::new().write(true).read(true).open(path).unwrap();
    println!("Mapping....");
    let mut mem = match memmap::Mmap::open(&file, memmap::Protection::ReadWrite) {
        Err(reason) => panic!("Failed to map file: {}", reason),
        Ok(memFile) => memFile          
    };

    mem
}
