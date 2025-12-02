#include "cpp_mat.hpp"

bool process_cmat(CMat &c_mat) {

    return true;
}

#ifdef __cplusplus
extern "C"
{
#endif

    bool cpp_process_cmat(CMat input_c_mat, CMat* output_c_mat) {
        return false;
    }

    bool cpp_free_cmat(CMat* c_mat) {
        return false;
    }

#ifdef __cplusplus
}
#endif