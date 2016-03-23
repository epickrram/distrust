extern crate memmap;

use std::sync::atomic::AtomicIsize;
use std::fs::File;
use std::ptr::read;
use std::ptr::write;
use std::mem::size_of;
use std::mem::transmute;
use memmap::{Mmap, Protection};

const HEADER_BYTES: usize = 8;

pub trait Serialisable {
    fn transmute<'a>(&self) -> &'a[u8];
}

pub struct Buffer {
    capacity: usize,
    mem: memmap::Mmap
}

impl Buffer {
    pub fn sequence() {
        let s = AtomicIsize::new(42);
        let data: [u8; 8] = [17, 42, 99, 37, 0, 0, 0, 9];
        let num: isize = unsafe { transmute(data) };
        let i_bytes: isize = unsafe { transmute(s) };
        println!("{:?}", i_bytes);
    }
    pub fn create<T>(capacity: usize, file: &File) -> Self {
        let size_of_record = size_of::<T>();
        let mem = memmap::Mmap::open(file, memmap::Protection::ReadWrite).unwrap();
        Buffer {
            capacity: capacity,
            mem: mem
        }
    }

    pub fn offer<T: Serialisable>(&mut self, item: T) -> bool {
        let data = unsafe { self.mem.as_mut_slice() };
        let array: &[u8] = item.transmute();
        // TODO -> transmute from &[u8] to usize?
        for i in 0..array.len() {
            data[i] = array[i];
        }
        true 
    }

    pub fn get<T>(&self, sequence: usize) -> Option<T> {
        None
    }
}

