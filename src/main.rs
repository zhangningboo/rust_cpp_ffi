mod ffi;

use ffi::cpp_segment::SegmentEngine;
use rayon::prelude::*; // 引入 Rayon 的并行功能

fn main() {
    let mat = SegmentEngine::get_cpp_mat();
    println!("Got Mat: w={}, h={}", mat.width, mat.height);
    
    let results = SegmentEngine::segment(&mat);
    println!("Got results: {}", results.len());
    
    // 2. 并行保存阶段
    // 使用 par_iter() 替代 iter()
    results.par_iter().enumerate().for_each(|(i, bbox)| {
        println!("Thread {:?} processing Bbox {}", std::thread::current().id(), i);
        // 注意：这里假设 get_mask_mat() 返回的是新对象或者引用，且 C++ 端没有 Race Condition
        let mask_mat = bbox.mask_mat.clone();
        // get_data() 返回 slice，image::load_from_memory 是纯 CPU 计算
        if let Ok(image) = image::load_from_memory(&mask_mat.data) {
            // save_with_format 是 I/O + 编码操作，并行化收益巨大
            match image.save_with_format(format!("./target/{i}.jpg"), image::ImageFormat::Jpeg) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Save image error for {}: {:?}", i, err);
                }
            }
        } else {
            eprintln!("Failed to load image from memory for bbox {}", i);
        }
    });
    
    println!("All done!");
}