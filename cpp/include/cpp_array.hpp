#ifndef RUST_CPP_FFI_CPP_ARRAY
#define RUST_CPP_FFI_CPP_ARRAY

#include <iostream>
#include <vector>
#include <cstdlib>
#include <cstring>
#include <cstdint>

typedef struct {
	uint32_t *data;
	uint32_t len;
} CArray;

void add(std::vector<uint32_t>& c_vec);

#ifdef __cplusplus
extern "C"
{
#endif

	bool cpp_process_array(CArray input_array, CArray* output_array);

	bool cpp_free_array(CArray* c_array);

#ifdef __cplusplus
}
#endif

#endif  // RUST_CPP_FFI_CPP_ARRAY