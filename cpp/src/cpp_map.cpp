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
        // std::map<int, std::vector<std::pair<int, Rect>>> result{};
        std::map<int, std::vector<std::pair<int, Rect>>> result = {
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

        int i = 0;
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
    }

    void free_map_result(MapResult result)
    {
        for (int i = 0; i < result.group_count; ++i)
        {
            delete[] result.groups[i].pairs;
        }
        delete[] result.groups;
    }

#ifdef __cplusplus
}
#endif