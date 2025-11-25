//! Test fixture for unsafe function call detection

#![allow(dead_code)]

// Test 1: Core unsafe function
pub unsafe fn test_core_ptr_read(p: *const u8) -> u8 {
    core::ptr::read(p)
}

// Test 2: Alloc unsafe function (via Vec)
pub unsafe fn test_alloc_from_raw_parts<T>(p: *mut T, len: usize) -> Vec<T> {
    Vec::from_raw_parts(p, len, len)
}

// Test 3: Std unsafe function
pub unsafe fn test_std_ptr_write(p: *mut u8, value: u8) {
    std::ptr::write(p, value);
}

// Test 4: Multiple calls in one unsafe block
pub unsafe fn test_multiple_calls(p: *const u8, q: *mut u8) {
    let val = core::ptr::read(p);
    std::ptr::write(q, val);
}

// Test 5: Unsafe block with core call
pub fn test_unsafe_block() {
    let x = 42;
    let ptr = &x as *const i32;
    unsafe {
        core::ptr::read(ptr);
    }
}
