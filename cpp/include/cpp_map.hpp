#ifndef RUST_CPP_FFI_CPP_MAP
#define RUST_CPP_FFI_CPP_MAP

#include <cstring>
#include <string>

struct Rect {
    int32_t x, y, width, height;
};

struct PairIntRect {
    int32_t id;
    Rect rect;
};

struct Group {
    int32_t key;
    PairIntRect* pairs;
    int32_t pair_count;
};

struct MapResult {
    Group* groups;
    int32_t group_count;
};

#ifdef __cplusplus
extern "C"
{
#endif

    MapResult trans_map_result(int32_t i);
    
    void free_map_result(MapResult result);

#ifdef __cplusplus
}
#endif



#endif  // RUST_CPP_FFI_CPP_MAP