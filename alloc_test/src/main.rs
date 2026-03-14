
use core::alloc::{GlobalAlloc, Layout};
use std::alloc::System;

struct TestGlobalAlloc;
impl TestGlobalAlloc {
    const fn new() -> Self {
        TestGlobalAlloc {}
    }
}


static mut TEST_ALLOC_LAYOUT: Layout = Layout::new::<i32>();
static mut TEST_DEALLOC_LAYOUT: Layout = Layout::new::<i32>();


unsafe impl GlobalAlloc for TestGlobalAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unsafe {
            TEST_ALLOC_LAYOUT = _layout;
        }
        unsafe { System.alloc(_layout) }
    }
/*
Notes about where "daelloc" gets layout information:
 - https://stackoverflow.com/questions/61255554/memory-layout-alignment-and-deallocation
 - looks like it's all based on the type and various containers remembering size
 - https://os.phil-opp.com/heap-allocation/
*/
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unsafe {
            TEST_DEALLOC_LAYOUT = _layout;
        }
        unsafe { System.dealloc(_ptr, _layout) }
    }
}

#[global_allocator]
static TEST_GLOBAL_ALLOCATOR: TestGlobalAlloc = TestGlobalAlloc::new();


fn allocator_test_main() {

    // Testing alloc/dealloc a single 32-bit thingy
    println!("===== single 32-bit alloc =====");
    {
        let _x = Box::new(5);
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 4 align = 4
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 4 align = 4


    // Testing alloc/dealloc a single 8-bit thingy
    println!("===== single 8-bit alloc =====");
    {
        let _x = Box::new(5i8);
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 1 align = 1
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 1 align = 1

    // Testing alloc/dealloc a sngle struct
    println!("===== single random sized struct alloc =====");
    struct TEST(i32, u64); // 96 bits
    {
        let _x = Box::new(TEST(3, 10));
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 16 align = 8
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 16 align = 8


    // Testing alloc/dealloc a sngle larger struct
    println!("===== single random sized larger struct alloc =====");
    struct TEST2(i32, u64, [i32; 10]); // 52 bytes
    {
        let _x = Box::new(TEST2(3, 10, [0; 10]));
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 56 align = 8
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 56 align = 8


    // Testing alloc/dealloc an array of i32
    println!("===== array of 32-bit alloc =====");
    {
        let _x = Box::new([0; 10]);
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 40 align = 8
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 40 align = 8

    // Testing alloc/dealloc an array of i8
    println!("===== array of 8-bit alloc =====");
    {
        let _x = Box::new([0u8; 123]);
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 123 align = 1
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 123 align = 1

    // Testing alloc/dealloc a vector of ints
    println!("===== variable-sized alloc =====");
    {
        let mut _x = vec![1,2,3,4,5,6,7,8,9,10,11];
        _x.push(3);
        let temp_layout = unsafe { TEST_ALLOC_LAYOUT };

        println!("layout = {temp_layout:#?}"); // size = 88 align = 4 (after doubling capacity of 11)
    }

    let temp_layout = unsafe { TEST_DEALLOC_LAYOUT };
    println!("layout = {temp_layout:#?}"); // size = 88 align = 4

}


fn main() {
    allocator_test_main();
}