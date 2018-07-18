use std::slice;

static HELLO_WORLD: &str = "Hello world";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

unsafe fn dangerous() {
    println!("hello!");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C");
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // let address = 0x012345usize;
    // let r = address as *mut i32;

    // let slice = unsafe {
    //     slice::from_raw_parts_mut(r, 10000)
    // };

    // println!("{:?}", slice);

    unsafe {
        dangerous();
    }

    unsafe {
        println!("absolute value of -32 in C is {}", abs(-32));
    }

    println!("name is {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER == {}", COUNTER);
    }
}
