#ifndef RUST_CPP_FFI_CPP_MAP
#define RUST_CPP_FFI_CPP_MAP

#include <cstring>
#include <string>

struct Rect {
    int x, y, width, height;
};

struct PairIntRect {
    int id;
    Rect rect;
};

struct Group {
    int key;
    PairIntRect* pairs;
    int pair_count;
};

struct MapResult {
    Group* groups;
    int group_count;
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