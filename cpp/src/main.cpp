#include <iostream>
#include "cpp_str.hpp"

#include "cpp_segment.hpp"

void test_segment();

int main() {
    test_segment();
    return 0;
}

void test_segment() {

    CppCvMat* mat = gen_mat();
    SegmentBboxArray* array = cpp_segment(mat);
    cpp_segment_free(array);

}