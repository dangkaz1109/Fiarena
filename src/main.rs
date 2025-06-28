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
    pool_set: Vec<PoolAlloc>;
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
    fn allocate(&mut self, size) -> *mut u8 {
        if self.pool_set.len() == 0 {
            panic!("Error")
        }
        for i in self.pool_set {
            if i.size >= size {
                return i.allocate()
                break
            }
        }
    }

    fn deallocate(&self) {
        
    }

}



fn main() {
    let mut pool_allocator = PoolAlloc::new(1000, 32);
	let ptr = pool_allocator.allocate() as *mut f32;
	unsafe {
		ptr.write(100.0);
		let x = *ptr;
		println!("{:?}", x);
	}

}

