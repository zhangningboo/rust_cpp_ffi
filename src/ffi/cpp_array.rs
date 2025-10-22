#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    // cpp_array
    unsafe fn cpp_process_array(input_array: CArray, output_array: *mut CArray) -> bool;

    unsafe fn cpp_free_array(output_array: *mut CArray) -> bool;
}

#[repr(C)]
pub struct CArray {
    pub data: *mut i32,
    pub len: u32,
}


impl CArray {
    pub fn from_vec(data: &[i32]) -> CArray {
        let mut hold_vec = data.to_vec();
        let carray = CArray {
            data: hold_vec.as_mut_ptr(),
            len: hold_vec.len() as u32,
        };
        // 防止 Rust 自动释放 Vec
        std::mem::forget(hold_vec);
        carray
    }

    pub fn to_vec(&self) -> Vec<i32> {
        if self.data.is_null() || self.len == 0 {
            return Vec::new();
        }
        unsafe {
            std::slice::from_raw_parts(self.data, self.len as usize).to_vec()
        }
    }
}


pub fn rust_call_cpp_process_array(rust_data: &[i32]) -> Option<Vec<i32>> {
    let rust_input_array = CArray::from_vec(rust_data);
    let mut cpp_output_array = CArray {
        data: std::ptr::null_mut(),
        len: 0,
    };
    unsafe {
        if cpp_process_array(rust_input_array, &mut cpp_output_array) {
            let result = cpp_output_array.to_vec();
            // 调用 C++ 释放内存
            cpp_free_array(&mut cpp_output_array);
            return Some(result);
        }
    }
    None
}