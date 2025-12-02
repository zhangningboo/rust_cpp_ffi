#include "cpp_segment.hpp"

#ifdef __cplusplus
extern "C"
{
#endif
    CppCvMat* gen_mat() {
        cv::Mat tmp = cv::Mat::zeros(1080, 1920, CV_8UC3);
        std::vector<uchar> encode_buffer;
        cv::imencode(".jpg", tmp, encode_buffer);
        // 内存分配：图像数据
        uint8_t* buffer_ptr = new uint8_t[encode_buffer.size()];
        std::memcpy(buffer_ptr, encode_buffer.data(), encode_buffer.size());
        CppCvMat* mask_mat = new CppCvMat();
        mask_mat->timestamp = 123456789;
        mask_mat->data = buffer_ptr;
        mask_mat->size = encode_buffer.size();
        mask_mat->width = tmp.cols;
        mask_mat->height = tmp.rows;
        mask_mat->channels = tmp.channels();
        return mask_mat;
    }

    bool free_mat(CppCvMat* mat) {
        if (mat) {
            delete[] mat->data;
            delete mat;
            return true;
        }
        return false;
    }

    SegmentBboxArray* cpp_segment(CppCvMat* input_mat) 
    {
        // 1. 类型修复: u_char -> uint8_t
        // 2. 向量构造: 确保数据指针有效
        if (!input_mat || !input_mat->data) {
            return nullptr;
        }

        std::vector<uint8_t> data(input_mat->data, input_mat->data + input_mat->size);
        
        // 3. OpenCV API 修复: imdecode 返回 Mat，而不是通过引用传参 (除非是特定重载，但通常用法如下)
        cv::Mat frame = cv::imdecode(data, cv::IMREAD_COLOR);
        if (frame.empty()) {
            return nullptr; // 解码失败处理
        }

        SegmentBboxArray* bbox_array = new SegmentBboxArray();
        int count = 1000; // 为了演示改为10，原代码1000太大容易内存爆炸，实际根据需求改回
        bbox_array->bboxes = new SegmentBbox[count];
        bbox_array->len = count;

        for (int i = 0; i < count; i++) {
            std::cout << "i: " << i << std::endl;
            // 直接操作数组元素，不要在这里 new SegmentBbox，否则会导致结构体本身的内存泄漏
            SegmentBbox& box = bbox_array->bboxes[i];
            // 绘图操作 (注意：这里是在原图 frame 上不断叠加文字，如果是为了测试每张图不同，应该 clone)
            // 如果每次需要纯净的图，应该在循环里 clone，而不是 putText 后再 clone
            cv::Mat tmp = frame.clone(); 
            std::string label = "Data " + std::to_string(i);
            cv::putText(tmp, label, cv::Point(50, 50), cv::FONT_HERSHEY_SIMPLEX, 0.6, cv::Scalar(0, 0, 255), 2);
            
            std::vector<uchar> encode_buffer;
            cv::imencode(".jpg", tmp, encode_buffer);

            // 内存分配：图像数据
            uint8_t* buffer_ptr = new uint8_t[encode_buffer.size()];
            std::memcpy(buffer_ptr, encode_buffer.data(), encode_buffer.size());

            // 内存分配：Mask Mat 结构体
            // 4. 语法修复: 使用 C++ 构造写法，避免冒号语法(C style designated initializer 在老版本C++不支持)
            CppCvMat* mask_mat = new CppCvMat();
            mask_mat->timestamp = i;
            mask_mat->data = buffer_ptr;
            mask_mat->size = encode_buffer.size();
            mask_mat->width = tmp.cols;
            mask_mat->height = tmp.rows;
            mask_mat->channels = tmp.channels();

            // 内存分配：Mask 浮点数组
            int32_t mask_len = (i == 0) ? 1 : i; // 防止 alloc(0)
            float* mask = new float[mask_len];
            // 初始化 mask 数据 (可选)
            for(int k=0; k<mask_len; k++) mask[k] = 0.5f;

            // 5. 赋值修复: 直接给结构体成员赋值，而不是创建一个指针再取地址赋给结构体
            box.x1 = i;
            box.y1 = i;
            box.x2 = i + 20;
            box.y2 = i + 20;
            box.score = 0.1f;
            box.class_id = i;
            box.mask = mask;
            box.mask_len = mask_len;
            box.mask_mat = mask_mat;
        }

        return bbox_array;
    }

    void cpp_segment_free(SegmentBboxArray* bbox_array) 
    {
        if (bbox_array) {
            if (bbox_array->bboxes) {
                // 6. 严重的内存泄漏修复: 必须深度释放内部指针
                for (int i = 0; i < bbox_array->len; i++) {
                    std::cout << "free i: " << i << std::endl;
                    SegmentBbox& box = bbox_array->bboxes[i];

                    // 释放 mask float 数组
                    if (box.mask) {
                        delete[] box.mask;
                        box.mask = nullptr;
                    }

                    // 释放 mask_mat 内部的 data 和结构体本身
                    if (box.mask_mat) {
                        if (box.mask_mat->data) {
                            delete[] box.mask_mat->data;
                            box.mask_mat->data = nullptr;
                        }
                        delete box.mask_mat;
                        box.mask_mat = nullptr;
                    }
                }
                // 释放结构体数组
                delete[] bbox_array->bboxes;
            }
            // 释放最外层容器
            delete bbox_array;
        }
    }
#ifdef __cplusplus
}
#endif