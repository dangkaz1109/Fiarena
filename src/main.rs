



use std::ptr::{null_mut};
use std::alloc::{alloc, dealloc, Layout};

struct Block {
    pointer: *mut u8,
    next: *mut Block
}
impl Block {
    fn new(ptr: *mut u8) -> Self {
        Self {
            pointer: ptr,
            next: null_mut()
        }
    }
}

struct PoolAlloc {
    block_size: usize,
    block_number: usize,
    head: *mut Block,
}
impl PoolAlloc {
    fn new(size: usize, number: usize) -> Self{
	let layout = Layout::from_size_align(size*number, 8).unwrap();
	let start_pool = unsafe{alloc(layout)};
	let mut head_block = Box::new(Block::new(start_pool));
	let mut current = &mut *head_block;
	for i in 1..number {	
	    let ptr = unsafe {start_pool.add(i*size)};
	    let new_block = Box::new(Block::new(ptr));
	    let new_block_ptr = Box::into_raw(new_block);
	    current.next = new_block_ptr;
	    current = unsafe {
		&mut *new_block_ptr
	    }

        }
	PoolAlloc {
	    block_size: size,
	    block_number: number,
	    head: Box::into_raw(head_block)
	
    }
}}
fn main() {
    println!("uuhhhHello, world!");
}

