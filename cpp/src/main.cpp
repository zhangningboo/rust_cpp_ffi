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
    for (int i = 0; i < array->len; i++) {
        SegmentBbox& box = array->bboxes[i];
        CppCvMat* mask_mat = box.mask_mat;
        std::vector<u_char> data(mask_mat->data, mask_mat->data + mask_mat->size);
        cv::Mat frame{};
        cv::imdecode(data, cv::IMREAD_COLOR, &frame);
        std::string file = "./" + std::to_string(i) + ".jpg";
        cv::imwrite(file, frame);
    }
    cpp_segment_free(array);

}