mod ffi;

use crate::ffi::cpp_str::rust_trans_string;
use crate::ffi::cpp_number::rust_trans_number;
use crate::ffi::cpp_array::rust_call_cpp_process_array;


fn main() {
    let rust_string = String::from("from rust string");
    let ffi_process = rust_trans_string(&rust_string);
    println!("rust_trans_string 结果：{}", ffi_process);


    let ffi_process = rust_trans_number(1000, 2.0 , 3.0);
    println!("rust_trans_number 结果：{}", ffi_process);

    let rust_data: Vec<i32> = vec![1, 2, 3, 4, 5];
    rust_call_cpp_process_array(&rust_data);

    for i in 1..10 {
        println!("======map_result======== {:?}", i);
        let map_result = crate::ffi::cpp_map::get_map_result(i);
        for item in map_result {
            println!("{:?}", item);
        }
    }
}
