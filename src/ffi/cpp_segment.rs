use std::ffi::c_void;
use std::slice;

#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    unsafe fn gen_mat() -> *mut RawCppCvMat;

    unsafe fn free_mat(mat: *mut RawCppCvMat) -> bool;
    
    unsafe fn cpp_segment(mat: *mut RawCppCvMat) -> *mut RawSegmentBboxArray; 
    
    unsafe fn cpp_segment_free(array: *mut RawSegmentBboxArray);
}

#[repr(C)]
pub struct RawCppCvMat {
    timestamp: i64,
    data: *mut u8,
    size: u64,
    width: i32,
    height: i32,
    channels: i32,
}

#[repr(C)]
pub struct RawSegmentBbox {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    score: f32,
    class_id: i32,
    mask: *mut f32,
    mask_len: i32,
    mask_mat: *mut RawCppCvMat,
}

#[repr(C)]
pub struct RawSegmentBboxArray {
    bboxes: *mut RawSegmentBbox,
    len: i32,
}

#[derive(Debug, Clone)]
pub struct CppCvMatSafe {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
}

unsafe impl Send for CppCvMatSafe {}
unsafe impl Sync for CppCvMatSafe {}

#[derive(Debug, Clone)]
pub struct SegmentBboxSafe {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub score: f32,
    pub class_id: i32,
    pub mask: Vec<f32>,
    pub mask_mat: CppCvMatSafe,
}

unsafe impl Send for SegmentBboxSafe {}
unsafe impl Sync for SegmentBboxSafe {}

#[derive(Clone, Copy)]
pub struct SegmentEngine;

impl SegmentEngine {
    pub fn get_cpp_mat() -> CppCvMatSafe {
        unsafe {
            // 1. 获取 C++ 指针
            let ffi_mat_ptr = gen_mat();
            
            if ffi_mat_ptr.is_null() {
                panic!("gen_mat returned null");
            }
            
            let ffi_mat = &*ffi_mat_ptr; // 解引用

            // 2. 关键修复：使用 slice::from_raw_parts 结合 to_vec() 进行深拷贝
            // 不要使用 Vec::from_raw_parts，否则 Rust 会尝试释放 C++ 的内存
            let data_slice = slice::from_raw_parts(ffi_mat.data, ffi_mat.size as usize);
            let safe_mat = CppCvMatSafe {
                timestamp: ffi_mat.timestamp,
                data: data_slice.to_vec(), // Deep Copy
                width: ffi_mat.width,
                height: ffi_mat.height,
                channels: ffi_mat.channels,
            };

            // 3. 通知 C++ 释放它那边的内存
            if !free_mat(ffi_mat_ptr) {
                panic!("free_mat failed");
            }
            
            safe_mat
        }
    }
    
    pub fn segment(mat: &CppCvMatSafe) -> Vec<SegmentBboxSafe> {
        unsafe {
            // 1. 构建临时的 FFi 结构体，将 Rust 的 Vec 指针暴露给 C++
            // 这里不需要拷贝数据，只需要传递指针，因为 C++ 只是读取 (const)
            let mut ffi_input = RawCppCvMat {
                timestamp: mat.timestamp,
                data: mat.data.as_ptr() as *mut u8, // 获取 Vec 的指针
                size: mat.data.len() as u64,
                width: mat.width,
                height: mat.height,
                channels: mat.channels,
            };

            // 2. 调用 C++ 推理
            let ffi_array_ptr = cpp_segment(&mut ffi_input);
            
            if ffi_array_ptr.is_null() {
                return Vec::new();
            }

            let ffi_array = &*ffi_array_ptr;
            let mut bboxes = Vec::with_capacity(ffi_array.len as usize);

            // 3. 遍历结果并转换为 Rust 安全类型
            // 使用 slice::from_raw_parts 来遍历 C 数组
            let ffi_bboxes_slice = slice::from_raw_parts(ffi_array.bboxes, ffi_array.len as usize);

            for bbox in ffi_bboxes_slice {
                // 处理 mask float 数组
                let mask_vec = if !bbox.mask.is_null() && bbox.mask_len > 0 {
                    slice::from_raw_parts(bbox.mask, bbox.mask_len as usize).to_vec()
                } else {
                    Vec::new()
                };

                let m_ptr = &*bbox.mask_mat;
                let m_data = slice::from_raw_parts(m_ptr.data, m_ptr.size as usize).to_vec();

                // 处理 mask_mat 图像
                let mask_mat_safe = CppCvMatSafe {
                    timestamp: m_ptr.timestamp,
                    data: m_data,
                    width: m_ptr.width,
                    height: m_ptr.height,
                    channels: m_ptr.channels,
                };

                bboxes.push(SegmentBboxSafe {
                    x1: bbox.x1,
                    y1: bbox.y1,
                    x2: bbox.x2,
                    y2: bbox.y2,
                    score: bbox.score,
                    class_id: bbox.class_id,
                    mask: mask_vec,
                    mask_mat: mask_mat_safe,
                });
            }

            // 4. 释放 C++ 分配的内存
            // 因为我们上面都用了 to_vec() 拷贝，现在可以安全地让 C++ 销毁它的数据
            cpp_segment_free(ffi_array_ptr);
            
            bboxes
        }
    }

}