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
	    let new_block_ptr = Box::into_raw(new_block);
		tail = new_block_ptr;
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
		let refresh_block = Box::new(Block::new(ptr));
		let tail = unsafe { &mut *self.tail};
		let refresh_raw = Box::into_raw(refresh_block);
		tail.next = refresh_raw;
		self.tail = refresh_raw;
	}	

}


struct MultiPool {
    pool_set: Vec<PoolAlloc>
}
impl MultiPool {
    fn new(number: usize)-> Self {
        Self {
            pool_set:  vec![
                PoolAlloc::new(8, number),
                PoolAlloc::new(16, number),
                PoolAlloc::new(32, number),
                PoolAlloc::new(64, number),
                PoolAlloc::new(128, number),
            ]
        }
    }
    fn allocate(&mut self, size_: usize) -> *mut u8 {
        if self.pool_set.len() == 0 {
            panic!("Error")
        }
        for pool in &mut self.pool_set {
            if pool.block_size >= size_ {
                return pool.allocate();
            }
        }
		panic!("No suitable block size found for size {}", size_);
    }

    fn deallocate<T>(&mut self, ptr: *mut T) {
		let size = std::mem::size_of::<T>();
		for pool in &mut self.pool_set {
			if pool.block_size == size {
				pool.deallocate(ptr as *mut u8);
				return
			}
		}
    }

}



fn main() {
	let mut pool = MultiPool::new(10);
	let ptr = pool.allocate(8);
	pool.deallocate(ptr);
}

