#include "cpp_segment.hpp"

#ifdef __cplusplus
extern "C"
{
#endif

	SegmentBboxArray* cpp_segment(const CppCvMat* input_mat) 
	{
		SegmentBboxArray* bbox_array = new SegmentBboxArray();
		bbox_array->bboxes = new SegmentBbox[100];
		bbox_array->len = 100;
		return bbox_array;
	}

	void cpp_segment_free(SegmentBboxArray* bbox_array) 
	{
		if (bbox_array) {
			delete[] bbox_array->bboxes;
			delete bbox_array;
		}
	}
#ifdef __cplusplus
}
#endif