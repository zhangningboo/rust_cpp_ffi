mod ffi;

use crate::ffi::cpp_str::rust_trans_string;
use crate::ffi::cpp_number::rust_trans_number;

fn main() {
    let rust_string = String::from("from rust string");
    let ffi_process = rust_trans_string(&rust_string);
    println!("rust_trans_string 结果：{}", ffi_process);


    let ffi_process = rust_trans_number(1, 2.0 , 3.0);
    println!("rust_trans_number 结果：{}", ffi_process);
}
