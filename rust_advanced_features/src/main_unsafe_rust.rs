

fn main() {
    // Dereference a raw pointer
    // ================================================================
    // *const T
    // *mut T

    // let mut num = 5;

    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }



    // Call an unsafe function or method
    // ================================================================
    // unsafe fn myfn() { }

    // unsafe {
    //     myfn();
    // }

    let mut v = vec![1,2,3,4,5,6];

    // let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3);

    // assert_eq!(a, &mut [1,2,3]);
    // assert_eq!(b, &mut [4,5,6]);

    let (left, rigth) = split_at_mut(&mut v, 3);

    use std::slice;
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
        // (&mut values[..mid], &mut values[mid..])
    }


    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // unsafe {
    //     println!("absolute valud of -3: {}", abs(-3));
    // }

    // #[no_mangle]
    // pub extern "C" fn print_text() {
    //     println!("Just called a Rust function!");
    // }


    // Access or modify a mutable static variable (global variable)
    // ================================================================
    println!("greeting: {}", HELLO);

    add_to_count(3);
    unsafe {
        println!("Counter: {}", COUNTER);
    }



    // Implement an unsafe trait
    // ================================================================
    unsafe trait Foo {}

    unsafe impl Foo for i32 {}

}

static HELLO: &str = "Hello";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Access fields of unions
// ================================================================
union MyUnion {
    f1: u32,
    f2: u32,
}