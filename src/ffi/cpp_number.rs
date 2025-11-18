use std::ffi::{c_int, c_float, c_double};

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

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test --lib ffi::cpp_number
    #[test]
    fn test_rust_trans_number() {
        let ffi_process = rust_trans_number(1000, 2.0 , 3.0);
        println!("rust_trans_number 结果：{}", ffi_process);
    }

}