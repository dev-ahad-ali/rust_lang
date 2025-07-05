use std::slice;

/*  SAFETY: Calling this from more than a single thread at a time is undefined
behavior, so you *must* guarantee you only call it from a single thread at
a time. */
static mut COUNTER: u32 = 0;

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // Dereferencing raw pointers
    let mut num = 5;

    let address = 0x012345usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    // unsafe function or method
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    // safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Using extern functions to call external code/from other languages.
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
        // safe fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // println!("Absolute value of -3 according to C: {}", abs(-3));

    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // Accessing or modifying a mutable static/global variable

    unsafe {
        add_to_count(3);
        // SAFETY: This is only called from a single thread in `main`.
        println!("Counter: {}", *(&raw const COUNTER));
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
