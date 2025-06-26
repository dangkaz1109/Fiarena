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
	tail: *mut Block
}
impl PoolAlloc {
    fn new(size: usize, number: usize) -> Self{
	let layout = Layout::from_size_align(size*number, 8).unwrap();
	let start_pool = unsafe{alloc(layout)};
	let mut head_block = Box::new(Block::new(start_pool));
	let mut current = &mut *head_block;
	let mut tail = null_mut();
	for i in 1..number {	
	    let ptr = unsafe {start_pool.add(i*size)};
	    let new_block = Box::new(Block::new(ptr));
		tail = Box::into_raw(new_block);
	    let new_block_ptr = Box::into_raw(new_block);
	    current.next = new_block_ptr;
	    current = unsafe {
		&mut *new_block_ptr
	    }

        }
	PoolAlloc {
	    block_size: size,
	    block_number: number,
	    head: Box::into_raw(head_block),
		tail: tail
    	}
	}

	fn allocate(&mut self) -> *mut u8 {
		let head_block_ptr = self.head;
		if head_block_ptr.is_null() {
			eprintln!("ERROR");
			return null_mut();
		}
		let block = unsafe {&mut *head_block_ptr};
		let block_pointer = block.pointer;
		self.head = block.next;
		block_pointer
	}

	fn deallocate(&mut self, ptr: *mut u8) {
		
	}	

}
fn main() {
    println!("uuhhhHello, world!");
}

