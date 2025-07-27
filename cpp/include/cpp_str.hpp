#ifndef RUST_CPP_FFI_CPP_STR
#define RUST_CPP_FFI_CPP_STR

#include <cstring>
#include <string>

std::string cpp_process_string(const std::string &rust_string);

#ifdef __cplusplus
extern "C"
{
#endif

    const char* trans_string(const char* rust_string);
    
    void free_string(const char* str);

#ifdef __cplusplus
}
#endif



#endif  // RUST_CPP_FFI_CPP_STR