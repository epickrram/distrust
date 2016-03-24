extern crate memmap;

use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering;
use std::fs::File;
use std::ptr::read;
use std::ptr::write;
use std::mem::size_of;
use std::mem;
use std::mem::transmute;
use memmap::{Mmap, Protection};

const HEADER_BYTES: usize = 8;

pub struct MappedAtomicIsize<'a> {
    value: &'a AtomicIsize
}

impl<'a> MappedAtomicIsize<'a> {
    pub fn load(&self, ordering: Ordering) -> isize {
        self.value.load(ordering)
    }

    pub fn fetch_add(&self, value_to_add: isize, ordering: Ordering) -> isize {
        self.value.fetch_add(value_to_add, ordering)
    }
}

pub fn create_mapped_atomic(mem: &mut memmap::Mmap, offset: usize) -> MappedAtomicIsize {
    let data: &mut[u8] = unsafe { mem.as_mut_slice() };
    MappedAtomicIsize {
        value:  unsafe { mem::transmute(data.get_unchecked(offset)) }
    }

}

pub fn create_mapped_atomic_isize(file: &File, offset: usize) -> MappedAtomicIsize {
    let mut mem = match memmap::Mmap::open(file, memmap::Protection::ReadWrite) {
        Err(reason) => panic!("Failed to map file: {}", reason),
        Ok(memFile) => memFile
    };
    let data: &mut[u8] = unsafe { mem.as_mut_slice() };
    MappedAtomicIsize {
        value:  unsafe { mem::transmute(data.get_unchecked(offset)) }
    }
}


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

