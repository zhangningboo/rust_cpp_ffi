#include "cpp_array.hpp"

CArray add(std::vector<uint32_t>& c_vec) {
    for (auto &item : c_vec) {
        item++;
        std::cout << "item: " << item << std::endl;
    }
}


#ifdef __cplusplus
extern "C"
{
#endif

    bool cpp_process_array(CArray input_array, CArray* output_array) {
        try {
            std::vector<uint32_t> c_vec(input_array.data, input_array.data + input_array.len);
            add(c_vec);
            // 把结果返回给 Rust
            uint32_t* buf = (uint32_t*)std::malloc(c_vec.size());
            std::memcpy(buf, c_vec.data(), c_vec.size());

            output_array->data = buf;
            output_array->len = static_cast<uint32_t>(c_vec.size());
            return true;
        } catch (const std::exception& e) {
            std::cout << "cpp_process_array 执行异常: " << e.what() << std::endl;
            return false;
        }
    }

    bool cpp_free_array(CArray* c_array) {
        try {
            if (c_array->data) {
                free(c_array->data);
            }
            return true;
        } catch (const std::exception& e) {
            std::cout << "cpp_free_array 执行异常: " << e.what() << std::endl;
            return false;
        }
    }

#ifdef __cplusplus
}
#endif