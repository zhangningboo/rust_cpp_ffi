#ifndef RUST_CPP_FFI_CPP_CLASS
#define RUST_CPP_FFI_CPP_CLASS

#include <cstring>
#include <string>

class CppClass {
    public:
        CppClass(int32_t dev_id);

        ~CppClass();

        int32_t add(int32_t a, int32_t b);
    
    private:
        int32_t m_dev_id;
        
};

#ifdef __cplusplus
extern "C"
{
#endif

    CppClass* create_class_instance(int32_t dev_id);

    int32_t call_instance_func(CppClass* instance, int32_t a, int32_t b);
    
    bool free_class_instance(CppClass* instance);

#ifdef __cplusplus
}
#endif



#endif  // RUST_CPP_FFI_CPP_CLASS