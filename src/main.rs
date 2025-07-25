mod ffi;

use crate::ffi::cpp_str::rust_trans_string;

fn main() {
    let rust_string = String::from("from rust string");
    let ffi_process = rust_trans_string(&rust_string);
    println!("结果：{}", ffi_process);
}
