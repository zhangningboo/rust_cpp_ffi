mod ffi;

use ffi::cpp_segment::SegmentEngine;

fn main() {
    let mat = SegmentEngine::new();
    println!("Got Mat: w={}, h={}", mat.get_width(), mat.get_height());
    
    let results = SegmentEngine::segment(&mat);
    println!("Got results: {}", results.len());
    
    // for (i, bbox) in results.iter().enumerate() {
    //     println!("Bbox {}: score={}, mask_len={}", i, bbox.score, bbox.mask.len());
    //     // if let m = &bbox.mask_mat {
    //     //     println!("   MaskMat size={}", m.data.len());
    //     // }
    // }
}