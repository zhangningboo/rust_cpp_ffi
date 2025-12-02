#ifndef RUST_CPP_FFI_CPP_SEGMENT
#define RUST_CPP_FFI_CPP_SEGMENT

#include <cstring>
#include <string>
#include <cstdint>

struct CppCvMat
{	
	int64_t timestamp;
	void * buffer_ptr;
	uint8_t * data;
	int32_t size;
	int32_t width;
	int32_t height;
	int32_t channels;
};

struct SegmentBbox 
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
};

struct SegmentBboxArray
{
	SegmentBbox* bboxes;
	int32_t len;
};

#ifdef __cplusplus
extern "C"
{
#endif

	SegmentBboxArray* cpp_segment(const CppCvMat* input_mat);

	void cpp_segment_free(SegmentBboxArray* bbox_array);
#ifdef __cplusplus
}
#endif



#endif  // RUST_CPP_FFI_CPP_SEGMENT
