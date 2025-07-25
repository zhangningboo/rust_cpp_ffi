use std::{ffi::{c_char, CStr, CString}, str::FromStr};

#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    unsafe fn trans_string(rust_string: *const c_char) -> *const c_char;
    
    unsafe fn free_string(str: *const c_char);
}

pub fn rust_trans_string(rust_string: &String) -> String {
    let c_str = CString::new(rust_string.clone()).unwrap();
    unsafe {
        let c_res_str = trans_string(c_str.as_ptr());
        let str = CStr::from_ptr(c_res_str).to_str().unwrap();
        let res = String::from_str(str).unwrap();
        free_string(c_res_str);
        res
    }

}