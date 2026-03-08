// #![no_std]

// #![feature(allocator_api)]  // api for non-global custom allocators

use core::alloc::{GlobalAlloc, Layout};
use std::alloc::System;

struct TestGlobalAlloc;
impl TestGlobalAlloc {
    const fn new() -> Self {
        TestGlobalAlloc {}
    }
}

unsafe impl GlobalAlloc for TestGlobalAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unsafe {
            TEST_GLOBAL_ALLOC_COUNT += 1;
        }
        unsafe { System.alloc(_layout) }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unsafe {
            TEST_GLOBAL_DEALLOC_COUNT += 1;
        }
        unsafe { System.dealloc(_ptr, _layout) }
    }
}

#[global_allocator]
static TEST_GLOBAL_ALLOCATOR: TestGlobalAlloc = TestGlobalAlloc::new();

static mut TEST_GLOBAL_ALLOC_COUNT: i32 = 0;
static mut TEST_GLOBAL_DEALLOC_COUNT: i32 = 0;

// Exported things need to be explicitly exported
mod bindings;
pub use bindings::{test, test_str};
