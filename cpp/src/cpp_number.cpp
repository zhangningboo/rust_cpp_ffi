#include "cpp_number.hpp"
#include <iostream>

int cpp_process_number(
    const int rust_int,
    const float rust_float,
    const double rust_double
)
{
    std::cout << "cpp_process_number rust_int: " << rust_int << std::endl;
    std::cout << "cpp_process_number rust_float: " << rust_float << std::endl;
    std::cout << "cpp_process_number rust_double: " << rust_double << std::endl;
    return rust_int + 1;
}

#ifdef __cplusplus
extern "C" {
#endif

    int trans_number(
        const int rust_int,
        const float rust_float,
        const double rust_double
    ) 
    {
        return cpp_process_number(rust_int, rust_float, rust_double);
    }

#ifdef __cplusplus
}
#endif