#ifndef RUST_CPP_FFI_CPP_NUMBER
#define RUST_CPP_FFI_CPP_NUMBER

#include <cstring>
#include <string>

int cpp_process_number(
    const usize rust_int,
    const float rust_float,
    const double rust_double
);

#ifdef __cplusplus
extern "C"
{
#endif

    int trans_number(
        const int rust_int,
        const float rust_float,
        const double rust_double
    );

#ifdef __cplusplus
}
#endif



#endif  // RUST_CPP_FFI_CPP_NUMBER