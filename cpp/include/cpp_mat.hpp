#ifndef RUST_CPP_FFI_CPP_MAT
#define RUST_CPP_FFI_CPP_MAT

#include <cstring>
#include <string>

typedef struct {
    uint8_t *data;
    uint32_t len;
    uint32_t width;
    uint32_t height;
    uint32_t channel;
} CMat;

bool process_cmat(CMat &c_mat);

#ifdef __cplusplus
extern "C"
{
#endif

    bool cpp_process_cmat(CMat input_c_mat, CMat* output_c_mat);

    bool cpp_free_cmat(CMat* c_mat);

#ifdef __cplusplus
}
#endif

#endif  // RUST_CPP_FFI_CPP_MAT