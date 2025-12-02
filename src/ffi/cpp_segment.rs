use std::ffi::c_void;
use std::slice;

#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    unsafe fn gen_mat() -> *mut FFiCppCvMat;

    unsafe fn free_mat(mat: *mut FFiCppCvMat) -> bool;
    
    unsafe fn cpp_segment(mat: *mut FFiCppCvMat) -> *mut FFiSegmentBboxArray; 
    
    unsafe fn cpp_segment_free(array: *mut FFiSegmentBboxArray);
}

#[repr(C)]
pub struct FFiCppCvMat {
    timestamp: i64,
    data: *mut u8,
    size: u64,
    width: i32,
    height: i32,
    channels: i32,
}

#[repr(C)]
pub struct FFiSegmentBbox {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub score: f32,
    pub class_id: i32,
    pub mask: *mut f32,
    pub mask_len: i32,
    pub mask_mat: *mut FFiCppCvMat,
}

#[repr(C)]
pub struct FFiSegmentBboxArray {
    pub bboxes: *mut FFiSegmentBbox,
    pub len: i32,
}

#[derive(Debug, Clone)]
pub struct CppCvMatSafe {
    timestamp: i64,
    data: Vec<u8>,
    width: i32,
    height: i32,
    channels: i32,
}

impl CppCvMatSafe {
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }
    
    pub fn get_height(&self) -> i32 {
        self.height
    }
    
    pub fn get_channels(&self) -> i32 {
        self.channels
    }
}

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

impl SegmentBboxSafe {

    pub fn get_x1(&self) -> i32 {
        self.x1
    }
    
    pub fn get_y1(&self) -> i32 {
        self.y1
    }

    pub fn get_x2(&self) -> i32 {
        self.x2
    }
    
    pub fn get_y2(&self) -> i32 {
        self.y2
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }
    
    pub fn get_class_id(&self) -> i32 {
        self.class_id
    }

    pub fn get_mask(&self) -> &[f32] {
        &self.mask
    }

    pub fn get_mask_mat(&self) -> &CppCvMatSafe {
        &self.mask_mat
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SegmentEngine(*mut c_void);

unsafe impl Send for SegmentEngine {}
unsafe impl Sync for SegmentEngine {}

impl SegmentEngine {
    pub fn new() -> CppCvMatSafe {
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
            let mut ffi_input = FFiCppCvMat {
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

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test --lib ffi::cpp_segment
    #[test]
    fn test_rust_segment() {
        // 模拟流程
        let mat = SegmentEngine::new();
        println!("Got Mat: size={}, w={}, h={}", mat.data.len(), mat.width, mat.height);
        
        let results = SegmentEngine::segment(&mat);
        println!("Got results: {}", results.len());
        
        for (i, bbox) in results.iter().enumerate() {
            println!("Bbox {}: score={}, mask_len={}", i, bbox.score, bbox.mask.len());
            if let Some(m) = &bbox.mask_mat {
                println!("   MaskMat size={}", m.data.len());
            }
        }
    }
}