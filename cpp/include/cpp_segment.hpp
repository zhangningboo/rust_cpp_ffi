#ifndef RUST_CPP_FFI_CPP_SEGMENT
#define RUST_CPP_FFI_CPP_SEGMENT

#include <cstring>
#include <string>
#include <cstdint>
#include <vector>

#include "opencv2/opencv.hpp"

typedef struct
{	
	int64_t timestamp;
	uint8_t * data;
	uint64_t size;
	int32_t width;
	int32_t height;
	int32_t channels;
} CppCvMat;

typedef struct 
{
	int32_t x1;
	int32_t y1;
	int32_t x2;
	int32_t y2;
	float score;
	int32_t class_id;
	float * mask;
	int32_t mask_len;
	CppCvMat* mask_mat;
} SegmentBbox;

typedef struct
{
	SegmentBbox* bboxes;
	int32_t len;
} SegmentBboxArray;

#ifdef __cplusplus
extern "C"
{
#endif

    CppCvMat* gen_mat();

    bool free_mat(CppCvMat* mat);
	
    SegmentBboxArray* cpp_segment(CppCvMat* input_mat);

	void cpp_segment_free(SegmentBboxArray* bbox_array);
#ifdef __cplusplus
}
#endif



#endif  // RUST_CPP_FFI_CPP_SEGMENT
