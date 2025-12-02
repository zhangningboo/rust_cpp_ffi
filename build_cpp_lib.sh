#!/bin/bash

export OpenCV_HOME=/data/workspace/aky_vlm_project/3rd/opencv_4.9.0_x86_ffmpeg
export FFMPEG_DIR=/data/workspace/aky_vlm_project/3rd/ffmpeg_n7.1.1_x86

cd cpp/build
rm -rf ./* && cmake .. && make && make install
cd ../..