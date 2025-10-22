use std::ffi::c_int;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PairIntRect {
    pub id: i32,
    pub rect: Rect,
}

#[repr(C)]
pub struct Group {
    pub key: i32,
    pub pairs: *mut PairIntRect,
    pub pair_count: i32,
}

#[repr(C)]
pub struct MapResult {
    pub groups: *mut Group,
    pub group_count: i32,
} 

#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    unsafe fn trans_map_result(i: c_int) -> MapResult;

    unsafe fn free_map_result(result: MapResult);
}

#[derive(Debug)]
pub struct RustGroup {
    pub key: i32,
    pub pairs: Vec<PairIntRect>,
}

pub fn get_map_result(i: i32) -> Vec<RustGroup> {
    let mut result_vec: Vec<RustGroup> = Vec::new();
    unsafe {
        let result = trans_map_result(i);
        let groups = std::slice::from_raw_parts(result.groups, result.group_count as usize);
        for g in groups {
            let pairs = std::slice::from_raw_parts(g.pairs, g.pair_count as usize);
            // ✅ 为每个 group 构造一个 RustGroup
            let rust_group = RustGroup {
                key: g.key,
                pairs: pairs.to_vec(),
            };
            result_vec.push(rust_group);
        }
        free_map_result(result); // ✅ 循环结束后再释放 C++ 内存
    }
    result_vec
}