use std::ffi::{c_short, c_int, c_float, c_double};

#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    // cpp_str
    unsafe fn trans_number(rust_int: c_int, rust_float: c_float, rust_double: c_double) -> c_int;
}

pub fn rust_trans_number(rust_int: i32, rust_float: f32, rust_double: f64) -> i32 {
    unsafe {
        trans_number(rust_int, rust_float, rust_double)
    }
}