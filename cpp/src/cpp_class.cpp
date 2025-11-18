#include "cpp_class.hpp"

#include <iostream>

CppClass::CppClass(int32_t dev_id) {
    m_dev_id = dev_id;
    std::cout << "cpp instance, receive m_dev_id: " << m_dev_id << std::endl;
}

CppClass::~CppClass() {
    std::cout << "destroy cpp instance, m_dev_id: " << m_dev_id << std::endl;
}

int32_t CppClass::add(int32_t a, int32_t b) {
    std::cout << "cpp instance, add m_dev_id: " << m_dev_id << std::endl;
    return a + b;
}

#ifdef __cplusplus
extern "C"
{
#endif

    CppClass* create_class_instance(int32_t dev_id) {
        return new CppClass(dev_id);
    }

    int32_t call_instance_func(CppClass* instance, int32_t a, int32_t b) {
        return instance->add(a, b);
    }
    
    bool free_class_instance(CppClass* instance) {
        if (instance != nullptr) {
            delete instance;
            instance = nullptr;
        }
        return true;
    }

#ifdef __cplusplus
}
#endif