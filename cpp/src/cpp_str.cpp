#include "cpp_str.hpp"

std::string cpp_process_string(const std::string &rust_string)
{
	return rust_string + std::string(" cpp");
}

#ifdef __cplusplus
extern "C" {
#endif

    const char* trans_string(const char* rust_string) 
	{
        std::string rstr(rust_string);
		std::string res = cpp_process_string(rstr);
		char* c_str = new char[res.size() + 1];  // 堆内存
		std::strcpy(c_str, res.c_str());
		return c_str;
	}
	
	void free_string(const char* str) 
	{
		delete[] str;
	}

#ifdef __cplusplus
}
#endif