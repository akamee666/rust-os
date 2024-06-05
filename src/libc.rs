/* The Rust compiler assumes that a certain set of built-in functions is available for all systems. Most of ""these functions are provided by the compiler_builtins crate that we just recompiled. However, there are some memory-related functions in that crate that are not enabled by default because they are normally provided by the C library on the system. These functions include memset, which sets all bytes in a memory block to a given value, memcpy, which copies one memory block to another, and memcmp, which compares two memory blocks. While we didn’t need any of these functions to compile our kernel right now, they will be required as soon as we add some more code to it (e.g. when copying structs around). */

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        *s.add(i) = c;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(d: *mut u8, s: *mut u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *d.add(i) = *s.add(i);
    }
    d
}
