#include "cpp_map.hpp"
#include <map>
#include <vector>
#include <utility>

#ifdef __cplusplus
extern "C"
{
#endif

    MapResult trans_map_result(int32_t rust_i)
    {
        try {
            // std::map<int, std::vector<std::pair<int, Rect>>> result{};
            std::map<int32_t, std::vector<std::pair<int32_t, Rect>>> result = {
                {1, // key = 1
                {
                    {10, {0, 0, 100, 50}}, // pair<int, Rect>
                    {11, {10, 10, 80, 40}}}},
                {2, // key = 2
                {
                    {20, {5, 5, 60, 30}},
                    {21, {15, 15, 70, 35}}}}};

            MapResult out;
            out.group_count = result.size();
            out.groups = new Group[out.group_count];

            int32_t i = 0;
            for (auto &[key, vec] : result)
            {
                out.groups[i].key = key;
                out.groups[i].pair_count = vec.size();
                out.groups[i].pairs = new PairIntRect[vec.size()];

                for (int j = 0; j < vec.size(); ++j)
                {
                    out.groups[i].pairs[j].id = vec[j].first;
                    out.groups[i].pairs[j].rect = {vec[j].second.x + rust_i, vec[j].second.y, vec[j].second.width, vec[j].second.height};
                }
                i++;
            }

            return out;
        } catch (...) {
            return { nullptr, 0};
        }
    }

    void free_map_result(MapResult result)
    {
        // 1. ✅ 核心判空：如果 groups 指针本身是空的，说明没有分配顶层内存，直接返回
        if (result.groups == nullptr)
        {
            return;
        }

        // 2. 只有当 group_count 大于 0 时才需要遍历释放内部内存
        if (result.group_count > 0) 
        {
            for (int32_t i = 0; i < result.group_count; ++i)
            {
                // C++ 标准规定 delete[] nullptr 是安全的（无操作），
                // 所以这里即使 pairs 是 nullptr 也是安全的，无需额外判空。
                delete[] result.groups[i].pairs;
            }
        }
        
        // 3. 释放顶层数组
        delete[] result.groups;
    }

#ifdef __cplusplus
}
#endif