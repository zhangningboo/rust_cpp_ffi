#include <iostream>
#include "cpp_str.hpp"

int main() {

	std::cout << "11111" << std::endl;

	std::string cstr("cpp str-");
	auto res = cpp_process_string(cstr);
	std::cout << res << std::endl;

	std::string ccstr("cpp str=");
	auto cres = trans_string(ccstr.c_str());
	std::cout << cres << std::endl;
	free_string(cres);

	return 0;
}